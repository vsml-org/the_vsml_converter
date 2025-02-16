use std::collections::HashMap;
use image::GenericImageView;
use vsml_common_image::Image as VsmlImage;
use vsml_core::schemas::{ObjectProcessor, RectSize};

pub struct ImageProcessor {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl ImageProcessor {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self { device, queue }
    }
}

impl<A> ObjectProcessor<VsmlImage, A> for ImageProcessor {
    fn name(&self) -> &str {
        "image"
    }

    fn default_duration(&self, _attributes: &HashMap<String, String>) -> f64 {
        f64::INFINITY
    }

    fn default_image_size(&self, attributes: &HashMap<String, String>) -> RectSize {
        let src_path = attributes.get("src").unwrap();
        let image = image::open(src_path).unwrap();
        RectSize::new(image.width() as f32, image.height() as f32)
    }

    fn process_image(
        &self,
        _: f64,
        attributes: &HashMap<String, String>,
        _: Option<VsmlImage>,
    ) -> Option<VsmlImage> {
        let src_path = attributes.get("src").unwrap();
        let image = image::open(src_path).unwrap();
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );
        Some(texture)
    }

    fn process_audio(&self, _attributes: &HashMap<String, String>, _audio: Option<A>) -> Option<A> {
        None
    }
}
