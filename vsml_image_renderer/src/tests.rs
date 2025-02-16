use super::*;
use image::GenericImageView;
use wgpu::util::DeviceExt;

fn create_image_data(
    device: wgpu::Device,
    queue: wgpu::Queue,
    image_bytes: &[u8],
) -> (VsmlImage, RenderingInfo) {
    // 合成するテクスチャの作成
    let image = image::load_from_memory(image_bytes).unwrap();
    let rgba = image.to_rgba8();
    let dimensions = image.dimensions();
    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
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
    let info = RenderingInfo {
        x: 0.0,
        y: 0.0,
        width: dimensions.0 as f32,
        height: dimensions.1 as f32,
    };
    (texture, info)
}

#[test]
fn test_render() {
    // GPUのdeviceとqueueを作成
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
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: None,
            memory_hints: Default::default(),
        },
        None,
    ))
    .unwrap();

    // rendering_contextとrendererを作成
    let mut context = RenderingContextImpl::new(device.clone(), queue.clone());
    let mut renderer = context.create_renderer();

    // 合成するテクスチャの作成
    let (texture, info) = create_image_data(
        device.clone(),
        queue.clone(),
        include_bytes!("../test_assets/origin.png"),
    );
    renderer.render_image(texture, info);
    let (texture, info) = create_image_data(
        device.clone(),
        queue.clone(),
        include_bytes!("../test_assets/red.png"),
    );
    renderer.render_image(texture, info);
    let (texture, info) = create_image_data(
        device.clone(),
        queue.clone(),
        include_bytes!("../test_assets/portrait-alpha.png"),
    );
    renderer.render_image(texture, info);
    let (texture, info) = create_image_data(
        device.clone(),
        queue.clone(),
        include_bytes!("../test_assets/icon.png"),
    );
    renderer.render_image(texture, info);

    // テクスチャを合成
    let output_dimensions = (1920, 1080);
    let result = renderer.render(output_dimensions.0, output_dimensions.1);

    // assert
    assert_eq!(result.width(), 1920);
    assert_eq!(result.height(), 1080);

    // 試しに画像を保存
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: &vec![0u8; output_dimensions.0 as usize * output_dimensions.1 as usize * 4],
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
    });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &result,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * output_dimensions.0),
                rows_per_image: Some(output_dimensions.1),
            },
        },
        wgpu::Extent3d {
            width: output_dimensions.0,
            height: output_dimensions.1,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(std::iter::once(encoder.finish()));

    let slice = &buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});

    device.poll(wgpu::MaintainBase::Wait);

    image::save_buffer(
        "output.png",
        &slice.get_mapped_range().to_vec(),
        output_dimensions.0,
        output_dimensions.1,
        image::ColorType::Rgba8,
    )
    .unwrap();
}
