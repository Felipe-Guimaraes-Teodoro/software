use crate::{environment::*, sf::Renderer};

use cgmath::Vector3;
use std::sync::{Arc, RwLock};

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
}

// implement world draw method
impl Renderer {
    pub unsafe fn draw_world(world: Arc<RwLock<World>>) {
        
        for mirror in 0..world.mirrors.len() {
            if let Some(mirror) = world.mirrors[mirror] {
                mirror.draw(&self.mirror_shader);
            }
        }
        
    }
}
