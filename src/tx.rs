use anyhow::*;
use image::GenericImageView;
use crate::globals;

pub struct TextureWrapper {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
}

impl TextureWrapper {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
    pub fn multilayer_from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[&[u8]],
        label: &str,
    ) -> Result<Self> {
        let images: Vec<image::DynamicImage> = bytes.iter()
            .map(|b| image::load_from_memory(*b).unwrap())
            .collect();
        Self::multilayer_from_images(device, queue, &images, Some(label))
    }

    /**
        This creates multilayered texture from provided data.
        Such textures are useful to bind them to texture_2d_array slot in shader so that it is
        possible to dynamically choose which texture to render in shader.

        Each individual texture is separate layer.
        Limitation of this approach is that all images need to have same dimensions.
    */
    pub fn multilayer_from_images(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        images: &[image::DynamicImage],
        label: Option<&str>,
    ) -> Result<Self> {

        assert!(images.len() > 0, "Trying to create layered texture with no images");
        let base_dimensions = images[0].dimensions();

        let total_tx_size = wgpu::Extent3d {
            width: base_dimensions.0,
            height: base_dimensions.1,
            depth_or_array_layers: 2,
        };
        let single_layer_size = wgpu::Extent3d {
            width: base_dimensions.0,
            height: base_dimensions.1,
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

        for image_idx in 0..images.len() {
            let image = &images[image_idx];

            // This has to match texture format
            // todo: remove this assumption, chosen format should have assigned function to choose data from image
            let rgba_data = image.to_rgba8();
            assert_eq!(base_dimensions, image.dimensions(), "Dimension of each image of layered textures must be the same!");
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    // z coordinate here is layer
                    origin: wgpu::Origin3d { x: 0, y: 0, z: image_idx as u32 },
                },
                &rgba_data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * base_dimensions.0),
                    rows_per_image: Some(base_dimensions.1),
                },
                single_layer_size,
            );

        }

        let view_descriptor = wgpu::TextureViewDescriptor {
            format: Some(globals::TEXTURE_FORMAT),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        };
        let view = texture.create_view(&view_descriptor);

        Ok(Self { texture, view })
    }

    // A depth buffer, also known as a z-buffer, is used to implement depth testing in 3D rendering.
    // This technique ensures that closer objects are drawn in front of those
    // that are farther away from the camera.
    // When rendering a pixel, the GPU compares its depth value (z-value)
    // with the corresponding value in the depth buffer.
    // - If the existing value in the depth buffer is closer to the camera
    //   (i.e., smaller), the new pixel is not drawn (it's occluded by something closer).
    // - If the depth buffer position is empty (nothing has been drawn there yet)
    //   or the existing depth value is farther away, the new pixel is drawn,
    //   and its depth value is stored in the depth buffer.
    // Note that the depth buffer does not inherently prevent all overdraws.
    // In cases where objects are processed in a back-to-front order,
    // pixels from objects in the back are drawn and then potentially overwritten by nearer objects.
    // While this results in overdraw, the final rendered image correctly reflects the nearest visible surfaces.
    // Advanced techniques like early Z-testing/Z-culling can help
    // minimize such overdraw by discarding fragments that would be occluded before running fragment shaders.
    // todo: what is early z-testing / z-culling ?
    pub fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, label: &str) -> Self {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING ,
            view_formats: &[Self::DEPTH_FORMAT],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, view }
    }
}
