mod command;
mod pipeline;
mod present;

pub use self::pipeline::{Pipeline, PipelineDescriptor};

use ash::ext::debug_utils;
use ash::khr::{surface, swapchain, timeline_semaphore, win32_surface};
use ash::vk;
use tracing::{debug, error, info, warn};
use winit::raw_window_handle::WindowHandle;

use crate::gpu::command::{CommandBuffer, CommandBufferAllocator};
use crate::gpu::present::Swapchain;

#[derive(Default)]
pub struct ContextOptions {
    pub enable_debug: bool,
}

#[allow(dead_code)]
pub struct Context {
    entry: ash::Entry,

    instance: ash::Instance,
    debug_utils_instance: Option<debug_utils::Instance>,
    debug_utils_messenger: Option<vk::DebugUtilsMessengerEXT>,
    surface_instance: surface::Instance,
    surface: vk::SurfaceKHR,
    selected_physical_device: SelectedPhysicalDevice,
    device: ash::Device,
    graphics_compute_queue: vk::Queue,
    command_buffer_allocator: CommandBufferAllocator,
    swapchain: Swapchain,

    current_frame_index: usize,

    timeline_device: timeline_semaphore::Device,
    timeline_semaphore: vk::Semaphore,
    present_semaphore: vk::Semaphore,
    next_acquire_semaphore: vk::Semaphore,

    progress: u64,

    frame_command_buffers: Vec<CommandBuffer>,
    frame_sync: Vec<FrameSync>,
}

struct FrameSync {
    acquire_semaphore: vk::Semaphore,
    prev_progress: u64,
}

pub struct Frame {
    image: vk::Image,
    image_view: vk::ImageView,
    command_buffer: CommandBuffer,
    acquire_semaphore: vk::Semaphore,
    index: u32,
}

// IMPORTANT: I couldn't figure out how to marry Vulkan with RAII, so all Vulkan
// objects are managed manually. Only `Context` has a Drop impl.
impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();

            for sync in self.frame_sync.drain(..) {
                self.device.destroy_semaphore(sync.acquire_semaphore, None);
            }

            for command_buffer in self.frame_command_buffers.drain(..) {
                self.command_buffer_allocator
                    .destroy_command_buffer(command_buffer);
            }

            self.device
                .destroy_semaphore(self.next_acquire_semaphore, None);
            self.device.destroy_semaphore(self.present_semaphore, None);
            self.device.destroy_semaphore(self.timeline_semaphore, None);

            self.swapchain.destroy();
            self.command_buffer_allocator.destroy();
            self.device.destroy_device(None);
            self.surface_instance.destroy_surface(self.surface, None);

            if let Some(instance) = &self.debug_utils_instance {
                instance.destroy_debug_utils_messenger(self.debug_utils_messenger.unwrap(), None);
            }

            self.instance.destroy_instance(None);
        }
    }
}

impl Context {
    pub fn new(
        window_handle: WindowHandle,
        width: u32,
        height: u32,
        context_options: ContextOptions,
    ) -> Self {
        unsafe {
            let entry = ash::Entry::load().unwrap();
            let instance = create_instance(&entry, context_options.enable_debug);

            let (debug_utils_instance, debug_utils_messenger) = if context_options.enable_debug {
                let (debug_utils_instance, debug_utils_messenger) =
                    create_debug_utils(&entry, &instance);

                (Some(debug_utils_instance), Some(debug_utils_messenger))
            } else {
                (None, None)
            };

            let surface_instance = surface::Instance::new(&entry, &instance);
            let surface = create_surface(&entry, &instance, window_handle);

            let selected_physical_device =
                select_physical_device(&instance, &surface_instance, surface);

            let device = create_device(&instance, &selected_physical_device);
            let graphics_compute_queue = device.get_device_queue(
                selected_physical_device.graphics_compute_queue_family_index,
                0,
            );

            let command_buffer_allocator = CommandBufferAllocator::new(
                &device,
                selected_physical_device.graphics_compute_queue_family_index,
            );

            let swapchain = Swapchain::new(
                &instance,
                &selected_physical_device,
                &device,
                surface,
                surface_instance.clone(),
                selected_physical_device.formats[0],
                width,
                height,
            );

            let timeline_device = timeline_semaphore::Device::new(&instance, &device);
            let timeline_semaphore = create_timeline_semaphore(&device);
            let present_semaphore = create_semaphore(&device);
            let next_acquire_semaphore = create_semaphore(&device);

            let mut frame_command_buffers = Vec::new();
            let mut frame_sync = Vec::new();

            for _ in 0..swapchain.image_count() {
                let command_buffer = command_buffer_allocator.allocate();
                frame_command_buffers.push(command_buffer);

                frame_sync.push(FrameSync {
                    acquire_semaphore: create_semaphore(&device),
                    prev_progress: 0,
                });
            }

            Self {
                entry,
                instance,
                debug_utils_instance,
                debug_utils_messenger,
                surface_instance,
                surface,
                selected_physical_device,
                device,
                graphics_compute_queue,
                command_buffer_allocator,
                swapchain,
                current_frame_index: 0, // will be filled by the swapchain once rendering starts
                timeline_device,
                timeline_semaphore,
                present_semaphore,
                progress: 0,
                next_acquire_semaphore,
                frame_command_buffers,
                frame_sync,
            }
        }
    }

