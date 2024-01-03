use crate::{sf::*, cstr};
use gl::*;
use cgmath::*;

pub struct Renderer {
    pub camera: Camera,

    line_shader: Shader,
    polygon_ammount: usize,
    polygons: [Option<Polygon>; 2000], // currently not actually immediate lol

    line_ammount: usize,
    lines: [Option<Line>; 2000],
}

impl Renderer {
    pub fn new() -> Self {
        let camera = Camera::new();
        let line_shader = Shader::new_pipeline(POLYGON_VS, POLYGON_FS);

        Self {
            camera,

            line_shader,
            polygon_ammount: 0,
            line_ammount: 0,
            polygons: [None; 2000],
            lines: [None; 2000],
        }
    }

    pub fn add_polygon(&mut self, verts: &Vec<f32>, color: Vector3<f32>) {
        let poly = Polygon::new(verts, color);

        self.polygons[self.polygon_ammount] = Some(poly);
        self.polygon_ammount += 1;
    }

    pub fn update_polygon(&self, idx: usize, new_verts: &Vec<f32>) {
        if let Some(mut polygon) = self.polygons[idx] {
            polygon.update(new_verts);
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

    }
}
