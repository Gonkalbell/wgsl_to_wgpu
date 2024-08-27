pub mod src {
    pub const GROUP: u32 = 0u32;
    pub const BINDING: u32 = 0u32;
    pub const LAYOUT: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: BINDING,
        visibility: wgpu::ShaderStages::all(),
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
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
pub mod vertex_weights {
    pub const GROUP: u32 = 0u32;
    pub const BINDING: u32 = 1u32;
    pub const LAYOUT: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: BINDING,
        visibility: wgpu::ShaderStages::all(),
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
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
pub mod dst {
    pub const GROUP: u32 = 0u32;
    pub const BINDING: u32 = 2u32;
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
pub mod transforms {
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
