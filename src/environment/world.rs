use crate::{environment::*, sf::Renderer};
use crate::sf::Drawable;

use cgmath::Vector3;
use std::sync::{Arc, RwLock};

pub struct World {
    pub mirrors: Vec<Mirror>,
    mirror_ammount: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            mirrors: vec![],
            mirror_ammount: 0,
        }
    }

    pub fn push_mirror(&mut self, pos: Vector3<f32>, angle: f32) {
        self.mirrors.push(Mirror::new(pos, angle));
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
