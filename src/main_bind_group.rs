use std::num::NonZeroU32;
use wgpu::{StorageTextureAccess};
use winit::window::CursorIcon::Default;
use crate::{globals,};

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
) -> wgpu::BindGroup {
    device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout,
            entries: &[
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
                ty: wgpu::BindingType::Texture {
                    // It is another thing that informs GPU about data layout, I think.
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: false,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true }
                },
                count: Some(NonZeroU32::try_from(2).unwrap()),
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