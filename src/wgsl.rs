use std::collections::BTreeMap;

use naga::{Handle, StructMember, Type};

pub struct GroupData<'a> {
    pub bindings: Vec<GroupBinding<'a>>,
}

pub struct GroupBinding<'a> {
    pub name: Option<String>,
    pub binding_index: u32,
    pub binding_type: &'a naga::Type,
}

// TODO: Improve error handling/error reporting.

fn rust_scalar_type(kind: naga::ScalarKind, width: u8) -> String {
    // TODO: Support other widths?
    match (kind, width) {
        (naga::ScalarKind::Sint, 4) => "i32".to_string(),
        (naga::ScalarKind::Uint, 4) => "u32".to_string(),
        (naga::ScalarKind::Float, 4) => "f32".to_string(),
        // TODO: Do booleans have a width?
        (naga::ScalarKind::Bool, _) => "bool".to_string(),
        _ => todo!(),
    }
}

// TODO: Can this be made easier to test?
pub fn rust_type(module: &naga::Module, ty: &naga::Type) -> String {
    match &ty.inner {
        naga::TypeInner::Scalar { kind, width } => rust_scalar_type(*kind, *width),
        naga::TypeInner::Vector { size, kind, width } => match size {
            naga::VectorSize::Bi => format!("[{}; 2]", rust_scalar_type(*kind, *width)),
            naga::VectorSize::Tri => format!("[{}; 3]", rust_scalar_type(*kind, *width)),
            naga::VectorSize::Quad => format!("[{}; 4]", rust_scalar_type(*kind, *width)),
        },
        naga::TypeInner::Matrix {
            columns,
            rows,
            width,
        } => match (rows, columns, width) {
            // TODO: Don't force glam here?
            (naga::VectorSize::Quad, naga::VectorSize::Quad, 4) => "glam::Mat4".to_string(),
            _ => todo!(),
        },
        naga::TypeInner::Image { .. } => todo!(),
        naga::TypeInner::Sampler { .. } => todo!(),
        naga::TypeInner::Atomic { kind, width } => todo!(),
        naga::TypeInner::Pointer { base, class } => todo!(),
        naga::TypeInner::ValuePointer {
            size,
            kind,
            width,
            class,
        } => todo!(),
        naga::TypeInner::Array { base, size, stride } => {
            // TODO: Support arrays other than arrays with a static size?
            let element_type = rust_type(module, &module.types[*base]);
            let count = array_length(size, module);
            format!("[{element_type}; {count}]")
        }
        naga::TypeInner::Struct { members, span } => todo!(),
    }
}

pub fn vertex_format(ty: &naga::Type) -> wgpu::VertexFormat {
    // Not all wgsl types work as vertex attributes in wgpu.
    match &ty.inner {
        naga::TypeInner::Scalar { kind, width } => todo!(),
        naga::TypeInner::Vector { size, kind, width } => match size {
            naga::VectorSize::Bi => match (kind, width) {
                (naga::ScalarKind::Uint, 4) => wgpu::VertexFormat::Uint32x2,
                (naga::ScalarKind::Float, 4) => wgpu::VertexFormat::Float32x2,
                _ => todo!(),
            },
            naga::VectorSize::Tri => match (kind, width) {
                (naga::ScalarKind::Uint, 4) => wgpu::VertexFormat::Uint32x3,
                (naga::ScalarKind::Float, 4) => wgpu::VertexFormat::Float32x3,
                _ => todo!(),
            },
            naga::VectorSize::Quad => match (kind, width) {
                (naga::ScalarKind::Uint, 4) => wgpu::VertexFormat::Uint32x4,
                (naga::ScalarKind::Float, 4) => wgpu::VertexFormat::Float32x4,
                _ => todo!(),
            },
        },
        _ => todo!(), // are these types even valid as attributes?
    }
}

