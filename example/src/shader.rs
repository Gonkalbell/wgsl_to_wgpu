// File automatically generated by build.rs.
// Changes made to this file will not be saved.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck :: Pod, bytemuck :: Zeroable)]
pub struct VertexInput {
    pub position: glam::Vec3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, encase :: ShaderType)]
pub struct Uniforms {
    pub color_rgb: glam::Vec3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, encase :: ShaderType)]
pub struct PushConstants {
    pub color_matrix: glam::Mat4,
}
pub mod color_texture {
    pub const GROUP: u32 = 0u32;
    pub const BINDING: u32 = 0u32;
    pub const LAYOUT: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: BINDING,
        visibility: wgpu::ShaderStages::all(),
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    };
    pub type Resource<'a> = &'a wgpu::TextureView;
    pub fn bind_group_entry(resource: Resource) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding: BINDING,
            resource: wgpu::BindingResource::TextureView(resource),
        }
    }
}
pub mod color_sampler {
    pub const GROUP: u32 = 0u32;
    pub const BINDING: u32 = 1u32;
    pub const LAYOUT: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: BINDING,
        visibility: wgpu::ShaderStages::all(),
        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        count: None,
    };
    pub type Resource<'a> = &'a wgpu::Sampler;
    pub fn bind_group_entry(resource: Resource) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding: BINDING,
            resource: wgpu::BindingResource::Sampler(resource),
        }
    }
}
pub mod uniforms {
    pub const GROUP: u32 = 1u32;
    pub const BINDING: u32 = 0u32;
    pub const LAYOUT: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: BINDING,
        visibility: wgpu::ShaderStages::all(),
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    };
    pub type Resource<'a> = wgpu::BufferBinding<'a>;
    pub fn bind_group_entry(resource: Resource) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding: BINDING,
            resource: wgpu::BindingResource::Buffer(resource),
        }
    }
}
pub mod storage_vars {
    pub const GROUP: u32 = 0u32;
    pub const BINDING: u32 = 0u32;
    pub const LAYOUT: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: BINDING,
        visibility: wgpu::ShaderStages::all(),
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: false },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    };
    pub type Resource<'a> = wgpu::BufferBinding<'a>;
    pub fn bind_group_entry(resource: Resource) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding: BINDING,
            resource: wgpu::BindingResource::Buffer(resource),
        }
    }
}
pub struct OverrideConstants {
    pub force_black: bool,
    pub scale: Option<f32>,
}
impl OverrideConstants {
    pub fn constants(&self) -> std::collections::HashMap<String, f64> {
        let mut entries = std::collections::HashMap::from([(
            "force_black".to_owned(),
            if self.force_black { 1.0 } else { 0.0 },
        )]);
        if let Some(value) = self.scale {
            entries.insert("scale".to_owned(), value as f64);
        }
        entries
    }
}
impl VertexInput {
    pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Float32x3,
        offset: std::mem::offset_of!(VertexInput, position) as u64,
        shader_location: 0,
    }];
    pub const fn vertex_buffer_layout(
        step_mode: wgpu::VertexStepMode,
    ) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexInput>() as u64,
            step_mode,
            attributes: &VertexInput::VERTEX_ATTRIBUTES,
        }
    }
}
pub const MAIN_WORKGROUP_SIZE: [u32; 3] = [1, 1, 1];
pub const ENTRY_VS_MAIN: &str = "vs_main";
pub const ENTRY_FS_MAIN: &str = "fs_main";
pub const ENTRY_MAIN: &str = "main";
#[derive(Debug)]
pub struct VertexEntry<const N: usize> {
    pub entry_point: &'static str,
    pub buffers: [wgpu::VertexBufferLayout<'static>; N],
    pub constants: std::collections::HashMap<String, f64>,
}
pub fn vertex_state<'a, const N: usize>(
    module: &'a wgpu::ShaderModule,
    entry: &'a VertexEntry<N>,
) -> wgpu::VertexState<'a> {
    wgpu::VertexState {
        module,
        entry_point: entry.entry_point,
        buffers: &entry.buffers,
        compilation_options: wgpu::PipelineCompilationOptions {
            constants: &entry.constants,
            ..Default::default()
        },
    }
}
pub fn vs_main_entry(
    vertex_input: wgpu::VertexStepMode,
    overrides: &OverrideConstants,
) -> VertexEntry<1> {
    VertexEntry {
        entry_point: ENTRY_VS_MAIN,
        buffers: [VertexInput::vertex_buffer_layout(vertex_input)],
        constants: overrides.constants(),
    }
}
#[derive(Debug)]
pub struct FragmentEntry<const N: usize> {
    pub entry_point: &'static str,
    pub targets: [Option<wgpu::ColorTargetState>; N],
    pub constants: std::collections::HashMap<String, f64>,
}
pub fn fragment_state<'a, const N: usize>(
    module: &'a wgpu::ShaderModule,
    entry: &'a FragmentEntry<N>,
) -> wgpu::FragmentState<'a> {
    wgpu::FragmentState {
        module,
        entry_point: entry.entry_point,
        targets: &entry.targets,
        compilation_options: wgpu::PipelineCompilationOptions {
            constants: &entry.constants,
            ..Default::default()
        },
    }
}
pub fn fs_main_entry(
    targets: [Option<wgpu::ColorTargetState>; 1],
    overrides: &OverrideConstants,
) -> FragmentEntry<1> {
    FragmentEntry {
        entry_point: ENTRY_FS_MAIN,
        targets,
        constants: overrides.constants(),
    }
}
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    let source = std::borrow::Cow::Borrowed(include_str!("shader.wgsl"));
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(source),
    })
}
