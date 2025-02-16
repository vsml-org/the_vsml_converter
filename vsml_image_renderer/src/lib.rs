#[cfg(test)]
mod tests;

use vsml_common_image::Image as VsmlImage;
use vsml_core::{
    ImageEffectStyle, Property, Rect, Renderer, RenderingContext, RenderingInfo, TextData,
    TextRenderingInfo,
};
use wgpu::util::DeviceExt;

pub struct RendererImpl {
    images: Vec<(VsmlImage, RenderingInfo)>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

pub struct RenderingContextImpl {
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    base_width: u32,
    base_height: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        0 => Uint32,
        1 => Uint32,
        2 => Float32,
        3 => Float32,
        4 => Float32,
        5 => Float32,
    ];
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl Renderer for RendererImpl {
    type Image = VsmlImage;

    fn render_image(&mut self, image: Self::Image, info: RenderingInfo) {
        self.images.push((image, info));
    }

    fn render_text(&mut self, text_data: &[TextData], info: TextRenderingInfo) -> Rect {
        todo!()
    }

    fn render_box(&mut self, property: Property, info: RenderingInfo) {
        todo!()
    }

    fn render(self, width: u32, height: u32) -> Self::Image {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.images.iter().for_each(|(image, info)| {
            let child_view = image.create_view(&wgpu::TextureViewDescriptor::default());

            let diffuse_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&child_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
                label: None,
            });

            let vertex: &[Vertex] = &[Vertex {
                base_width: width,
                base_height: height,
                x: info.x,
                y: info.y,
                width: info.width,
                height: info.height,
            }];
            let vertex_buffer = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertex),
                    usage: wgpu::BufferUsages::VERTEX,
                });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            render_pass.set_pipeline(&self.render_pipeline); // 2.
            render_pass.set_bind_group(0, &diffuse_bind_group, &[]); // 1.
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            let max_width = if width < info.x as u32 + info.width as u32 {
                width - info.x as u32
            } else {
                info.width as u32
            };
            let max_height = if height < info.y as u32 + info.height as u32 {
                height - info.y as u32
            } else {
                info.height as u32
            };
            render_pass.set_scissor_rect(info.x as u32, info.y as u32, max_width, max_height);
            render_pass.draw(0..3, 0..1); // 3
        });
        self.queue.submit(std::iter::once(encoder.finish()));
        texture
    }
}

impl RenderingContext for RenderingContextImpl {
    type Image = VsmlImage;
    type Renderer = RendererImpl;

    fn create_renderer(&mut self) -> Self::Renderer {
        RendererImpl {
            images: vec![],
            device: self.device.clone(),
            queue: self.queue.clone(),
            render_pipeline: self.render_pipeline.clone(),
            texture_bind_group_layout: self.texture_bind_group_layout.clone(),
            sampler: self.sampler.clone(),
        }
    }

    fn apply_style(&mut self, image: Self::Image, style: ImageEffectStyle) -> Self::Image {
        todo!()
    }
}

impl RenderingContextImpl {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("render_image.wgsl").into()),
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            device,
            queue,
            render_pipeline,
            texture_bind_group_layout,
            sampler,
        }
    }
}
