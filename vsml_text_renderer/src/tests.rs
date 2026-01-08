use super::*;
use std::path::PathBuf;
use vsml_core::schemas::{Color, TextData, TextStyleData};
use vsml_test_utils::vrt_out_path;
use wgpu::util::DeviceExt;

fn create_gpu_context() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .unwrap();
    pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        label: None,
        memory_hints: Default::default(),
        experimental_features: Default::default(),
        trace: Default::default(),
    }))
    .unwrap()
}

fn save_texture_to_file(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    width: u32,
    height: u32,
    path: PathBuf,
) {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Save Texture Encoder"),
    });

    let bytes_per_pixel = 4u32;
    let unpadded_bytes_per_row = bytes_per_pixel * width;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = (unpadded_bytes_per_row + align - 1) / align * align;

    let buffer_size = (padded_bytes_per_row * height) as usize;
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Save Texture Buffer"),
        contents: &vec![0u8; buffer_size],
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
    });

    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(std::iter::once(encoder.finish()));

    let slice = &buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});

    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        })
        .unwrap();

    let data = slice.get_mapped_range();
    if padded_bytes_per_row == unpadded_bytes_per_row {
        image::save_buffer(path, &data, width, height, image::ColorType::Rgba8).unwrap();
    } else {
        let mut unpadded_data = Vec::with_capacity((unpadded_bytes_per_row * height) as usize);
        for row in 0..height {
            let offset = (row * padded_bytes_per_row) as usize;
            unpadded_data
                .extend_from_slice(&data[offset..offset + unpadded_bytes_per_row as usize]);
        }
        image::save_buffer(path, &unpadded_data, width, height, image::ColorType::Rgba8).unwrap();
    }
}

#[test]
fn test_render_simple_text_vrt() {
    let (device, queue) = create_gpu_context();
    let context = TextRendererContext::new(device.clone(), queue.clone());

    let text_data = vec![TextData {
        text: "Hello World".to_string(),
        style: TextStyleData {
            color: Color::from_rgb(255, 255, 255),
            font_size: 32.0,
            font_family: vec![],
        },
    }];

    let texture = context.render_text(&text_data);

    let size = texture.size();
    assert!(size.width > 0);
    assert!(size.height > 0);

    save_texture_to_file(
        &device,
        &queue,
        &texture,
        size.width,
        size.height,
        vrt_out_path!("test_render_simple_text.png"),
    );
}

#[test]
fn test_render_text_with_color_vrt() {
    let (device, queue) = create_gpu_context();
    let context = TextRendererContext::new(device.clone(), queue.clone());

    let text_data = vec![TextData {
        text: "Colored Text".to_string(),
        style: TextStyleData {
            color: Color::from_rgb(255, 0, 0), // 赤色
            font_size: 48.0,
            font_family: vec![],
        },
    }];

    let texture = context.render_text(&text_data);

    let size = texture.size();
    assert!(size.width > 0);
    assert!(size.height > 0);

    save_texture_to_file(
        &device,
        &queue,
        &texture,
        size.width,
        size.height,
        vrt_out_path!("test_render_text_with_color.png"),
    );
}

#[test]
fn test_render_text_with_alpha_vrt() {
    let (device, queue) = create_gpu_context();
    let context = TextRendererContext::new(device.clone(), queue.clone());

    let text_data = vec![TextData {
        text: "Semi-transparent Text".to_string(),
        style: TextStyleData {
            color: Color::from(0, 255, 0, 128),
            font_size: 40.0,
            font_family: vec![],
        },
    }];

    let texture = context.render_text(&text_data);

    let size = texture.size();
    assert!(size.width > 0);
    assert!(size.height > 0);

    save_texture_to_file(
        &device,
        &queue,
        &texture,
        size.width,
        size.height,
        vrt_out_path!("test_render_text_with_alpha.png"),
    );
}

