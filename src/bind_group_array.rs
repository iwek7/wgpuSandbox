use wgpu::{StorageTextureAccess};
use crate::{globals,};

pub fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    label: &str,
    textures: &[&wgpu::TextureView],
    linear_sampler: &wgpu::Sampler,
    nearest_sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(textures),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(linear_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(nearest_sampler),
                }
            ],
            label: Some(label),
        }
    )
}

pub fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::StorageTexture {
                    // Apparently one can read and write to storage texture.
                    // It is one of the ways for GPU to commute w CPU.
                    // This setting (I think) dictates access right for this binding for CPU.
                    // To add to this by default WGPU does not support
                    // ReadOnly and WriteOnly so I don't really have too much choice while setting this
                    access: StorageTextureAccess::WriteOnly,
                    // see const for doc
                    format: globals::TEXTURE_FORMAT,
                    // It is another thing that informs GPU about data layout, I think.
                    view_dimension: Default::default(),
                },
                count: None,
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
        ],
        label: Some("texture_bind_group_layout"),
    })
}