use crate::{environment::*, sf::Renderer};
use crate::sf::Drawable;
use crate::util::{SecondOrderDynamics};

use cgmath::*;

use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
enum State {
    PlacingMirror,
    HoldingMirror(usize), // argument is mirror vector index in world
    JustPlacedMirror,
    None,
}

pub struct World {
    pub mirrors: Vec<Mirror>,
    sod_controller: SecondOrderDynamics,
    state: State,
    mouse_angle: f32,

    width: f32,
    height: f32,
}

impl World {
    pub fn new() -> Self {
        let viewing_mirror = Mirror::new(&[-10.0, -10.0, 0.0].into(), 0.0);
        let sod_controller = SecondOrderDynamics::new(0.5, 0.5, 0.5, vec3(0.0, 0.0, 0.0));

        Self {
            mirrors: vec![viewing_mirror],
            sod_controller,
            state: State::None,
            mouse_angle: 0.0,

            width: 800.0,
            height: 800.0,
        }
    }

    pub fn push_mirror(&mut self, pos: Vector3<f32>, angle: f32) {
        self.mirrors.push(Mirror::new(&pos, angle));
    }

    pub fn debug_mirrors(&mut self, fdl: &imgui::DrawListMut) {
        for mirror in &self.mirrors {
            fdl.add_text([(mirror.pos.x * (self.width / 2.0)) - self.width, (mirror.pos.y * (self.height / 2.0)) - self.height], [1.0, 1.0, 1.0], "str");
        }
    }

    pub fn io(&mut self, _glfw: &mut glfw::Glfw, window: &mut glfw::Window) {
        if window.get_key(glfw::Key::Num1) == glfw::Action::Press && self.state == State::None{
            self.state = State::PlacingMirror;
        } else if window.get_key(glfw::Key::Num2) == glfw::Action::Press {
            self.state = State::None;
        }

        if window.get_key(glfw::Key::K) == glfw::Action::Press {
            self.mouse_angle += 0.1;
        } else if window.get_key(glfw::Key::J) == glfw::Action::Press {
            self.mouse_angle -= 0.1;
        }

        if window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Press && self.state == State::None {
            let x = window.get_cursor_pos().0 as f32;
            let y = window.get_cursor_pos().1 as f32;

            for i in 0..self.mirrors.len() {
                if self.mirrors[i].in_bounds(x, y, self.mirrors[i].pos, self.width, self.height) {
                    // self.sod_controller.set_starting_point(self.mirrors[i].pos);
                    self.mouse_angle = self.mirrors[i].angle;
                    self.state = State::HoldingMirror(i);
                    break;
                }
            }
        }

        match self.state {
            State::PlacingMirror => {
                let x = (((window.get_cursor_pos().0 as f32 * 2.0) - 1.0) / self.width) - 1.0;
                let y = (((-window.get_cursor_pos().1 as f32 * 2.0) - 1.0) / self.height) + 1.0;

                let yr = self.sod_controller.update(0.1, vec3(x, y, 0.0));

                self.mirrors[0].pos = yr; 
                self.mirrors[0].angle = self.mouse_angle;

                if window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Press {
                    self.state = State::JustPlacedMirror;
                    self.push_mirror([x, y, 0.0].into(), self.mouse_angle);
                    self.mirrors[0].pos = [-5.0, -5.0, 0.0].into(); 
                }     
            },
            State::JustPlacedMirror => { 
                if window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Release {
                    self.state = State::None;
                }
            }

            State::HoldingMirror(idx) => {
                let x = (((window.get_cursor_pos().0 as f32 * 2.0) - 1.0) / self.width) - 1.0;
                let y = (((-window.get_cursor_pos().1 as f32 * 2.0) - 1.0) / self.height) + 1.0;

                let yr = self.sod_controller.update(0.1, vec3(x, y, 0.0));

                self.mirrors[idx].pos = yr; 
                self.mirrors[idx].angle = self.mouse_angle;

                if window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Release {
                    self.state = State::None;
                }
            }

            State::None => {},
        }
    }

    pub fn scroll_wheel(&mut self, v: f32) {
        self.mouse_angle += v / (8.0 * 3.1415);
    } 

    pub fn set_framebuffer_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

// implement world draw method
impl Renderer {
    pub unsafe fn draw_world(&mut self, world: &World) {
        for mirror in &world.mirrors {
            mirror.draw(&self.mirror_shader, world.width, world.height);
        }
    }
}
