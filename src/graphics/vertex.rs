use wgpu::{util::DeviceExt, Buffer};

use crate::utils::consts::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex {
    pub fn new(pos: [f32; 3], color: [f32; 4]) -> Self {
        Self { pos, color }
    }
}

pub struct BufferOutput {
    pub vbo: Buffer,
    pub vbo_size: u32,
    pub idxbuf: Buffer,
    pub idx_size: u32,
}

pub fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &VBO_ATTRIBS,
    };
    vertex_buffer_layout
}

pub fn test_cube(device: &wgpu::Device) -> BufferOutput {
    // let vertices = [
    //     Vertex::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 1.0]),
    //     Vertex::new([0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0]),
    //     Vertex::new([1.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]),
    //     Vertex::new([1.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0]),
    //     Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0, 1.0, 1.0]),
    //     Vertex::new([0.0, 1.0, 1.0], [0.0, 1.0, 1.0, 1.0]),
    //     Vertex::new([1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0]),
    //     Vertex::new([1.0, 1.0, 0.0], [1.0, 1.0, 0.0, 1.0]),
    // ];

    // let indices = [
    //     0, 3, 1, 1, 3, 2, 0, 1, 5, 0, 5, 4, 0, 4, 7, 0, 7, 3, 2, 3, 6, 3, 7, 6, 4, 5, 7, 5, 6, 7,
    //     1, 2, 5, 2, 6, 5,
    // ];

    let vertices = [
        Vertex::new([-0.3, -0.3, 1.0], [1.0, 0.0, 0.0, 1.0]),
        Vertex::new([0.3, -0.3, 0.0], [0.0, 1.0, 0.0, 1.0]),
        Vertex::new([0.3, 0.3, -0.5], [0.0, 0.0, 1.0, 1.0]),
        Vertex::new([-0.3, 0.3, 0.0], [1.0, 1.0, 1.0, 1.0]),
    ];

    let indices = [
        0, 1, 2, 
        0, 2, 3,
    ];

    let vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("VBO descriptor"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("IBO descriptor"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    BufferOutput {
        vbo,
        vbo_size: vertices.len() as u32,
        idxbuf: index_buffer,
        idx_size: indices.len() as u32,
    }
}