#[test]
fn test_render_text_different_sizes_vrt() {
    let (device, queue) = create_gpu_context();
    let context = TextRendererContext::new(device.clone(), queue.clone());

    let small_text = vec![TextData {
        text: "Small".to_string(),
        style: TextStyleData {
            color: Color::WHITE,
            font_size: 16.0,
            font_family: vec![],
        },
    }];
    let small_texture = context.render_text(&small_text);
    let small_size = small_texture.size();

    let large_text = vec![TextData {
        text: "Large".to_string(),
        style: TextStyleData {
            color: Color::WHITE,
            font_size: 64.0,
            font_family: vec![],
        },
    }];
    let large_texture = context.render_text(&large_text);
    let large_size = large_texture.size();

    assert!(large_size.height > small_size.height);

    save_texture_to_file(
        &device,
        &queue,
        &small_texture,
        small_size.width,
        small_size.height,
        vrt_out_path!("test_render_text_small.png"),
    );

    save_texture_to_file(
        &device,
        &queue,
        &large_texture,
        large_size.width,
        large_size.height,
        vrt_out_path!("test_render_text_large.png"),
    );
}

#[test]
fn test_render_japanese_text_vrt() {
    let (device, queue) = create_gpu_context();
    let context = TextRendererContext::new(device.clone(), queue.clone());

    let text_data = vec![TextData {
        text: "こんにちは世界".to_string(),
        style: TextStyleData {
            color: Color::from_rgb(0, 0, 255),
            font_size: 36.0,
            font_family: vec![],
        },
    }];

    let texture = context.render_text(&text_data);

    let size = texture.size();
    assert!(size.width > 0);
    assert!(size.height > 0);

    save_texture_to_file(
        &device,
        &queue,
        &texture,
        size.width,
        size.height,
        vrt_out_path!("test_render_japanese_text.png"),
    );
}

#[test]
fn test_calculate_text_size_simple() {
    let (device, queue) = create_gpu_context();
    let context = TextRendererContext::new(device, queue);

    let text_data = vec![TextData {
        text: "Test".to_string(),
        style: TextStyleData {
            color: Color::WHITE,
            font_size: 32.0,
            font_family: vec![],
        },
    }];

    let size = context.calculate_text_size(&text_data);

    assert!(size.width > 0.0);
    assert!(size.height > 0.0);
}

#[test]
fn test_calculate_text_size_empty() {
    let (device, queue) = create_gpu_context();
    let context = TextRendererContext::new(device, queue);

    let text_data = vec![TextData {
        text: "".to_string(),
        style: TextStyleData {
            color: Color::WHITE,
            font_size: 32.0,
            font_family: vec![],
        },
    }];

    let size = context.calculate_text_size(&text_data);

    assert_eq!(size.width, 0.0);
    assert!(size.height > 0.0);
}

#[test]
fn test_calculate_text_size_comparison() {
    let (device, queue) = create_gpu_context();
    let context = TextRendererContext::new(device, queue);

    let short_text = vec![TextData {
        text: "Hi".to_string(),
        style: TextStyleData {
            color: Color::WHITE,
            font_size: 32.0,
            font_family: vec![],
        },
    }];

    let long_text = vec![TextData {
        text: "Hello World".to_string(),
        style: TextStyleData {
            color: Color::WHITE,
            font_size: 32.0,
            font_family: vec![],
        },
    }];

    let short_size = context.calculate_text_size(&short_text);
    let long_size = context.calculate_text_size(&long_text);

    assert!(long_size.width > short_size.width);
    assert_eq!(long_size.height, short_size.height);
}

#[test]
fn test_calculate_text_size_different_font_sizes() {
    let (device, queue) = create_gpu_context();
    let context = TextRendererContext::new(device, queue);

    let small_text = vec![TextData {
        text: "Test".to_string(),
        style: TextStyleData {
            color: Color::WHITE,
            font_size: 16.0,
            font_family: vec![],
        },
    }];

    let large_text = vec![TextData {
        text: "Test".to_string(),
        style: TextStyleData {
            color: Color::WHITE,
            font_size: 64.0,
            font_family: vec![],
        },
    }];

    let small_size = context.calculate_text_size(&small_text);
    let large_size = context.calculate_text_size(&large_text);

    assert!(large_size.width > small_size.width);
    assert!(large_size.height > small_size.height);
}
