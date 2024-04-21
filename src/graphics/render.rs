use wgpu::SurfaceError;

use crate::AppData;

use super::rendering_object::RenderingObject;

pub fn render(appdata: &mut AppData) -> Result<(), SurfaceError> {
    let robj = &appdata.rendering_object;
    let output = robj.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor {
        label: Some("RenderTexture View"),
        format: Some(robj.config.view_formats[0]),
        dimension: Some(wgpu::TextureViewDimension::D2),
        aspect: wgpu::TextureAspect::All,
        ..Default::default()
    });

    let color_attachment = match RenderingObject::SAMPLE_COUNT {
        1 => wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        },
        _ => wgpu::RenderPassColorAttachment {
            view: &robj.msaa_buffer,
            resolve_target: Some(&view),
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Discard,
            },
        },
    };

    let mut encoder = robj
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("commandencoder"),
        });

    // Main Render Pass
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("MainRenderPass"),
            color_attachments: &[Some(color_attachment.clone())],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &robj.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.execute_bundles(std::iter::once(&robj.msaa_bundle));

        render_pass.set_pipeline(&robj.pipeline);
        render_pass.set_bind_group(0, &robj.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, robj.vertex_buffer.slice(..));
        render_pass.set_index_buffer(robj.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..robj.index_buffer_size, 0, 0..1);
    }

    robj.queue.submit(std::iter::once(encoder.finish()));

    output.present();

    Ok(())
}