fn array_length(size: &naga::ArraySize, module: &naga::Module) -> usize {
    match size {
        naga::ArraySize::Constant(c) => match &module.constants[*c].inner {
            naga::ConstantInner::Scalar { value, .. } => match value {
                naga::ScalarValue::Sint(v) => *v as usize,
                naga::ScalarValue::Uint(v) => *v as usize,
                naga::ScalarValue::Float(_) => todo!(),
                naga::ScalarValue::Bool(_) => todo!(),
            },
            _ => todo!(),
        },
        naga::ArraySize::Dynamic => todo!(),
    }
}

pub fn get_bind_group_data(module: &naga::Module) -> BTreeMap<u32, GroupData> {
    // Use a BTree to sort type and field names by group index.
    // This isn't strictly necessary but makes the generated code cleaner.
    let mut groups = BTreeMap::new();

    for global_handle in module.global_variables.iter() {
        let global = &module.global_variables[global_handle.0];
        if let Some(binding) = &global.binding {
            let group = groups.entry(binding.group).or_insert(GroupData {
                bindings: Vec::new(),
            });
            let binding_type = &module.types[module.global_variables[global_handle.0].ty];

            // Assume bindings are unique since duplicates would trigger a WGSL compiler error.
            let group_binding = GroupBinding {
                name: global.name.clone(),
                binding_index: binding.binding,
                binding_type,
            };
            group.bindings.push(group_binding);
        }
    }

    groups
}

pub struct VertexInput {
    pub name: String,
    pub fields: Vec<(u32, StructMember)>,
}

// TODO: Handle errors.
// Collect the necessary data to generate an equivalent Rust struct.
pub fn get_vertex_input_structs(module: &naga::Module) -> Vec<VertexInput> {
    let vertex_entry = module
        .entry_points
        .iter()
        .find(|e| e.stage == naga::ShaderStage::Vertex)
        .unwrap();

    let mut structs = Vec::new();

    for argument in &vertex_entry.function.arguments {
        // For entry points, arguments must have a binding unless they are a structure.
        if let Some(binding) = &argument.binding {
            // TODO: How to create a structure for regular bindings?
        } else {
            let arg_type = &module.types[argument.ty];
            match &arg_type.inner {
                naga::TypeInner::Struct { members, span: _ } => {
                    let input = VertexInput {
                        name: arg_type.name.as_ref().unwrap().clone(),
                        fields: members
                            .iter()
                            .map(|member| {
                                let location = match member.binding.as_ref().unwrap() {
                                    naga::Binding::BuiltIn(_) => todo!(), // TODO: is it possible to have builtins for inputs?
                                    naga::Binding::Location { location, .. } => *location,
                                };

                                (location, member.clone())
                            })
                            .collect(),
                    };

                    structs.push(input);
                }
                // This case should be prevented by the checks above.
                _ => unreachable!(),
            }
        }
    }

    structs
}

