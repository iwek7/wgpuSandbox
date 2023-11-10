struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
    @location(9) use_linear_sampler: i32
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) use_linear_sampler: i32
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
   let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    out.use_linear_sampler = instance.use_linear_sampler;
    return out;
}


@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var linear_sampler: sampler;
@group(0) @binding(2)
var nearest_sampler: sampler;
//var samplers: vec2<sampler>

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

//    var sampler_to_use: sampler;
//    if(in.use_linear_sampler == 1) {
//      sampler_to_use = linear_sampler;
//    } else {
//      sampler_to_use = nearest_sampler;
//    }

    if(in.use_linear_sampler == 1) {
       return textureSample(t_diffuse, linear_sampler, in.tex_coords);
    } else {

    }
}


