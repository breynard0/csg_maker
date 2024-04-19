use crate::utils::consts::*;

use super::wgpu_object::WgpuObject;

pub fn create_multisampled_framebuffer(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    sample_count: u32,
) -> wgpu::TextureView {
    let multisampled_texture_extent = wgpu::Extent3d {
        width: config.width,
        height: config.height,
        depth_or_array_layers: 1,
    };

    let multisampled_frame_descriptor = &wgpu::TextureDescriptor {
        size: multisampled_texture_extent,
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: config.view_formats[0],
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        label: Some("MSAAFrameDescriptor"),
        view_formats: &[],
    };

    device
        .create_texture(multisampled_frame_descriptor)
        .create_view(&wgpu::TextureViewDescriptor::default())
}

pub fn create_bundle(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    pipeline: &wgpu::RenderPipeline,
    vertex_buffer: &wgpu::Buffer,
    index_buffer: &wgpu::Buffer,
    index_buffer_size: u32,
) -> wgpu::RenderBundle {
    let mut encoder = device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
        label: None,
        color_formats: &[Some(config.view_formats[0])],
        depth_stencil: Some(wgpu::RenderBundleDepthStencil {
            format: DEPTH_FORMAT,
            depth_read_only: false,
            stencil_read_only: false,
        }),
        sample_count: WgpuObject::SAMPLE_COUNT,
        multiview: None,
    });
    encoder.set_pipeline(&pipeline);
    encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
    encoder.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    encoder.draw_indexed(0..index_buffer_size, 0, 0..1);
    encoder.finish(&wgpu::RenderBundleDescriptor {
        label: Some("MSAA Render Bundle"),
    })
}

pub fn rebuild_msaa(wobj: &mut WgpuObject) {
    wobj.msaa_bundle = create_bundle(
        &wobj.device,
        &wobj.config,
        &wobj.pipeline,
        &wobj.vertex_buffer,
        &wobj.index_buffer,
        wobj.index_buffer_size,
    );
    wobj.msaa_buffer =
        create_multisampled_framebuffer(&wobj.device, &wobj.config, WgpuObject::SAMPLE_COUNT);
}