pub fn get_vertex_input_locations(module: &naga::Module) -> Vec<(String, u32)> {
    let vertex_entry = module
        .entry_points
        .iter()
        .find(|e| e.stage == naga::ShaderStage::Vertex)
        .unwrap();

    let mut shader_locations = Vec::new();

    for argument in &vertex_entry.function.arguments {
        // For entry points, arguments must have a binding unless they are a structure.
        if let Some(binding) = &argument.binding {
            if let naga::Binding::Location { location, .. } = binding {
                shader_locations.push((argument.name.clone().unwrap(), *location));
            }
        } else {
            let arg_type = &module.types[argument.ty];
            match &arg_type.inner {
                naga::TypeInner::Struct { members, span: _ } => {
                    for member in members {
                        match member.binding.as_ref().unwrap() {
                            naga::Binding::BuiltIn(_) => (),
                            naga::Binding::Location { location, .. } => {
                                shader_locations.push((member.name.clone().unwrap(), *location))
                            }
                        }
                    }
                }
                // This case should be prevented by the checks above.
                _ => unreachable!(),
            }
        }
    }

    shader_locations
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn vertex_input_structs_two_structs() {
        let source = indoc! {r#"
            struct VertexInput0 {
                [[location(0)]] in0: vec4<f32>;
                [[location(1)]] in1: vec4<f32>;
                [[location(2)]] in2: vec4<f32>;
            };
            
            struct VertexInput1 {
                [[location(3)]] in3: vec4<f32>;
                [[location(4)]] in4: vec4<f32>;
                [[location(5)]] in5: vec4<f32>;
                [[location(6)]] in6: vec4<u32>;
            };

            [[stage(vertex)]]
            fn main(
                in0: VertexInput0,
                in1: VertexInput1
            ) -> [[builtin(position)]] vec4<f32> {
                return vec4<f32>(0.0);
            }
        "#};

        let module = naga::front::wgsl::parse_str(source).unwrap();

        let vertex_inputs = get_vertex_input_structs(&module);
        assert_eq!(2, vertex_inputs.len());

        assert_eq!("VertexInput0", vertex_inputs[0].name);
        assert_eq!(3, vertex_inputs[0].fields.len());
        assert_eq!("in1", vertex_inputs[0].fields[1].1.name.as_ref().unwrap());
        assert_eq!(1, vertex_inputs[0].fields[1].0);

        assert_eq!("VertexInput1", vertex_inputs[1].name);
        assert_eq!(4, vertex_inputs[1].fields.len());
        assert_eq!("in5", vertex_inputs[1].fields[2].1.name.as_ref().unwrap());
        assert_eq!(5, vertex_inputs[1].fields[2].0);
    }

    #[test]
    fn vertex_locations_struct_two_fields() {
        let source = indoc! {r#"
            struct VertexInput {
                [[location(0)]] position: vec3<f32>;
                [[location(1)]] tex_coords: vec2<f32>;
            };

            [[stage(vertex)]]
            fn main(
                model: VertexInput,
            ) -> [[builtin(position)]] vec4<f32> {
                return vec4<f32>(0.0);
            }
        "#};
        let module = naga::front::wgsl::parse_str(source).unwrap();

        let shader_locations = get_vertex_input_locations(&module);
        assert_eq!(
            &[("position".to_string(), 0), ("tex_coords".to_string(), 1)],
            &shader_locations[..]
        );
    }

    #[test]
    fn vertex_locations_struct_no_fields() {
        let source = indoc! {r#"
            struct VertexInput {
            };

            [[stage(vertex)]]
            fn main(
                model: VertexInput,
            ) -> [[builtin(position)]] vec4<f32> {
                return vec4<f32>(0.0);
            }
        "#};
        let module = naga::front::wgsl::parse_str(source).unwrap();

        let shader_locations = get_vertex_input_locations(&module);
        assert!(shader_locations.is_empty());
    }

    #[test]
    fn vertex_locations_struct_builtin_field() {
        let source = indoc! {r#"
            struct VertexInput {
                [[builtin(vertex_index)]] VertexIndex : u32;
            };

            [[stage(vertex)]]
            fn main(
                model: VertexInput,
            ) -> [[builtin(position)]] vec4<f32> {
                return vec4<f32>(0.0);
            }
        "#};
        let module = naga::front::wgsl::parse_str(source).unwrap();

        let shader_locations = get_vertex_input_locations(&module);
        assert!(shader_locations.is_empty());
    }

    #[test]
    fn vertex_locations_struct_builtin_parameter() {
        let source = indoc! {r#"
            [[stage(vertex)]]
            fn main(
                [[builtin(vertex_index)]] VertexIndex : u32,
            ) -> [[builtin(position)]] vec4<f32> {
                return vec4<f32>(0.0);
            }
        "#};
        let module = naga::front::wgsl::parse_str(source).unwrap();

        let shader_locations = get_vertex_input_locations(&module);
        assert!(shader_locations.is_empty());
    }

    #[test]
    fn vertex_locations_two_parameters() {
        let source = indoc! {r#"
            [[stage(vertex)]]
            fn main([[location(0)]] position: vec4<f32>,
                    [[location(1)]] tex_coords: vec2<f32>
            ) -> [[builtin(position)]] vec4<f32> {
                return vec4<f32>(0.0);
            }
        "#};
        let module = naga::front::wgsl::parse_str(source).unwrap();

        let shader_locations = get_vertex_input_locations(&module);
        assert_eq!(
            &[("position".to_string(), 0), ("tex_coords".to_string(), 1)],
            &shader_locations[..]
        );
    }
}
