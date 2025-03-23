use ash::vk;

pub struct CommandBufferAllocator {
    device: ash::Device,
    pool: vk::CommandPool,
}

impl CommandBufferAllocator {
    pub(super) unsafe fn new(device: &ash::Device, queue_family_index: u32) -> Self {
        unsafe {
            let create_info =
                vk::CommandPoolCreateInfo::default().queue_family_index(queue_family_index);

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

            CommandBuffer { buffer }
        }
    }

    pub(super) unsafe fn destroy_command_buffer(&self, buffer: CommandBuffer) {
        unsafe {
            self.device
                .free_command_buffers(self.pool, &[buffer.buffer]);
        }
    }
}

#[derive(Clone, Copy)]
pub struct CommandBuffer {
    buffer: vk::CommandBuffer,
}

impl CommandBuffer {
    pub(super) fn raw(&self) -> vk::CommandBuffer {
        self.buffer
    }
}
