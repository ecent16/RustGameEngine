/*
    Description: Main entry point for the Rust Game Engine application. This file sets up the application 
    state and handles window events using the winit crate. The AppState struct manages the window and its 
    properties, while the ApplicationHandler trait implementation defines how to respond to various events 
    such as window creation, resizing, and keyboard input.
*/

use winit::{
    event_loop::{ControlFlow, EventLoop}
};

use engine_window::{AppState}; 

fn main() {

    env_logger::init(); // Initialize the logger for debugging purposes
    let event_loop = EventLoop::new().expect("Failed to create event loop"); // Create a new event loop
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = AppState {
        title: "Rust Game Engine".to_string(),
        width: 1280,
        height: 720,
        resizable: true,
        ..Default::default()
    };

    event_loop.run_app(&mut app).expect("Failed to run application"); // Run the application with the event loop and app state
}