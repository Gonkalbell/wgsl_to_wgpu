struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    // A fullscreen triangle.
    var out: VertexOutput;
    out.clip_position = vec4(in.position.xyz, 1.0);
    out.tex_coords = in.position.xy * 0.5 + 0.5;
    return out;
}

@group(0) @binding(0)
var color_texture: texture_2d<f32>;
@group(0) @binding(1)
var color_sampler: sampler;

struct Uniforms {
    color_rgb: vec3<f32>,
}

@group(1) @binding(0)
var<uniform> uniforms: Uniforms;

struct PushConstants {
    color_matrix: mat4x4<f32>
}

var<push_constant> constants: PushConstants;

override force_black: bool;
override scale: f32 = 1.0;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(color_texture, color_sampler, in.tex_coords).rgb;
    if force_black {
        return vec4(0.0);
    } else {
        return constants.color_matrix * vec4(color * uniforms.color_rgb.rgb * scale, 1.0);
    }
}

@group(0) @binding(0)
var<storage, read_write> storage_vars: Uniforms;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // A basic compute shader for demonstrating how to use generated bindings.
    if global_id.x == 0u {
        storage_vars.color_rgb = vec3(1.0);
    }
}