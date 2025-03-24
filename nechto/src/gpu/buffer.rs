use ash::vk;
use gpu_alloc::{Config, GpuAllocator, MemoryPropertyFlags, Request, UsageFlags};
use gpu_alloc_ash::AshMemoryDevice;

use crate::gpu::{BufferAllocationDescriptor, MemoryType};

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

    pub fn allocate_buffer(&mut self, desc: BufferAllocationDescriptor) -> vk::Buffer {
        let aa = AshMemoryDevice::wrap(&self.device);

        unsafe {
            self.allocator
                .alloc(
                    aa,
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
                .unwrap()
        };

        unimplemented!()
    }
}
