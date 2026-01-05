#[cfg(test)]
mod tests;

use vsml_common_image::Image as VsmlImage;
use vsml_core::{
    ImageEffectStyle, ImageSize, RenderBoxProperty, Renderer, RenderingContext, RenderingInfo,
};
use wgpu::util::DeviceExt;

enum RenderItem {
    Image(VsmlImage, RenderingInfo),
    Box(RenderBoxProperty, RenderingInfo),
}

pub struct RendererImpl {
    items: Vec<RenderItem>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    box_render_pipeline: wgpu::RenderPipeline,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

pub struct RenderingContextImpl {
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    box_render_pipeline: wgpu::RenderPipeline,
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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct BoxVertex {
    base_width: u32,
    base_height: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: [f32; 4],
}

impl BoxVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        0 => Uint32,
        1 => Uint32,
        2 => Float32,
        3 => Float32,
        4 => Float32,
        5 => Float32,
        6 => Float32x4,
    ];
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<BoxVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl Renderer for RendererImpl {
    type Image = VsmlImage;

    fn render_image(&mut self, image: Self::Image, info: RenderingInfo) {
        self.items.push(RenderItem::Image(image, info));
    }

    fn render_box(&mut self, property: RenderBoxProperty, info: RenderingInfo) {
        self.items.push(RenderItem::Box(property, info));
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

        // 登録順に描画
        self.items.iter().for_each(|item| {
            match item {
                RenderItem::Box(property, info) => {
                    // 背景色がない場合はスキップ
                    let Some(color) = property.background_color else {
                        return;
                    };

                    if info.width == 0.0 || info.height == 0.0 {
                        return;
                    }

                    let normalized_color = [
                        color.r as f32 / 255.0,
                        color.g as f32 / 255.0,
                        color.b as f32 / 255.0,
                        color.a as f32 / 255.0,
                    ];

                    let box_vertex: &[BoxVertex] = &[BoxVertex {
                        base_width: width,
                        base_height: height,
                        x: info.x,
                        y: info.y,
                        width: info.width,
                        height: info.height,
                        color: normalized_color,
                    }];
                    let vertex_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: None,
                                contents: bytemuck::cast_slice(box_vertex),
                                usage: wgpu::BufferUsages::VERTEX,
                            });

                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                        multiview_mask: None,
                    });
                    render_pass.set_pipeline(&self.box_render_pipeline);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    let scissor_rect_width = if width < info.x as u32 + info.width as u32 {
                        width - info.x as u32
                    } else {
                        info.width as u32
                    };
                    let scissor_rect_height = if height < info.y as u32 + info.height as u32 {
                        height - info.y as u32
                    } else {
                        info.height as u32
                    };
                    render_pass.set_scissor_rect(
                        info.x as u32,
                        info.y as u32,
                        scissor_rect_width,
                        scissor_rect_height,
                    );
                    render_pass.draw(0..3, 0..1);
                }
                RenderItem::Image(image, info) => {
                    let child_view = image.create_view(&wgpu::TextureViewDescriptor::default());

                    let diffuse_bind_group =
                        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    let vertex_buffer =
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some("Vertex Buffer"),
                                contents: bytemuck::cast_slice(vertex),
                                usage: wgpu::BufferUsages::VERTEX,
                            });

                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                        multiview_mask: None,
                    });
                    render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.set_bind_group(0, &diffuse_bind_group, &[]);
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    let scissor_rect_width = if width < info.x as u32 + info.width as u32 {
                        width - info.x as u32
                    } else {
                        info.width as u32
                    };
                    let scissor_rect_height = if height < info.y as u32 + info.height as u32 {
                        height - info.y as u32
                    } else {
                        info.height as u32
                    };
                    render_pass.set_scissor_rect(
                        info.x as u32,
                        info.y as u32,
                        scissor_rect_width,
                        scissor_rect_height,
                    );
                    render_pass.draw(0..3, 0..1);
                }
            }
        });
        self.queue.submit(std::iter::once(encoder.finish()));
        texture
    }
}

impl RenderingContext for RenderingContextImpl {
    type Image = VsmlImage;
    type Renderer = RendererImpl;

    fn get_size(&self, image: &Self::Image) -> ImageSize {
        let size = image.size();
        ImageSize {
            width: size.width as f32,
            height: size.height as f32,
        }
    }

    fn create_renderer(&mut self) -> Self::Renderer {
        RendererImpl {
            items: vec![],
            device: self.device.clone(),
            queue: self.queue.clone(),
            render_pipeline: self.render_pipeline.clone(),
            box_render_pipeline: self.box_render_pipeline.clone(),
            texture_bind_group_layout: self.texture_bind_group_layout.clone(),
            sampler: self.sampler.clone(),
        }
    }

    fn apply_style(&mut self, _image: Self::Image, _style: ImageEffectStyle) -> Self::Image {
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
                immediate_size: 0,
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
            multiview_mask: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        // 背景色用のシェーダーとパイプライン
        let box_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Box Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("render_box.wgsl").into()),
        });
        let box_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Box Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });
        let box_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Box Render Pipeline"),
            layout: Some(&box_render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &box_shader,
                entry_point: Some("vs_main"),
                buffers: &[BoxVertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &box_shader,
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
            multiview_mask: None,
            cache: None,
        });

        Self {
            device,
            queue,
            render_pipeline,
            box_render_pipeline,
            texture_bind_group_layout,
            sampler,
        }
    }
}
