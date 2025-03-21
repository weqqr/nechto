use ash::vk;

pub struct Pipeline {
    device: ash::Device,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
}

pub struct PipelineDescriptor {
    pub vertex_shader: Vec<u8>,
    pub fragment_shader: Vec<u8>,
}

impl Pipeline {
    pub(super) unsafe fn new(device: &ash::Device, desc: PipelineDescriptor) -> Self {
        unsafe {
            let pipeline_layout = create_pipeline_layout(device);
            let pipeline = create_graphics_pipeline(device, &desc, pipeline_layout);

            Self {
                device: device.clone(),
                pipeline_layout,
                pipeline,
            }
        }
    }

    pub(super) unsafe fn destroy(&self) {
        unsafe {
            self.device.destroy_pipeline(self.pipeline, None);

            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}

unsafe fn create_pipeline_layout(device: &ash::Device) -> vk::PipelineLayout {
    let create_info = vk::PipelineLayoutCreateInfo::default()
        .push_constant_ranges(&[])
        .set_layouts(&[]);

    unsafe { device.create_pipeline_layout(&create_info, None).unwrap() }
}

unsafe fn create_shader_module(device: &ash::Device, spirv_bytecode: &[u8]) -> vk::ShaderModule {
    let spirv_u32 = bytemuck::cast_slice(spirv_bytecode);

    let create_info = vk::ShaderModuleCreateInfo::default().code(spirv_u32);

    unsafe { device.create_shader_module(&create_info, None).unwrap() }
}

unsafe fn create_graphics_pipeline(
    device: &ash::Device,
    desc: &PipelineDescriptor,
    pipeline_layout: vk::PipelineLayout,
) -> vk::Pipeline {
    let vertex_shader_module = unsafe { create_shader_module(device, &desc.vertex_shader) };
    let fragment_shader_module = unsafe { create_shader_module(device, &desc.fragment_shader) };

    let vertex_shader_stage = vk::PipelineShaderStageCreateInfo::default()
        .module(vertex_shader_module)
        .stage(vk::ShaderStageFlags::VERTEX)
        .name(c"vs_main");

    let fragment_shader_stage = vk::PipelineShaderStageCreateInfo::default()
        .module(fragment_shader_module)
        .stage(vk::ShaderStageFlags::FRAGMENT)
        .name(c"fs_main");

    let shader_stages = &[fragment_shader_stage, vertex_shader_stage];

    let vertex_attribute_descriptions = &[vk::VertexInputAttributeDescription::default()
        .binding(0)
        .format(vk::Format::R32G32B32_SFLOAT)
        .location(0)
        .offset(0)];

    let vertex_binding_descriptions = &[vk::VertexInputBindingDescription::default()
        .binding(0)
        .input_rate(vk::VertexInputRate::VERTEX)
        .stride((3 * size_of::<f32>()) as u32)];

    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_attribute_descriptions(vertex_attribute_descriptions)
        .vertex_binding_descriptions(vertex_binding_descriptions);

    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::default()
        .primitive_restart_enable(false)
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

    let color_blend_attachments = &[vk::PipelineColorBlendAttachmentState::default()
        .color_write_mask(vk::ColorComponentFlags::RGBA)
        .blend_enable(false)];

    let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
        .attachments(color_blend_attachments)
        .logic_op(vk::LogicOp::COPY)
        .logic_op_enable(false);

    let tesselation_state = vk::PipelineTessellationStateCreateInfo::default();

    let viewport_state = vk::PipelineViewportStateCreateInfo::default()
        .scissor_count(1)
        .viewport_count(1);

    let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
        .cull_mode(vk::CullModeFlags::NONE)
        .polygon_mode(vk::PolygonMode::FILL);

    let multisample_state = vk::PipelineMultisampleStateCreateInfo::default()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::TYPE_1);

    let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(false)
        .depth_write_enable(false)
        .stencil_test_enable(false);

    let dynamic_state = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&[
        vk::DynamicState::VIEWPORT,
        vk::DynamicState::SCISSOR,
        vk::DynamicState::LINE_WIDTH,
    ]);

    let mut rendering_create_info = vk::PipelineRenderingCreateInfo::default()
        .color_attachment_formats(&[
            vk::Format::R8G8B8A8_SRGB, // FIXME
        ]);

    let create_info = vk::GraphicsPipelineCreateInfo::default()
        .stages(shader_stages)
        .vertex_input_state(&vertex_input_state)
        .input_assembly_state(&input_assembly_state)
        .tessellation_state(&tesselation_state)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterization_state)
        .multisample_state(&multisample_state)
        .depth_stencil_state(&depth_stencil_state)
        .color_blend_state(&color_blend_state)
        .dynamic_state(&dynamic_state)
        .layout(pipeline_layout)
        .push_next(&mut rendering_create_info);

    unsafe {
        device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None)
            .unwrap()[0]
    }
}
