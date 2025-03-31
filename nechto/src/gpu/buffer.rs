use ash::vk;
use gpu_alloc::{Config, GpuAllocator, MemoryBlock, MemoryPropertyFlags, Request, UsageFlags};
use gpu_alloc_ash::AshMemoryDevice;

use crate::gpu::{BufferDescriptor, MemoryType};

pub struct BufferAllocator {
    device: ash::Device,
    allocator: GpuAllocator<vk::DeviceMemory>,
}

impl BufferAllocator {
    pub fn new(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: vk::PhysicalDevice,
    ) -> Self {
        let config = Config::i_am_prototyping();
        let props = unsafe {
            gpu_alloc_ash::device_properties(instance, vk::API_VERSION_1_3, physical_device)
                .unwrap()
        };

        let allocator = GpuAllocator::new(config, props);

        Self {
            device: device.clone(),
            allocator,
        }
    }

    pub fn allocate_buffer(&mut self, desc: BufferDescriptor) -> Buffer {
        let mem = AshMemoryDevice::wrap(&self.device);

        unsafe {
            let block = self
                .allocator
                .alloc(
                    mem,
                    Request {
                        size: desc.size,
                        align_mask: 0,
                        usage: UsageFlags::DEVICE_ADDRESS,
                        memory_types: match desc.memory_type {
                            MemoryType::HostVisible => MemoryPropertyFlags::HOST_VISIBLE,
                            MemoryType::DeviceLocal => MemoryPropertyFlags::DEVICE_LOCAL,
                        }
                        .bits() as u32,
                    },
                )
                .unwrap();

            let create_info = vk::BufferCreateInfo::default()
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .size(block.size())
                .usage(vk::BufferUsageFlags::VERTEX_BUFFER);

            let buffer = self.device.create_buffer(&create_info, None).unwrap();

            self.device
                .bind_buffer_memory(buffer, *block.memory(), block.offset())
                .unwrap();

            Buffer { buffer, block }
        }
    }

    pub fn deallocate_buffer(&mut self, buffer: Buffer) {
        let mem = AshMemoryDevice::wrap(&self.device);

        unsafe {
            self.device.destroy_buffer(buffer.buffer, None);
            self.allocator.dealloc(mem, buffer.block);
        }
    }
}

pub struct Buffer {
    buffer: vk::Buffer,
    block: MemoryBlock<vk::DeviceMemory>,
}
