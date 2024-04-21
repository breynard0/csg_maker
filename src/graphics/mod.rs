pub mod cam;
pub mod depth;
pub mod init;
pub mod input;
pub mod msaa;
pub mod render;
pub mod texture;
pub mod uniform;
pub mod vertex;
pub mod rendering_object;

use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};

use crate::{object::world_object::WorldObject, utils::log::log, AppData};
pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let window = WindowBuilder::new()
        .with_title("CSG Maker")
        .with_decorations(true)
        .build(&event_loop)
        .unwrap();
    let window_id = window.id();

    let mut app_data = AppData {
        rendering_object: init::gfx_init(&window).await,
        world_object: WorldObject::new(),
    };

    let mut last_frame = std::time::Instant::now();

    match event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            ref event,
            window_id: queried_window_id,
        } if queried_window_id == window_id => match event {
            WindowEvent::RedrawRequested if window_id == app_data.rendering_object.window().id() => {
                app_data.rendering_object.update();
                match render::render(&mut app_data) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => app_data.rendering_object.resize(app_data.rendering_object.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log(
                            "Not enough memory to map the next frame!",
                            crate::utils::log::LogLevel::ERROR,
                        );
                    }
                    Err(e) => log(
                        format!("{}", e.to_string()),
                        crate::utils::log::LogLevel::ERROR,
                    ),
                }
            }
            WindowEvent::KeyboardInput { event, .. } => input::poll_keyboard_event(event),
            WindowEvent::MouseInput { button, state, .. } => {
                input::poll_mousebutton_event(button, state)
            }
            WindowEvent::MouseWheel { delta, .. } => input::poll_scroll_wheel(delta),
            WindowEvent::Resized(physical_size) => {
                app_data.rendering_object.resize(*physical_size);
            }
            WindowEvent::CursorMoved { position, .. } => input::poll_mouse_move_event(position),
            WindowEvent::CloseRequested => elwt.exit(),
            _ => {}
        },
        Event::AboutToWait => {
            app_data.rendering_object.delta_time = std::time::Instant::now()
                .duration_since(last_frame)
                .as_secs_f32();
            last_frame = std::time::Instant::now();
            app_data.rendering_object.window().request_redraw();
        }
        _ => {}
    }) {
        Ok(_) => {}
        Err(e) => log(e.to_string(), crate::utils::log::LogLevel::FATAL),
    }
}
