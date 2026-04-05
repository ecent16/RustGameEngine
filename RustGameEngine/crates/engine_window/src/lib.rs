/*
    Description: Main entry point for the Rust Game Engine application. This file sets up the application 
    state and handles window events using the winit crate. The AppState struct manages the window and its 
    properties, while the ApplicationHandler trait implementation defines how to respond to various events 
    such as window creation, resizing, and keyboard input.
*/

// Import necessary modules from the winit crate
use winit::{
    application::ApplicationHandler, 
    event::{WindowEvent, MouseScrollDelta}, 
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, 
    keyboard::NamedKey, window::{Window, WindowAttributes, WindowId}
};

mod input; // Import the input module which defines the InputState struct and its associated methods
pub use input::InputState; // Make InputState available for external use (main.rs file will use this to manage input state)

// App State
#[derive(Default)]
pub struct AppState {
    pub window: Option<Window>, 
    pub input_state: input::InputState,
    pub title: String,
    pub height: u32,
    pub width: u32,
    pub resizable: bool,
}

// Implement the ApplicationHandler trait for AppState to handle application events
impl ApplicationHandler for AppState {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let window_attr = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
            .with_resizable(self.resizable);

        match event_loop.create_window(window_attr) {
            Ok(window) => {
                // Open window successfully created, store it in the app state
                self.window = Some(window);
            }
            Err(e) => {
                // Failed to create window, log the error and exit the application
                eprintln!("Failed to create window: {:?}", e);
                event_loop.exit();
            }
        }
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
                self.input_state.start_frame(); // Reset input state at the start of each frame
                if let Some(window) = &self.window {
                    window.request_redraw();
                    println!("Redraw requested for window: {:?}", window_id);   
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