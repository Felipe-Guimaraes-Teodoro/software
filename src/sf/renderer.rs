use crate::sf::*;
use crate::environment::*;

use std::sync::{Arc, RwLock};

pub struct Renderer {
    pub camera: Camera,

    line_shader: Shader,
    pub mirror_shader: Shader, 

    pub polygon_ammount: usize,
    pub polygons: [Option<Polygon>; 2000], // currently not actually immediate lol

    pub line_ammount: usize,
    pub lines: [Option<Line>; 2000],

    pub world_handle: Arc<RwLock<World>>,
}

impl Renderer {
    pub fn new(world_handle: Arc<RwLock<World>>) -> Renderer {
        let camera = Camera::new();
        let line_shader = Shader::new_pipeline(POLYGON_VS, POLYGON_FS);
        let mirror_shader = Shader::new_pipeline(MIRROR_VS, MIRROR_FS);

        Self {
            camera,

            line_shader,
            mirror_shader,

            polygon_ammount: 0,
            line_ammount: 0,
            polygons: [None; 2000],
            lines: [None; 2000],

            world_handle,
        }
    }

    pub unsafe fn draw(&mut self) {
        // draw polygons
        self.camera.send_uniforms(&self.line_shader);

        for polygon in 0..self.polygon_ammount {
            if let Some(polygon) = self.polygons[polygon] {
                polygon.draw(&self.line_shader);
            }
        }

        // draw world 
        Self::draw_world(self.world_handle); // impl @ src/environment/world.rs 
    }
}
