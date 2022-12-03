use wgpu::{BindGroup, BindGroupLayout, Buffer, BufferUsages, FragmentState, RenderPass, RenderPipeline, ShaderModule, VertexState};
use wgpu::IndexFormat::Uint16;
use crate::INDICES2;
use crate::rendering::buffer::BufferBuilder;
use crate::rendering::canvas::Canvas;
use crate::rendering::pipeline::Pipeline;
use crate::rendering::shader::{FragmentEntry, Shader, VertexEntry};

pub struct ShapeDrawer<'a, 'b> where 'a: 'b {
    render_pass: &'b mut RenderPass<'a>,
}

impl<'a, 'b> ShapeDrawer<'a, 'b> {
    pub fn new(render_pass: &'b mut RenderPass<'a>) -> Self {
        Self { render_pass }
    }

    pub fn draw_shape(&mut self, shape: &'a impl Shape<'a>) {
        shape.draw(self.render_pass);
    }
}

pub trait Shape<'a> {
    fn draw<'b>(&'a self, render_pass: &'b mut RenderPass<'a>) where 'a: 'b;
}

pub struct ShapeData<'a> {
    vertices: &'a [Vertex],
    indices: Option<&'a [u16]>,
}

impl<'a> ShapeData<'a> {
    pub fn new(vertices: &'a [Vertex], indices: Option<&'a [u16]>) -> Self {
        Self { vertices, indices, }
    }
}

pub struct Rectangle<'a> {
    vertices: &'a [Vertex],
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pipeline: RenderPipeline,
    texture: Option<(BindGroupLayout,BindGroup)>,
}

impl<'a> Rectangle<'a> {
    const INDICES_RECTANGLE: &'a [u16] = &[0, 2, 1, 0, 3, 2];

    pub fn new(shader: &Shader, vertices: &'a [Vertex], texture: Option<(BindGroupLayout, BindGroup)>, canvas: &Canvas) -> Self {
        let vertex = BufferBuilder::new(vertices, BufferUsages::VERTEX, Some("Vertex"), canvas);
        let index = BufferBuilder::new(Rectangle::INDICES_RECTANGLE, BufferUsages::INDEX, Some("Index"), canvas);

        let layouts = if texture.is_some() {
            vec![&texture.as_ref().unwrap().0]
        } else {
            vec![]
        };

        let pipeline = Pipeline::new(&canvas,
            layouts.as_slice(),
            Some("Pipeline Rectangle"),
            VertexEntry::new(&shader.shader_mod, "vs_main", &[Vertex::desc()]),
            FragmentEntry::new(&shader.shader_mod, "fs_main"),
        );

        Self { vertices, vertex_buffer: vertex, index_buffer: index, pipeline, texture }
    }
}

impl<'a> Shape<'a> for Rectangle<'a> {
    fn draw<'b>(&'a self, render_pass: &'b mut RenderPass<'a>) where 'a: 'b {
        render_pass.set_pipeline(&self.pipeline);

        if self.texture.is_some() {
            render_pass.set_bind_group(0, &self.texture.as_ref().unwrap().1, &[]);
        }

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), Uint16);
        render_pass.draw_indexed(0..6, 0,0..1);
    }
}

pub struct Triangle<'a> {
    vertices: &'a [Vertex],
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pipeline: RenderPipeline,
    texture: Option<(BindGroupLayout,BindGroup)>,
}

impl<'a> Triangle<'a> {
    const INDICES_TRIANGLE: &'a [u16] = &[0, 1, 2];

    pub fn new(shader: &Shader, vertices: &'a [Vertex], texture: Option<(BindGroupLayout, BindGroup)>, canvas: &Canvas) -> Self {
        let vertex = BufferBuilder::new(vertices, BufferUsages::VERTEX, Some("Vertex"), canvas);
        let index = BufferBuilder::new(Triangle::INDICES_TRIANGLE, BufferUsages::INDEX, Some("Index"), canvas);

        let layouts = if texture.is_some() {
            vec![&texture.as_ref().unwrap().0]
        } else {
            vec![]
        };

        let pipeline = Pipeline::new(&canvas,
                                     layouts.as_slice(),
                                     Some("Pipeline Triangle"),
                                     VertexEntry::new(&shader.shader_mod, "vs_main", &[Vertex::desc()]),
                                     FragmentEntry::new(&shader.shader_mod, "fs_main"),
        );

        Self { vertices: vertices, vertex_buffer: vertex, index_buffer: index, pipeline, texture }
    }
}

impl<'a> Shape<'a> for Triangle<'a> {
    fn draw<'b>(&'a self, render_pass: &'b mut RenderPass<'a>) where 'a: 'b {
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), Uint16);
        render_pass.draw_indexed(0..3, 0,0..1);
    }
}

pub struct Polygon<'a> {
    vertices: &'a [Vertex],
    indices: &'a [u16],
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pipeline: RenderPipeline,
    num_indices: u32,
    texture: Option<(BindGroupLayout,BindGroup)>,
}

impl<'a> Polygon<'a> {
    pub fn new(shader: &Shader, vertices: &'a [Vertex], indices: &'a [u16], texture: Option<(BindGroupLayout, BindGroup)>, canvas: &Canvas) -> Self {
        let vertex = BufferBuilder::new(vertices, BufferUsages::VERTEX, Some("Vertex"), canvas);
        let index = BufferBuilder::new(indices, BufferUsages::INDEX, Some("Index"), canvas);

        let layouts = if texture.is_some() {
            vec![&texture.as_ref().unwrap().0]
        } else {
            vec![]
        };

        let pipeline = Pipeline::new(&canvas,
                                     layouts.as_slice(),
                                     Some("Pipeline Polygon"),
                                     VertexEntry::new(&shader.shader_mod, "vs_main", &[Vertex::desc()]),
                                     FragmentEntry::new(&shader.shader_mod, "fs_main"),
        );

        Self { vertices, indices, vertex_buffer: vertex, index_buffer: index, pipeline, num_indices: indices.len() as u32, texture }
    }
}

impl<'a> Shape<'a> for Polygon<'a> {

    fn draw<'b>(&'a self, render_pass: &'b mut RenderPass<'a>) where 'a: 'b {
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0,0..1);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) color_or_coord: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ],
        }
    }
}