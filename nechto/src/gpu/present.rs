use ash::khr::{surface, swapchain};
use ash::vk::{self, Handle};

use crate::gpu::SelectedPhysicalDevice;

pub(super) struct Swapchain {
    device: ash::Device,
    swapchain_device: swapchain::Device,
    swapchain: vk::SwapchainKHR,
    surface: vk::SurfaceKHR,
    surface_instance: surface::Instance,
    surface_format: vk::SurfaceFormatKHR,
    physical_device: vk::PhysicalDevice,
    graphics_compute_queue_family_index: u32,

    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
}

impl Swapchain {
    #[allow(clippy::too_many_arguments)]
    pub(super) unsafe fn new(
        instance: &ash::Instance,
        selected_physical_device: &SelectedPhysicalDevice,
        device: &ash::Device,
        surface: vk::SurfaceKHR,
        surface_instance: surface::Instance,
        surface_format: vk::SurfaceFormatKHR,
        width: u32,
        height: u32,
    ) -> Self {
        unsafe {
            let swapchain_device = swapchain::Device::new(instance, device);

            let swapchain = vk::SwapchainKHR::null();

            let mut swapchain = Self {
                swapchain_device,
                swapchain,
                surface,
                surface_instance,
                physical_device: selected_physical_device.physical_device,
                surface_format,
                graphics_compute_queue_family_index: selected_physical_device
                    .graphics_compute_queue_family_index,
                images: Vec::new(),
                image_views: Vec::new(),
                device: device.clone(),
            };

            swapchain.resize(width, height);

            swapchain
        }
    }

    pub(super) unsafe fn destroy(&self) {
        unsafe {
            if !self.swapchain.is_null() {
                for image_view in &self.image_views {
                    self.device.destroy_image_view(*image_view, None);
                }

                self.swapchain_device
                    .destroy_swapchain(self.swapchain, None);
            }
        }
    }

    pub(super) unsafe fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            if width == 0 || height == 0 {
                return;
            }

            // Drop old image views
            for image_view in self.image_views.drain(..) {
                self.device.destroy_image_view(image_view, None);
            }

            // Recreate swapchain
            let caps = self
                .surface_instance
                .get_physical_device_surface_capabilities(self.physical_device, self.surface)
                .unwrap();

            let width = width
                .max(caps.min_image_extent.width)
                .min(caps.max_image_extent.width);
            let height = height
                .max(caps.min_image_extent.height)
                .min(caps.max_image_extent.height);

            let queue_family_indices = &[self.graphics_compute_queue_family_index];

            let create_info = vk::SwapchainCreateInfoKHR::default()
                .surface(self.surface)
                .min_image_count(2)
                .image_format(self.surface_format.format)
                .image_color_space(self.surface_format.color_space)
                .image_extent(vk::Extent2D { width, height })
                .image_array_layers(1)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .queue_family_indices(queue_family_indices)
                .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(vk::PresentModeKHR::FIFO)
                .clipped(true)
                .old_swapchain(self.swapchain);

            let old_swapchain = self.swapchain;

            self.swapchain = self
                .swapchain_device
                .create_swapchain(&create_info, None)
                .unwrap();

            // Drop old swapchain
            self.swapchain_device.destroy_swapchain(old_swapchain, None);

            // Acquire new images
            self.images = self
                .swapchain_device
                .get_swapchain_images(self.swapchain)
                .unwrap();

            for image in &self.images {
                let create_info = vk::ImageViewCreateInfo::default()
                    .image(*image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(self.surface_format.format)
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::IDENTITY,
                        g: vk::ComponentSwizzle::IDENTITY,
                        b: vk::ComponentSwizzle::IDENTITY,
                        a: vk::ComponentSwizzle::IDENTITY,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    });

                let image_view = self.device.create_image_view(&create_info, None).unwrap();

                self.image_views.push(image_view);
            }
        }
    }

    pub(super) fn image_count(&self) -> usize {
        self.image_views.len()
    }

    pub(super) fn image_view(&self, index: usize) -> vk::ImageView {
        self.image_views[index]
    }

    pub(super) fn image(&self, index: usize) -> vk::Image {
        self.images[index]
    }

    pub(super) fn acquire_next_frame(&mut self, semaphore: vk::Semaphore) -> usize {
        const ACQUIRE_TIMEOUT_NS: u64 = 5_000_000_000;

        unsafe {
            self.swapchain_device
                .acquire_next_image(
                    self.swapchain,
                    ACQUIRE_TIMEOUT_NS,
                    semaphore,
                    vk::Fence::null(),
                )
                .unwrap()
                .0 as usize
        }
    }

    pub(super) fn present(
        &mut self,
        index: u32,
        present_semaphore: vk::Semaphore,
        graphics_queue: vk::Queue,
    ) {
        let image_indices = &[index];
        let swapchains = &[self.swapchain];
        let wait_semaphores = &[present_semaphore];

        let present_info = vk::PresentInfoKHR::default()
            .image_indices(image_indices)
            .swapchains(swapchains)
            .wait_semaphores(wait_semaphores);

        unsafe {
            self.swapchain_device
                .queue_present(graphics_queue, &present_info)
                .unwrap();
        }
    }
}
