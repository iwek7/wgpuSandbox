struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@group(0) @binding(0)
var depth_buffer_texture: texture_2d<f32>;
@group(0) @binding(1)
var depth_buffer_sampler: sampler;
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
   // Sample the depth buffer texture
    let depthValue = textureSample(depth_buffer_texture, depth_buffer_sampler, in.tex_coords).r;

    // Decide the color based on the depth value
    var color: vec4<f32>;
    if (depthValue == 1.0) {
        // If the depth value is 1.0 (far plane), render white
        color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    } else {
        // Otherwise, render black
        color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    return color;
}


