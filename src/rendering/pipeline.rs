use wgpu::{BindGroupLayout, FragmentState, RenderPipeline, VertexState};
use crate::rendering::canvas::Canvas;
use crate::rendering::shader::FragmentEntry;
use crate::util::textures;

pub struct Pipeline {

}

impl Pipeline {
    pub fn new(canvas: &Canvas, group_layouts: &[&BindGroupLayout], label: Option<&str>, vertex: VertexState, fragment: FragmentEntry) -> RenderPipeline {
        let render_pipeline_layout =
            canvas.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: group_layouts,
                push_constant_ranges: &[],
            });

        let render_pipeline = canvas.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&render_pipeline_layout),
            vertex,
            fragment: Some(FragmentState {
                entry_point: fragment.entry_point,
                module: fragment.shader_mod,
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: canvas.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: /*Some(wgpu::DepthStencilState {
                format: textures::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(), // 2.
                bias: wgpu::DepthBiasState::default(),
            })*/ None,
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        render_pipeline
    }
}