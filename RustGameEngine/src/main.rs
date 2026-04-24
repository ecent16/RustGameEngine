/*
    Description: Main entry point for the Rust Game Engine application. This file sets up the application 
    state and handles window events using the winit crate. The AppState struct manages the window and its 
    properties, while the ApplicationHandler trait implementation defines how to respond to various events 
    such as window creation, resizing, and keyboard input.
*/

use winit::{
    event_loop::{ControlFlow, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey}
};
use engine_window::{AppState, InputState}; 

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

/* 
fn update_player(input: &InputState, player: &mut Player) {
    // This is an example of character movement logic based on keyboard input. In a real game, you would likely have more complex logic for handling player movement, including physics and collision detection.
    if input.is_key_held(PhysicalKey::Code(KeyCode::KeyW)) {
        player.move_forward();
    }
    if input.is_key_held(PhysicalKey::Code(KeyCode::KeyS)) {
        player.move_backward();
    }
    if input.is_key_held(PhysicalKey::Code(KeyCode::KeyA)) {
        player.move_left();
    }
    if input.is_key_held(PhysicalKey::Code(KeyCode::KeyD)) {
        player.move_right();
    }
}*/