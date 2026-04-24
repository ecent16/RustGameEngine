/*
    Description: Main entry point for the Rust Game Engine application. This file sets up the application 
    state and handles window events using the winit crate. The AppState struct manages the window and its 
    properties, while the ApplicationHandler trait implementation defines how to respond to various events 
    such as window creation, resizing, and keyboard input.
*/

// Import necessary modules from the winit crate
use std::sync::Arc;
use winit::{
    application::ApplicationHandler, 
    event::{MouseScrollDelta, WindowEvent}, 
    event_loop::ActiveEventLoop, 
    keyboard::NamedKey, window::{self, Window, WindowAttributes, WindowId}
};
use wgpu::*;

use engine_renderer::{Renderer};
use engine_renderer::pipeline::create_render_pipeline;

mod input; // Import the input module which defines the InputState struct and its associated methods
pub use input::InputState; // Make InputState available for external use (main.rs file will use this to manage input state)

// App State
#[derive(Default)]
pub struct AppState {
    pub window: Option<Arc<Window>>,
    pub renderer: Option<Renderer>,
    pub pipeline: Option<wgpu::RenderPipeline>,
    pub input_state: input::InputState,
    pub title: String,
    pub height: u32,
    pub width: u32,
    pub resizable: bool,
}

// Implement the ApplicationHandler trait for AppState to handle application events
impl ApplicationHandler for AppState {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let window_attr = Arc::new(event_loop.create_window(
            WindowAttributes::default().with_title("Rust Engine")
        ).unwrap());

        // Render assets
        let renderer = pollster::block_on(Renderer::new(window_attr.clone()));
        let pipeline = create_render_pipeline(
            &renderer.device, 
            renderer.surface_config.format, 
        );

        self.renderer = Some(renderer); 
        self.pipeline = Some(pipeline); 
        self.window = Some(window_attr);
    }

    fn window_event(
        &mut self, 
        event_loop: &ActiveEventLoop, 
        window_id: WindowId,
        event: WindowEvent, 
    ) {
        
        match event {
            // Handle window close event
            WindowEvent::CloseRequested => {
                // Handle window close event, exit the application
                println!("Window close requested, exiting...");
                event_loop.exit();
            }

            // Handle window resize event
            WindowEvent::Resized(physical_size) => {
                // Handle window resize event
                println!("Window resized to: {:?}", physical_size);
                self.width = physical_size.width;
                self.height = physical_size.height;
            }

            // Handle redraw request event
            WindowEvent::RedrawRequested => {
                // Handle redraw request, trigger rendering logic here
                let renderer = self.renderer.as_ref().unwrap();
                let pipeline = self.pipeline.as_ref().unwrap();

                match renderer.render(pipeline) {
                    Ok(_) => {} 

                    Err(wgpu::SurfaceError::Lost) => {
                        renderer.resize_window(renderer.win_size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(), 
                    Err(e) => panic!("Render error {}", e),
                }
            }

            // Handle keyboard input events
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                // Handle keyboard input events
                println!("Keyboard input event: device_id={:?}, event={:?}, is_synthetic={}", device_id, event, is_synthetic);
                self.input_state.handle_key(&event);
                if event.logical_key == NamedKey::Escape {
                    event_loop.exit();
                }   
            }

            WindowEvent::CursorMoved { device_id, position } => {   
                self.input_state.handle_cursor_moved((position.x, position.y));
            }

            WindowEvent::MouseInput {state, button, device_id} => {
                self.input_state.handle_mouse_button(button, state);
            }

            WindowEvent::MouseWheel { device_id, delta, phase, .. } => {
                
                let scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x * 10.0, y * 10.0), // Scale line delta for better sensitivity
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                self.input_state.scroll_delta = scroll_delta;
            }

            // Ignore other events for now
            _ => {}
        }
    }
}