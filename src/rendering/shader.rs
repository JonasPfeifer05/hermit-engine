use wgpu::{ShaderModule, VertexBufferLayout, VertexState};
use crate::canvas::Canvas;
use crate::resources::load_string;

pub struct VertexEntry {}
impl VertexEntry {
    pub fn new<'a>(shader_mod: &'a ShaderModule, entry_point: &'a str, desc: &'a [VertexBufferLayout]) -> VertexState<'a> {
        let vertex = VertexState {
            module: shader_mod,
            entry_point, // 1.
            buffers: desc, // 2.
        };

        vertex
    }
}

pub struct FragmentEntry<'a> {
    pub shader_mod: &'a ShaderModule,
    pub entry_point: &'a str
}
impl<'a> FragmentEntry<'a> {
    pub fn new(shader_mod: &'a ShaderModule, entry_point: &'a str) -> FragmentEntry<'a> {
        Self {
            shader_mod,
            entry_point
        }
    }
}

pub struct Shader {
    pub shader_mod: ShaderModule,
}

impl Shader {
    pub async fn new(path: &str, canvas: &Canvas) -> Self {
        let mut error = String::from("Cannot open: ");
        error.push_str(path);
        let shader_text = load_string(path).await.expect(error.as_str());

        let shader = canvas.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(path),
            source: wgpu::ShaderSource::Wgsl(shader_text.as_str().into()),
        });

        Self { shader_mod: shader }
    }
}