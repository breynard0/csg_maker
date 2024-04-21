#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformData {
    pub cam_view_proj: [[f32; 4]; 4],
}
