use image::GenericImageView;
use anyhow::*;
use crate::globals;

pub struct TextureWrapper {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl TextureWrapper {
    // todo: rework this to accept any number of textures instead of always 2
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        bytes2: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        let img2 = image::load_from_memory(bytes2)?;
        Self::from_image(device, queue, &img, &img2,Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        img2: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let rgba2 = img2.to_rgba8();
        let dimensions2 = img2.dimensions();
        assert!(dimensions == dimensions2, "Dimension of each image of layered textures must be the same!");

        // check binding for comment on why we are using multilayered texture
        let total_tx_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 2,
        };
        let single_layer_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size: total_tx_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: globals::TEXTURE_FORMAT,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            }
        );

        // upload first image
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            single_layer_size,
        );

        // upload second image
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: 1 }, // Second layer
            },
            &rgba2,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions2.0),
                rows_per_image: Some(dimensions2.1),
            },
            single_layer_size,
        );

        let view_descriptor = wgpu::TextureViewDescriptor {
            format: Some(globals::TEXTURE_FORMAT),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        };
        let view = texture.create_view(&view_descriptor);

        Ok(Self { texture, view })
    }
}
