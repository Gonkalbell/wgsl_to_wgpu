use naga::ResourceBinding;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::wgsl::buffer_binding_type;

pub fn globals(module: &naga::Module) -> TokenStream {
    // Create matching Rust constants for WGSl resources.
    module
        .global_variables
        .iter()
        .filter_map(|(_, global)| -> Option<TokenStream> {
            global.name.as_ref().zip(global.binding.as_ref()).map(|(name, ResourceBinding { group, binding })| {
                let name = format_ident!("{name}");
                let ty = &module.types[global.ty];
                let layout_type = global_layout_entry_type(ty, global.space);

                let (resource_type, binding_resource) = match ty.inner {
                    naga::TypeInner::Image { .. } => (quote!(&'a wgpu::TextureView), quote!(wgpu::BindingResource::TextureView(resource))),
                    naga::TypeInner::Sampler { .. } => (quote!(&'a wgpu::Sampler), quote!(wgpu::BindingResource::Sampler(resource))),
                    _ => (quote!(wgpu::BufferBinding<'a>), quote!(wgpu::BindingResource::Buffer(resource))),
                };

                quote! {
                    pub mod #name {
                        pub const GROUP: u32 = #group;
                        pub const BINDING: u32 = #binding;
                        pub const LAYOUT: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
                            binding: BINDING,
                            visibility: wgpu::ShaderStages::all(),
                            ty: #layout_type,
                            count: None,
                        };
                        pub type Resource<'a> = #resource_type;
                        pub fn bind_group_entry(resource: Resource) -> wgpu::BindGroupEntry<'_> {
                            wgpu::BindGroupEntry {
                                binding: BINDING,
                                resource: #binding_resource,
                            }
                        }
                    }
                }
            })
        })
        .collect()
}

fn global_layout_entry_type(ty: &naga::Type, address_space: naga::AddressSpace) -> TokenStream {
    // TODO: Assume storage is only used for compute?
    // TODO: Support just vertex or fragment?
    // TODO: Visible from all stages?
    // TODO: Support more types.
    match ty.inner {
        naga::TypeInner::Image { dim, class, .. } => {
            let view_dim = match dim {
                naga::ImageDimension::D1 => quote!(wgpu::TextureViewDimension::D1),
                naga::ImageDimension::D2 => quote!(wgpu::TextureViewDimension::D2),
                naga::ImageDimension::D3 => quote!(wgpu::TextureViewDimension::D3),
                naga::ImageDimension::Cube => quote!(wgpu::TextureViewDimension::Cube),
            };

            match class {
                naga::ImageClass::Sampled { kind, multi } => {
                    let sample_type = match kind {
                        naga::ScalarKind::Sint => quote!(wgpu::TextureSampleType::Sint),
                        naga::ScalarKind::Uint => quote!(wgpu::TextureSampleType::Uint),
                        naga::ScalarKind::Float => {
                            // TODO: Don't assume all textures are filterable.
                            quote!(wgpu::TextureSampleType::Float { filterable: true })
                        }
                        _ => todo!(),
                    };
                    quote!(wgpu::BindingType::Texture {
                        sample_type: #sample_type,
                        view_dimension: #view_dim,
                        multisampled: #multi,
                    })
                }
                naga::ImageClass::Depth { multi } => {
                    quote!(wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: #view_dim,
                        multisampled: #multi,
                    })
                }
                naga::ImageClass::Storage { format, access } => {
                    // TODO: Will the debug implementation always work with the macro?
                    // Assume texture format variants are the same as storage formats.
                    let format = format_ident!("{format:?}");
                    let storage_access = storage_access(access);

                    quote!(wgpu::BindingType::StorageTexture {
                        access: #storage_access,
                        format: wgpu::TextureFormat::#format,
                        view_dimension: #view_dim,
                    })
                }
            }
        }
        naga::TypeInner::Sampler { comparison } => {
            let sampler_type = if comparison {
                quote!(wgpu::SamplerBindingType::Comparison)
            } else {
                quote!(wgpu::SamplerBindingType::Filtering)
            };
            quote!(wgpu::BindingType::Sampler(#sampler_type))
        }
        naga::TypeInner::AccelerationStructure => quote!(wgpu::BindingType::AccelerationStructure),
        _ => {
            let buffer_binding_type = buffer_binding_type(address_space);
            quote!(wgpu::BindingType::Buffer {
                ty: #buffer_binding_type,
                has_dynamic_offset: false,
                min_binding_size: None,
            })
        }
    }
}

fn storage_access(access: naga::StorageAccess) -> TokenStream {
    let is_read = access.contains(naga::StorageAccess::LOAD);
    let is_write = access.contains(naga::StorageAccess::STORE);
    match (is_read, is_write) {
        (true, true) => quote!(wgpu::StorageTextureAccess::ReadWrite),
        (true, false) => quote!(wgpu::StorageTextureAccess::ReadOnly),
        (false, true) => quote!(wgpu::StorageTextureAccess::WriteOnly),
        _ => unreachable!("Storage should be readable, writable, or both"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_tokens_eq;
    use indoc::indoc;

    #[test]
    fn globals_empty() {
        let source = indoc! {r#"
            @fragment
            fn main() {}
        "#};

        let module = naga::front::wgsl::parse_str(source).unwrap();
        let actual = globals(&module);
        assert_tokens_eq!(quote!(), actual);
    }

    fn test_globals(wgsl: &str, rust: &str) {
        let module = naga::front::wgsl::parse_str(wgsl).unwrap();
        let actual = globals(&module);

        assert_tokens_eq!(rust.parse().unwrap(), actual);
    }

    #[test]
    fn bind_groups_module_compute() {
        test_globals(
            include_str!("data/globals/compute.wgsl"),
            include_str!("data/globals/compute.rs"),
        );
    }

    #[test]
    fn bind_groups_module_vertex_fragment() {
        // Test different texture and sampler types.
        // TODO: Storage textures.
        test_globals(
            include_str!("data/globals/vertex_fragment.wgsl"),
            include_str!("data/globals/vertex_fragment.rs"),
        );
    }

    #[test]
    fn bind_groups_module_vertex() {
        // The actual content of the structs doesn't matter.
        // We only care about the groups and bindings.
        test_globals(
            include_str!("data/globals/vertex.wgsl"),
            include_str!("data/globals/vertex.rs"),
        );
    }

    #[test]
    fn bind_groups_module_fragment() {
        // The actual content of the structs doesn't matter.
        // We only care about the groups and bindings.
        test_globals(
            include_str!("data/globals/fragment.wgsl"),
            include_str!("data/globals/fragment.rs"),
        );
    }
}
