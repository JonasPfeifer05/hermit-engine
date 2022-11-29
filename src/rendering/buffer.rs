use bytemuck::Pod;
use wgpu::Buffer;
use wgpu::util::DeviceExt;
use crate::canvas::Canvas;

pub struct BufferBuilder {}

impl BufferBuilder {
    pub fn new<T: Pod>(content: &[T], usage: wgpu::BufferUsages, label: Option<&str>, canvas: &Canvas) -> Buffer {
        let buffer = canvas.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label,
                contents: bytemuck::cast_slice(content),
                usage,
            }
        );
        buffer
    }
}