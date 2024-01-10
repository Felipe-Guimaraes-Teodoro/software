use crate::{environment::*, sf::Renderer};
use crate::sf::Drawable;

use cgmath::Vector3;
use std::sync::{Arc, RwLock};

enum State {
    PlacingMirror,
    None,
}

pub struct World {
    pub mirrors: Vec<Mirror>,
    mirror_ammount: usize,
    state: State
}

impl World {
    pub fn new() -> Self {
        Self {
            mirrors: vec![],
            mirror_ammount: 0,
            state: State::None,
        }
    }

    pub fn push_mirror(&mut self, pos: Vector3<f32>, angle: f32) {
        self.mirrors.push(Mirror::new(pos, angle));
    }

    pub fn io(&mut self, glfw: &mut glfw::Glfw, window: &mut glfw::Window) {
        if window.get_key(glfw::Key::Num1) == glfw::Action::Press {
            self.state = State::PlacingMirror;
        } else if window.get_key(glfw::Key::Num2) == glfw::Action::Press {
            self.state = State::None;
        }

        match self.state {
            State::PlacingMirror => {
                if window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Press {
                    self.state = State::None;
                    dbg!("mirror has been placed");
                }     
            },

            State::None => {},
        }
    }
}

// implement world draw method
impl Renderer {
    pub unsafe fn draw_world(&mut self, world: &World) {
        let mirrors = &world.mirrors;
       
        for mirror in mirrors {
            mirror.draw(&self.mirror_shader);
        }
    }
}
