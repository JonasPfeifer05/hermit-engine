mod window;
mod camera;
mod rendering;
mod util;
mod shape;

use std::fs::remove_dir;
use std::iter;
use cgmath::{InnerSpace, Rotation3, Zero};
use wgpu::{BindGroup, BindGroupLayout, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, Color, ShaderStages, VertexBufferLayout};
use wgpu::BindingResource::{Sampler, TextureView};
use wgpu::IndexFormat::Uint16;

use winit::{
    event::*,
    event_loop::ControlFlow,
    window::Window,
};
use winit::dpi::PhysicalSize;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use rendering::bind_group::{BindGroupBuilder, GroupEntry, LayoutEntry};
use camera::camera_controller::CameraController;
use crate::camera::camera::{Camera, CameraUniform};
use crate::rendering::bind_group;
use crate::rendering::buffer::BufferBuilder;
use crate::rendering::canvas::Canvas;
use crate::rendering::instance::{Instance, InstanceRaw, NUM_INSTANCES_PER_ROW};
use crate::rendering::model::{DrawModel, Material, Mesh, Model, ModelVertex};
use crate::rendering::pipeline::Pipeline;
use crate::rendering::shader::{FragmentEntry, Shader, VertexEntry};
use crate::shape::shape_drawer::{Vertex, Polygon, Rectangle, Shape, ShapeDrawer, Triangle, ShapeData};
use crate::util::resources;
use crate::util::textures::Texture;
use crate::window::{HermitWindow, WindowData};

struct Engine<'a> {
    canvas: Canvas,

    polygon: Polygon<'a>,
    rectangle: Rectangle<'a>,
    triangle: Triangle<'a>,

    /*
    /*
     */
    square: Square,

    camera: Camera,
    camera_controller: CameraController,

    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,

    depth_texture: Texture,

    obj_model: Model,
     */
}

impl<'a> Engine<'a> {
    async fn new(window: &Window) -> Engine<'a> {
        let canvas = Canvas::new(window).await;

        let shader = Shader::new("shaders/color.wgsl", &canvas).await;
        let shader2 = Shader::new("shaders/texture.wgsl", &canvas).await;

