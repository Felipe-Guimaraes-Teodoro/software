use crate::{environment::*, sf::Renderer};

use cgmath::Vector3;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct World {
    pub mirrors: [Option<Mirror>; 2000],
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

    pub fn set_mirror_angle(&mut self, angle: f32, index: usize) {
        self.mirrors[index].unwrap().angle = angle;
    }
}

// implement world draw method
impl Renderer {
    pub unsafe fn draw_world(&mut self) {
        let world_handle_arc = Arc::clone(&self.world_handle);

        let mirrors = world_handle_arc.lock().unwrap().mirrors; 

        for mirror in 0..mirrors.len() {
            if let Some(mut mirror) = mirrors[mirror] {
                mirror.draw(&self.mirror_shader);
            }
        }
        
    }
}
