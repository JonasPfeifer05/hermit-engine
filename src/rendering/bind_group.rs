use wgpu::{BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer, ShaderStages};
use crate::canvas::Canvas;

pub struct BindGroupBuilder {

}

impl BindGroupBuilder {
    pub fn new(canvas: &Canvas, layout_entries: &[BindGroupLayoutEntry], group_entries: &[BindGroupEntry], label: Option<&str>, group: bool) -> (BindGroupLayout, Option<BindGroup>) {
        let group_layout = canvas.device.create_bind_group_layout(
          &wgpu::BindGroupLayoutDescriptor {
              entries: layout_entries,
              label,
          }
        );

        if !group { return (group_layout, None); }

        let group = canvas.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &group_layout,
            entries: group_entries,
            label,
        });

        (group_layout, Some(group))
    }
}

pub struct LayoutEntry {

}

impl LayoutEntry {
    pub fn new(binding: u32, visibility: ShaderStages, ty: BindingType) -> BindGroupLayoutEntry {
        BindGroupLayoutEntry {
            binding,
            visibility,
            ty,
            count: None,
        }
    }
}

pub struct GroupEntry {

}

impl GroupEntry {
    pub fn new(binding: u32, buffer: &Buffer) -> BindGroupEntry {
        BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        }
    }
}