        let diffuse_bytes = include_bytes!("../res/cube-diffuse.jpg");
        let diffuse_texture = Texture::from_bytes(&canvas.device, &canvas.queue, diffuse_bytes, "cube-diffuse").unwrap();
        let (dbgl, diffuse_bind_group) = BindGroupBuilder::new(&canvas,
            &[
                    LayoutEntry::new(0, ShaderStages::FRAGMENT, BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    }),
                    LayoutEntry::new(1, ShaderStages::FRAGMENT, BindingType::Sampler(wgpu::SamplerBindingType::Filtering)),
                ],
            &[
                GroupEntry::new_binding_resource(0, TextureView(&diffuse_texture.view)),
                GroupEntry::new_binding_resource(1, Sampler(&diffuse_texture.sampler)),
            ],
            None,
            true
        );
        let dbg = diffuse_bind_group.unwrap();

        let polygon = Polygon::new(&shader, VERTICES, INDICES, None, &canvas);
        let rectangle = Rectangle::new(&shader2, VERTICES2, Some((dbgl,dbg)), &canvas);
        let triangle = Triangle::new(&shader,VERTICES3, None, &canvas);

        /*
        let shader = Shader::new("shaders/shader.wgsl", &canvas).await;


        let vertex_buffer = BufferBuilder::new(VERTICES, BufferUsages::VERTEX, Some("Vertex Shader"), &canvas);
        let index_buffer = BufferBuilder::new(INDICES, BufferUsages::INDEX, Some("Index Buffer"), &canvas);
        let num_indices = INDICES.len() as u32;


        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: canvas.config.width as f32 / canvas.config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);
        let camera_buffer = BufferBuilder::new(&[camera_uniform], BufferUsages::UNIFORM | BufferUsages::COPY_DST, Some("Camera Buffer"), &canvas);

        let (camera_bind_group_layout, camera_bind_group) = BindGroupBuilder::new(
            &canvas,
            &[LayoutEntry::new(0, ShaderStages::VERTEX, BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            })],
            &[GroupEntry::new(0, &camera_buffer)],
            Some("Camera bind group"),
            true,
        );
        let camera_bind_group = camera_bind_group.unwrap();

        let camera_controller = CameraController::new(0.2);

        const SPACE_BETWEEN: f32 = 3.0;
        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance {
                    position,
                    rotation,
                }
            })
        }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = BufferBuilder::new(&instance_data, BufferUsages::VERTEX, Some("Instance Buffer"), &canvas);

        let depth_texture = Texture::create_depth_texture(&canvas.device, &canvas.config, "depth_texture");


        let (texture_bind_group_layout, _) = BindGroupBuilder::new(
            &canvas,
            &[
                LayoutEntry::new(0, ShaderStages::FRAGMENT, BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                }),
                LayoutEntry::new(1, ShaderStages::FRAGMENT, BindingType::Sampler(wgpu::SamplerBindingType::Filtering)),
            ],
            &[],
            Some("Texture Bind Group"),
            false,
        );

        let obj_model = resources::load_model(
            "cube.obj",
            &canvas.device,
            &canvas.queue,
            &texture_bind_group_layout,
        ).await.unwrap();

        let render_pipeline = Pipeline::new(
            &canvas,
            &[&texture_bind_group_layout, &camera_bind_group_layout],
            Some("Render Pipeline"),
            VertexEntry::new(&shader.shader_mod, "vs_main", &[ModelVertex::desc(), InstanceRaw::desc()]),
            FragmentEntry::new(&shader.shader_mod, "fs_main"),
        );

        let square = Square::new(&canvas, &camera_bind_group_layout).await;
         */

        Self {
            canvas,
            polygon,
            rectangle,
            triangle,
            /*
            /*
             */

            square,
            camera,
            camera_controller,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            instances,
            instance_buffer,
            depth_texture,
            obj_model,
             */
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.canvas.size = new_size;
            self.canvas.config.width = new_size.width;
            self.canvas.config.height = new_size.height;
            self.canvas.surface.configure(&self.canvas.device, &self.canvas.config);
        }
       // self.depth_texture = Texture::create_depth_texture(&self.canvas.device, &self.canvas.config, "depth_texture");
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        //self.camera_controller.process_events(event)
        false
    }

    fn update(&mut self) {
        //self.camera_controller.update_camera(&mut self.camera);
        //self.camera_uniform.update_view_proj(&self.camera);
        //self.canvas.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.canvas.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .canvas.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        },
                    })
                ],
                depth_stencil_attachment: /*Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                })*/ None,
            });

            //render_pass.set_pipeline(&self.render_pipeline);

            //render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            //render_pass.set_index_buffer(self.index_buffer.slice(..), Uint16);
            //render_pass.draw_indexed(0..self.num_indices, 0,0..1);

            let mut drawer = ShapeDrawer::new(&mut render_pass);
            drawer.draw_shape(&self.polygon);
            drawer.draw_shape(&self.rectangle);
            drawer.draw_shape(&self.triangle);

            // NEW!
            /*
            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // UPDATED!
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
             */
            /*render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            render_pass.set_pipeline(&self.render_pipeline);

            let mesh = &self.obj_model.meshes[0];
            let material = &self.obj_model.materials[mesh.material];


            render_pass.draw_mesh_instanced(mesh, material, 0..self.instances.len() as u32, &self.camera_bind_group);
             */
        }

        self.canvas.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let (event_loop, window) = HermitWindow::new(WindowData::new(true, "HERMIT ENGINE".to_string(), PhysicalSize::new(800, 800))).await;

    // State::new uses async code, so we're going to wait for it to finish
    let mut engine = Engine::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !engine.input(event) {
                    // UPDATED!
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            engine.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            engine.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                engine.update();
                match engine.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => engine.resize(engine.canvas.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color_or_coord: [1.0, 0.0, 0.0] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color_or_coord: [0.0, 1.0, 0.0] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color_or_coord: [1.0, 1.0, 0.0] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color_or_coord: [0.0, 0.0, 1.0] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color_or_coord: [1.0, 0.0, 1.0] }, // E
];

const VERTICES2: &[Vertex; 4] = &[
    Vertex { position: [-1.0,1.0,0.0], color_or_coord: [0.0,0.0,0.0] },
    Vertex { position: [0.0,1.0,0.0], color_or_coord: [1.0,0.0,0.0] },
    Vertex { position: [0.0,-0.0,0.0], color_or_coord: [1.0,1.0,0.0] },
    Vertex { position: [-1.0,0.0,0.0], color_or_coord: [0.0,1.0,0.0] },
];

const VERTICES3: &[Vertex; 3] = &[
    Vertex { position: [0.5,0.0,0.0], color_or_coord: [1.0,0.0,0.0] },
    Vertex { position: [0.0,-1.0,0.0], color_or_coord: [1.0,0.0,0.0] },
    Vertex { position: [1.0,-1.0,0.0], color_or_coord: [1.0,0.0,0.0] },
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

const INDICES2: &[u16; 6] = &[
    0, 2, 1,
    0, 3, 2,
];
