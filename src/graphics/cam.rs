use cgmath::SquareMatrix;

use crate::utils::consts::OPENGL_TO_WGPU_MATRIX;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn get_cam_view_proj() -> [[f32; 4]; 4] {
        cgmath::Matrix4::identity().into()
    }

    pub fn update_view_proj(&self, mat: &mut [[f32; 4]; 4]) {
        let x = self.build_view_projection_matrix().into();
        *mat = x;
    }
}