    pub fn resize_swapchain(&mut self, width: u32, height: u32) {
        unsafe {
            self.device
                .queue_wait_idle(self.graphics_compute_queue)
                .unwrap();
            self.swapchain.resize(width, height);
        }
    }

    pub fn create_pipeline(&mut self, desc: PipelineDescriptor) -> Pipeline {
        unsafe { Pipeline::new(&self.device, desc) }
    }

    pub fn destroy_pipeline(&mut self, pipeline: &mut Pipeline) {
        unsafe { pipeline.destroy() }
    }

    pub fn begin_frame(&mut self) -> Frame {
        let index = self
            .swapchain
            .acquire_next_frame(self.next_acquire_semaphore);

        let image = self.swapchain.image(index);
        let image_view = self.swapchain.image_view(index);
        let command_buffer = self.frame_command_buffers[index].clone();

        std::mem::swap(
            &mut self.frame_sync[0].acquire_semaphore,
            &mut self.next_acquire_semaphore,
        );

        let wait_semaphores = &[self.timeline_semaphore];
        let wait_values = &[self.progress];

        let wait_info = vk::SemaphoreWaitInfo::default()
            .semaphores(wait_semaphores)
            .values(wait_values);

        unsafe {
            self.device
                .wait_semaphores(&wait_info, 5_000_000_000)
                .unwrap();
        }

        command_buffer.reset();
        command_buffer.begin();

        Frame {
            image,
            image_view,
            command_buffer,
            acquire_semaphore: self.frame_sync[0].acquire_semaphore,
            index: index as u32,
        }
    }

    pub fn end_frame(&mut self, frame: Frame) {
        frame.command_buffer.image_barrier(
            frame.image,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::PRESENT_SRC_KHR,
        );

        frame.command_buffer.end();

        let command_buffers = &[frame.command_buffer.raw()];

        self.progress += 1;

        let wait_semaphores = &[frame.acquire_semaphore];
        let signal_semaphores = &[self.timeline_semaphore, self.present_semaphore];
        let timeline_semaphore_wait_values = &[0];
        let timeline_semaphore_signal_values = &[self.progress, 0];
        let wait_stages = &[vk::PipelineStageFlags::ALL_COMMANDS];

        let mut timeline_submit_info = vk::TimelineSemaphoreSubmitInfo::default()
            .signal_semaphore_values(timeline_semaphore_signal_values)
            .wait_semaphore_values(timeline_semaphore_wait_values);

        let queue_submit_info = vk::SubmitInfo::default()
            .command_buffers(command_buffers)
            .signal_semaphores(signal_semaphores)
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_stages)
            .push_next(&mut timeline_submit_info);

        unsafe {
            self.device
                .queue_submit(
                    self.graphics_compute_queue,
                    &[queue_submit_info],
                    vk::Fence::null(),
                )
                .unwrap();

            self.swapchain.present(
                frame.index,
                self.present_semaphore,
                self.graphics_compute_queue,
            );
        }
    }
}

unsafe fn create_instance(entry: &ash::Entry, enable_debug: bool) -> ash::Instance {
    unsafe {
        let application_info = vk::ApplicationInfo::default()
            .api_version(vk::API_VERSION_1_3)
            .application_name(c"videoland")
            .application_version(1)
            .engine_name(c"videoland")
            .engine_version(1);

        let mut enabled_extension_names =
            vec![win32_surface::NAME.as_ptr(), surface::NAME.as_ptr()];

        let mut enabled_layer_names = vec![];

        if enable_debug {
            enabled_extension_names.push(debug_utils::NAME.as_ptr());
            enabled_layer_names.push(c"VK_LAYER_KHRONOS_validation".as_ptr());
        }

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_extension_names(&enabled_extension_names)
            .enabled_layer_names(&enabled_layer_names);

        entry.create_instance(&create_info, None).unwrap()
    }
}

unsafe fn create_debug_utils(
    entry: &ash::Entry,
    instance: &ash::Instance,
) -> (debug_utils::Instance, vk::DebugUtilsMessengerEXT) {
    unsafe {
        use vk::DebugUtilsMessageSeverityFlagsEXT as Severity;
        use vk::DebugUtilsMessageTypeFlagsEXT as Type;

        let debug_utils_instance = debug_utils::Instance::new(entry, instance);

        let severity = Severity::ERROR | Severity::VERBOSE | Severity::INFO | Severity::WARNING;
        let message_type =
            Type::VALIDATION | Type::PERFORMANCE | Type::GENERAL | Type::DEVICE_ADDRESS_BINDING;

        let create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(severity)
            .message_type(message_type)
            .pfn_user_callback(Some(debug_utils_callback));

        let debug_utils_messenger = debug_utils_instance
            .create_debug_utils_messenger(&create_info, None)
            .unwrap();

        (debug_utils_instance, debug_utils_messenger)
    }
}

