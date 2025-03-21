use ash::vk;

pub struct Pipeline {
    device: ash::Device,
    pipeline_layout: vk::PipelineLayout,
}

impl Pipeline {
    pub(super) unsafe fn new(device: &ash::Device) -> Self {
        let create_info = vk::PipelineLayoutCreateInfo::default()
            .push_constant_ranges(&[])
            .set_layouts(&[]);

        let pipeline_layout = unsafe { device.create_pipeline_layout(&create_info, None).unwrap() };

        Self {
            device: device.clone(),
            pipeline_layout,
        }
    }

    pub(super) unsafe fn destroy(&self) {
        unsafe {
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}
