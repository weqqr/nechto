use ash::vk;

pub struct CommandBufferAllocator {
    device: ash::Device,
    pool: vk::CommandPool,
}

impl CommandBufferAllocator {
    pub(super) unsafe fn new(device: &ash::Device, queue_family_index: u32) -> Self {
        unsafe {
            let create_info = vk::CommandPoolCreateInfo::default()
                .queue_family_index(queue_family_index)
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

            let pool = device.create_command_pool(&create_info, None).unwrap();

            Self {
                device: device.clone(),
                pool,
            }
        }
    }

    pub(super) unsafe fn destroy(&self) {
        unsafe {
            self.device.destroy_command_pool(self.pool, None);
        }
    }

    pub(super) unsafe fn allocate(&self) -> CommandBuffer {
        unsafe {
            let allocate_info = vk::CommandBufferAllocateInfo::default()
                .command_buffer_count(1)
                .command_pool(self.pool)
                .level(vk::CommandBufferLevel::PRIMARY);

            let buffer = self
                .device
                .allocate_command_buffers(&allocate_info)
                .unwrap()[0];

            CommandBuffer {
                device: self.device.clone(),
                buffer,
            }
        }
    }

    pub(super) unsafe fn destroy_command_buffer(&self, buffer: CommandBuffer) {
        unsafe {
            self.device
                .free_command_buffers(self.pool, &[buffer.buffer]);
        }
    }
}

#[derive(Clone)]
pub struct CommandBuffer {
    device: ash::Device,
    buffer: vk::CommandBuffer,
}

impl CommandBuffer {
    pub(super) fn raw(&self) -> vk::CommandBuffer {
        self.buffer
    }

    pub(super) fn begin(&self) {
        let begin_info = vk::CommandBufferBeginInfo::default();

        unsafe {
            self.device
                .begin_command_buffer(self.buffer, &begin_info)
                .unwrap();
        }
    }

    pub(super) fn reset(&self) {
        unsafe {
            self.device
                .reset_command_buffer(self.buffer, vk::CommandBufferResetFlags::RELEASE_RESOURCES)
                .unwrap();
        }
    }

    pub(super) fn end(&self) {
        unsafe {
            self.device.end_command_buffer(self.buffer).unwrap();
        }
    }

    pub(super) fn image_barrier(
        &self,
        image: vk::Image,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
    ) {
        let image_memory_barrier = vk::ImageMemoryBarrier2::default()
            .old_layout(old_layout)
            .new_layout(new_layout)
            .image(image)
            // FIXME: use subresource range associated with the image
            .subresource_range(
                vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_array_layer(0)
                    .layer_count(1)
                    .base_mip_level(0)
                    .level_count(1),
            );

        let image_memory_barriers = &[image_memory_barrier];

        let dependency_info =
            vk::DependencyInfo::default().image_memory_barriers(image_memory_barriers);

        unsafe {
            self.device
                .cmd_pipeline_barrier2(self.buffer, &dependency_info);
        }
    }
}
