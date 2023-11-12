use std::num::NonZeroU32;

// My understanding of bind group is that it simply contains all the data that is entering shader.
// By using bind group we can describe what enters the shader:
// - as uniforms
// - as vertex shader input



pub fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    label: &str,
    texture_view: &wgpu::TextureView,
    linear_sampler: &wgpu::Sampler,
    nearest_sampler: &wgpu::Sampler,
    depth_texture_view: &wgpu::TextureView,
    depth_texture_sampler: &wgpu::Sampler
) -> wgpu::BindGroup {
    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                // As of now, WGPU does not support binding an array of separate texture bindings for dynamic indexing within a single shader or draw call.
                // Refer to this issue for more details: https://github.com/gpuweb/gpuweb/issues/822.
                // This limitation means that we cannot dynamically choose among an array of separate, non-uniform textures (e.g., textures with varying sizes) in a shader.
                //
                // The workaround is to create a single texture that contains multiple layers, known as a texture array. This texture array corresponds to 'texture_2d_array' in WGSL.
                // Each layer in this texture array can contain different image data, but all layers must have the same dimensions.
                // The shader can then dynamically index these layers within a single draw call.
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(linear_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(nearest_sampler),
                },
                wgpu::BindGroupEntry {
                    binding:3,
                    resource: wgpu::BindingResource::TextureView(depth_texture_view)
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(depth_texture_sampler),
                },
            ],
            label: Some(label),
        }
    )
}

pub fn create_bind_group_layout(device: &wgpu::Device, number_of_textures: u32) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    // It is another thing that informs GPU about data layout, I think.
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: false,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }
                },
                count: Some(NonZeroU32::try_from(number_of_textures).unwrap()),
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                // This should match the filterable field of the
                // corresponding Texture entry above.
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                // This should match the filterable field of the
                // corresponding Texture entry above.
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false }
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::FRAGMENT,
                // This should match the filterable field of the
                // corresponding Texture entry above.
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                count: None,
            },
        ],
        label: Some("texture_bind_group_layout"),
    })
}