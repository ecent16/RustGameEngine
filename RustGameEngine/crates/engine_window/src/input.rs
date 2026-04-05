/*
    Description: This module defines the input handling for the engine window, including keyboard and mouse e
    vents. It uses the winit crate to capture and process user input, allowing the application to respond 
    to various input events such as key presses, mouse movements, and scroll actions. The module maintains 
    a state of the current input status, which can be queried by other parts of the application to determine 
    how to react to user interactions.
*/
use std::collections::HashSet;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta};
use winit::keyboard::PhysicalKey;

#[derive(Default)]
pub struct InputState { 
    // Keyboard input state
    held_keys: HashSet<PhysicalKey>,
    pressed_keys: HashSet<PhysicalKey>,
    released_keys: HashSet<PhysicalKey>,

    // Mouse input state
    held_mouse_buttons: HashSet<MouseButton>,
    pressed_mouse_buttons: HashSet<MouseButton>,
    released_mouse_buttons: HashSet<MouseButton>,

    // Mouse position and scroll state
    pub cursor_position: (f64, f64),
    pub scroll_delta: (f32, f32),
    pub cursor_delta: (f64, f64), // Movement since last frame
}

impl InputState {

    pub fn start_frame(&mut self) {
        // Clear pressed and released states at the start of each frame
        self.pressed_keys.clear();
        self.released_keys.clear();
        self.pressed_mouse_buttons.clear();
        self.released_mouse_buttons.clear();
        self.cursor_delta = (0.0, 0.0); // Reset cursor movement delta
        self.scroll_delta = (0.0, 0.0); // Reset scroll delta
    }

    pub fn handle_key(&mut self, event: &KeyEvent) {
        if event.repeat {
           return; // Ignore repeated key events
        } 

        match event.state {
            ElementState::Pressed => {
                self.held_keys.insert(event.physical_key);
                self.pressed_keys.insert(event.physical_key);
            }
            ElementState::Released => {
                self.held_keys.remove(&event.physical_key);
                self.released_keys.insert(event.physical_key);
            }
        }
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.held_mouse_buttons.insert(button);
                self.pressed_mouse_buttons.insert(button);
            }
            ElementState::Released => {
                self.held_mouse_buttons.remove(&button);
                self.released_mouse_buttons.insert(button);
            }
        }
    }

    pub fn handle_cursor_moved(&mut self, position: (f64, f64)) {
        // Update cursor movement delta based on the new position x and y
        self.cursor_delta = (position.0 - self.cursor_position.0, position.1 - self.cursor_position.1);
        self.cursor_position = position;
    }

    pub fn handle_scroll(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                self.scroll_delta = (x, y);
            }
            MouseScrollDelta::PixelDelta(pos) => {
                self.scroll_delta = (pos.x as f32, pos.y as f32);
            }
        }
    }

    // Additional helper methods to query input state
    pub fn is_key_held(&self, key: PhysicalKey) -> bool {
        self.held_keys.contains(&key)
    }
    pub fn is_key_pressed(&self, key: PhysicalKey) -> bool {
        self.pressed_keys.contains(&key)
    }
    pub fn is_key_released(&self, key: PhysicalKey) -> bool {
        self.released_keys.contains(&key)   
    }
    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        self.held_mouse_buttons.contains(&button)
    }
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }   
    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        self.released_mouse_buttons.contains(&button)
    }
}


