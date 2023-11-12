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
    in: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = in.tex_coords;
    out.clip_position = vec4<f32>(in.position.x, in.position.y, 0.0, 1.0);
    return out;
}

@group(0) @binding(0)
var depth_buffer_texture: texture_2d<f32>;
@group(0) @binding(1)
var depth_buffer_sampler: sampler;
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//
//          let near = 0.1;
//          let far = 100.0;
//          let depth =.x;
//          let r = (2.0 * near) / (far + near - depth * (far - near));
          return textureSample(depth_buffer_texture, depth_buffer_sampler, in.tex_coords);
}