unsafe extern "system" fn debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    _message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    unsafe {
        use vk::DebugUtilsMessageSeverityFlagsEXT as Severity;

        if p_callback_data.is_null() {
            return vk::FALSE;
        }

        let message = (*p_callback_data)
            .message_as_c_str()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        if message_severity.contains(Severity::ERROR) {
            error!("{}", message);
        } else if message_severity.contains(Severity::WARNING) {
            warn!("{}", message);
        } else if message_severity.contains(Severity::INFO) {
            info!("{}", message);
        } else if message_severity.contains(Severity::VERBOSE) {
            debug!("{}", message);
        } else {
            info!("{}", message);
        }

        vk::FALSE
    }
}

#[cfg(windows)]
unsafe fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window_handle: WindowHandle,
) -> vk::SurfaceKHR {
    unsafe {
        use winit::raw_window_handle::RawWindowHandle;

        let win32_surface = win32_surface::Instance::new(entry, instance);

        let RawWindowHandle::Win32(handle) = window_handle.as_raw() else {
            panic!("unsupported platform");
        };

        let create_info = vk::Win32SurfaceCreateInfoKHR::default()
            .hinstance(handle.hinstance.unwrap().get())
            .hwnd(handle.hwnd.get());

        win32_surface
            .create_win32_surface(&create_info, None)
            .unwrap()
    }
}

struct SelectedPhysicalDevice {
    physical_device: vk::PhysicalDevice,
    graphics_compute_queue_family_index: u32,
    capabilities: vk::SurfaceCapabilitiesKHR,
    formats: Vec<vk::SurfaceFormatKHR>,
}

unsafe fn select_physical_device(
    instance: &ash::Instance,
    surface_instance: &surface::Instance,
    surface: vk::SurfaceKHR,
) -> SelectedPhysicalDevice {
    unsafe {
        for physical_device in instance.enumerate_physical_devices().unwrap() {
            let properties = instance.get_physical_device_properties(physical_device);

            info!(
                name = properties
                    .device_name_as_c_str()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                "trying device"
            );

            let mut graphics_compute_queue_family_index = vk::QUEUE_FAMILY_IGNORED;

            let queue_families =
                instance.get_physical_device_queue_family_properties(physical_device);

            for (i, queue_family) in queue_families.iter().enumerate() {
                // query graphics+compute
                let graphics_support = queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                let compute_support = queue_family.queue_flags.contains(vk::QueueFlags::COMPUTE);

                // query surface support
                let surface_support = surface_instance
                    .get_physical_device_surface_support(physical_device, i as u32, surface)
                    .unwrap();

                if surface_support && graphics_support && compute_support {
                    graphics_compute_queue_family_index = i as u32;
                }
            }

            if graphics_compute_queue_family_index == vk::QUEUE_FAMILY_IGNORED {
                println!("device has no graphics+compute queue family");
                continue;
            }

            let capabilities = surface_instance
                .get_physical_device_surface_capabilities(physical_device, surface)
                .unwrap();

            let formats = surface_instance
                .get_physical_device_surface_formats(physical_device, surface)
                .unwrap();

            return SelectedPhysicalDevice {
                physical_device,
                graphics_compute_queue_family_index,
                capabilities,
                formats,
            };
        }

        panic!("no devices?");
    }
}

unsafe fn create_device(
    instance: &ash::Instance,
    selected_physical_device: &SelectedPhysicalDevice,
) -> ash::Device {
    unsafe {
        let queue_priority = &[1.0];

        let graphics_compute_queue_create_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(selected_physical_device.graphics_compute_queue_family_index)
            .queue_priorities(queue_priority);

        let queue_create_infos = &[graphics_compute_queue_create_info];

        let enabled_extension_names = &[swapchain::NAME.as_ptr()];

        let mut vulkan_1_2_features =
            vk::PhysicalDeviceVulkan12Features::default().timeline_semaphore(true);

        let mut vulkan_1_3_features = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);

        let create_info = vk::DeviceCreateInfo::default()
            .enabled_extension_names(enabled_extension_names)
            .queue_create_infos(queue_create_infos)
            .push_next(&mut vulkan_1_3_features)
            .push_next(&mut vulkan_1_2_features);

        instance
            .create_device(selected_physical_device.physical_device, &create_info, None)
            .unwrap()
    }
}

unsafe fn create_timeline_semaphore(device: &ash::Device) -> vk::Semaphore {
    let mut timeline_create_info = vk::SemaphoreTypeCreateInfo::default()
        .initial_value(0)
        .semaphore_type(vk::SemaphoreType::TIMELINE);

    let create_info = vk::SemaphoreCreateInfo::default().push_next(&mut timeline_create_info);

    unsafe { device.create_semaphore(&create_info, None).unwrap() }
}

unsafe fn create_semaphore(device: &ash::Device) -> vk::Semaphore {
    let create_info = vk::SemaphoreCreateInfo::default();

    unsafe { device.create_semaphore(&create_info, None).unwrap() }
}
