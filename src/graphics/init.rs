use wgpu::{Backends, FragmentState, Limits, TextureFormat, VertexState};

use super::{
    msaa,
    vertex::{self},
    wgpu_object::WgpuObject,
};

use crate::utils::consts::*;

pub async fn gfx_init(window: &winit::window::Window) -> WgpuObject {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: Backends::all(),
        dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        #[cfg(debug_assertions)]
        flags: wgpu::InstanceFlags::DEBUG | wgpu::InstanceFlags::VALIDATION,
        #[cfg(not(debug_assertions))]
        flags: wgpu::InstanceFlags::empty(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });

    let surface = instance
        .create_surface(window)
        .expect("Failed to create surface");

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Unable to create rendering adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("render_device"),
                required_features: adapter.features(),
                required_limits: Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .expect("Unable to request rendering device and queue");

    let size = window.inner_size();
    let wireframe = false;

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb() && f == &TextureFormat::Rgba8UnormSrgb)
        .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![surface_format],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    let depth_texture = super::depth::create_depth_texture(&device, &config, "depth_texture");

    let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/main.wgsl"));

    let vertex_index_buffer = vertex::test_cube(&device);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("RenderPipelineLayout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = create_render_pipeline(
        &device,
        &render_pipeline_layout,
        &shader,
        &config,
        wireframe,
    );

    let msaa_buffer =
        msaa::create_multisampled_framebuffer(&device, &config, WgpuObject::SAMPLE_COUNT);

    let msaa_bundle = msaa::create_bundle(
        &device,
        &config,
        &render_pipeline,
        &vertex_index_buffer.vbo,
        &vertex_index_buffer.idxbuf,
        vertex_index_buffer.idx_size,
    );

    let mut out = WgpuObject {
        surface,
        device,
        queue,
        config,
        size,
        window: window,
        pipeline: render_pipeline,
        pipeline_layout: render_pipeline_layout,
        shader,
        vertex_buffer: vertex_index_buffer.vbo,
        vertex_buffer_size: vertex_index_buffer.vbo_size,
        index_buffer: vertex_index_buffer.idxbuf,
        index_buffer_size: vertex_index_buffer.idx_size,
        msaa_buffer,
        msaa_bundle,
        depth_texture,
        wireframe,
        delta_time: 0.0,
    };

    out.update();

    out
}

pub fn create_render_pipeline<'a>(
    device: &wgpu::Device,
    render_pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    config: &wgpu::SurfaceConfiguration,
    wireframe: bool,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("RenderPipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[vertex::vertex_buffer_layout()],
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(config.view_formats[0].into())],
        }),
        primitive: wgpu::PrimitiveState {
            topology: match wireframe {
                true => wgpu::PrimitiveTopology::LineList,
                false => wgpu::PrimitiveTopology::TriangleList,
            },
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            // cull_mode: Some(wgpu::Face::Back),
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: WgpuObject::SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
