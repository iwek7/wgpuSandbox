use wgpu::util::DeviceExt;
use crate::depth_visualisation_bind_group::{create_depth_vis_bind_group, create_depth_vis_bind_group_layout};
use crate::{tx, model};
use crate::model::ModelVertex;

pub struct DepthState {
    pub depth_texture: tx::TextureWrapper,
    depth_sampler: wgpu::Sampler,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}


const WHOLE_SCREEN_VERTICES: &[model::ModelVertex] = &[
    model::ModelVertex {
        position: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 1.0],
        normal: [0.0, 0.0, 0.0]
    },
    model::ModelVertex {
        position: [1.0, 0.0, 0.0],
        tex_coords: [1.0, 1.0],
        normal: [0.0, 0.0, 0.0]
    },
    model::ModelVertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
        normal: [0.0, 0.0, 0.0]
    },
    model::ModelVertex {
        position: [0.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
        normal: [0.0, 0.0, 0.0]
    },
];

const WHOLE_SCREEN_INDICES: &[u16] = &[
    0, 1, 2, 0, 2, 3
];


impl DepthState {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> DepthState {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(WHOLE_SCREEN_VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(WHOLE_SCREEN_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = WHOLE_SCREEN_INDICES.len() as u32;


        let depth_texture = tx::TextureWrapper::create_depth_texture(
            &device, &config, "depth_texture",
        );

        let depth_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            // mag_filter: wgpu::FilterMode::Linear,
            // min_filter: wgpu::FilterMode::Linear,
            // mipmap_filter: wgpu::FilterMode::Nearest,
            // compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        let bind_group_layout = create_depth_vis_bind_group_layout(device);

        let bind_group = create_depth_vis_bind_group(
            device, &bind_group_layout, &depth_texture.view, &depth_sampler);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/depth_visualisation_shader.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &bind_group_layout,
                ],
                push_constant_ranges: &[],
            }
        );

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Depth Pass Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    <ModelVertex as model::Vertex>::desc()
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            depth_texture,
            depth_sampler,
            bind_group_layout,
            bind_group,
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    pub fn resize(&mut self,
                  device: &wgpu::Device,
                  config: &wgpu::SurfaceConfiguration,
    ) {
        self.depth_texture = tx::TextureWrapper::create_depth_texture(device, config, "depth_texture");
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.depth_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.depth_sampler),
                },
            ],
            label: Some("depth_pass.bind_group"),
        });
    }

    pub fn build_render_pass(&mut self,
                             encoder: &mut wgpu::CommandEncoder,
                             texture_view: &wgpu::TextureView,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Depth Visualisation Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    // so if I clear color here then
                    // all results of previous render passes will be overridern
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}