use winit::{keyboard::KeyCode, window::Window};

use super::{init, input};

pub struct WgpuObject<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: &'a Window,
    pub pipeline: wgpu::RenderPipeline,
    pub pipeline_layout: wgpu::PipelineLayout,
    pub shader: wgpu::ShaderModule,
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_buffer_size: u32,
    pub index_buffer: wgpu::Buffer,
    pub index_buffer_size: u32,
    pub msaa_buffer: wgpu::TextureView,
    pub msaa_bundle: wgpu::RenderBundle,
    pub depth_texture: super::texture::Texture,
    pub wireframe: bool,
    pub delta_time: f32,
}

impl WgpuObject<'_> {
    pub const SAMPLE_COUNT: u32 = 8;

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.depth_texture =
                super::depth::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.surface.configure(&self.device, &self.config);
            super::msaa::rebuild_msaa(self);
        }
    }

    pub fn update(&mut self) {
        // Wireframe
        if input::is_key_pressed(KeyCode::F1) {
            self.pipeline = init::create_render_pipeline(
                &self.device,
                &self.pipeline_layout,
                &self.shader,
                &self.config,
                self.wireframe,
            );
        }

        input::input_update();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
