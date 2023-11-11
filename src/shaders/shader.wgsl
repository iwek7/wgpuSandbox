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
    @location(9) use_linear_sampler: i32,
    @location(10) texture_index: i32
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) use_linear_sampler: i32,
    @location(2) texture_index: i32
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
    out.texture_index = instance.texture_index;
    return out;
}


@group(0) @binding(0)
var myTextures: texture_2d_array<f32>;
@group(0) @binding(1)
var linear_sampler: sampler;
@group(0) @binding(2)
var nearest_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // See this to know why I use textureSampleLevel instead of textureSample.
    // https://stackoverflow.com/questions/77100370/how-to-conditionally-sample-a-texture-in-wgsl
    //
    // Apparently you can't use textureSample in conditional logic because of some Uniformity requirements.
    // See this https://www.w3.org/TR/WGSL/#uniformity
    //
    // As I vaugly understand it has something to do with the fact that this code can't be run
    // trully concurrently?
    if(in.use_linear_sampler == 1) {
    // as I understand I have one minimap per texture for now (texture itself) so I set 0 here
    // but maybe there are no minimaps and it defaults to sampling from texture?
    // who knows, for now I do not know almost anything about minimaps
       return textureSampleLevel(t_diffuse, linear_sampler, in.tex_coords,texture_index, 0.0);
    } else {
       return textureSampleLevel(t_diffuse, nearest_sampler, in.tex_coords,texture_index, 0.0);
    }
}


