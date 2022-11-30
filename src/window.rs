use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use crate::Engine;

pub struct WindowData {
    pub resizeable: bool,
    pub title: String,
    pub size: PhysicalSize<i32>,
}

impl WindowData {
    pub fn new(resizeable: bool, title: String, size: PhysicalSize<i32>) -> Self {
        Self { resizeable, title, size }
    }
}

pub struct HermitWindow {
}

impl HermitWindow {
    pub async fn new(data: WindowData) -> (EventLoop<()>, winit::window::Window) {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            } else {
                env_logger::init();
            }
        }

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_resizable(data.resizeable)
            .with_inner_size(data.size)
            .with_title(data.title)
            .build(&event_loop)
            .unwrap();

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::PhysicalSize;
            window.set_inner_size(data.size);

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let _state = Engine::new(&window).await;

        (event_loop, window)
    }
}