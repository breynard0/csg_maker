pub mod consts;
pub mod log;
pub mod tests;

pub fn push_if_absent<T>(vector: &mut Vec<T>, element: T)
where
    T: PartialEq,
{
    if !vector.contains(&element) {
        vector.push(element);
    }
}

pub fn empty_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("undefined buffer"),
        size: 0,
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: false,
    })
}
