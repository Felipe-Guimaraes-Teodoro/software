use crate::{environment::*, sf::Renderer};

use cgmath::Vector3;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct World {
    mirrors: [Option<Mirror>; 2000],
    mirror_ammount: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            mirrors: [None; 2000],
            mirror_ammount: 0,
        }
    }

    pub fn push_mirror(&mut self, pos: Vector3<f32>, angle: f32) {
        self.mirrors[self.mirror_ammount] = Some(Mirror::new(pos, angle));
        self.mirror_ammount += 1;
    }
}

// implement world draw method
impl Renderer {
    pub unsafe fn draw_world(&mut self) {
        let world_handle_arc = &self.world_handle;

        let mirrors = world_handle_arc.lock().unwrap().mirrors; 

        for mirror in 0..mirrors.len() {
            if let Some(mirror) = mirrors[mirror] {
                mirror.draw(&self.mirror_shader);
            }
        }
        
    }
}
