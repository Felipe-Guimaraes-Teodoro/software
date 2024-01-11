use crate::sf::*;
use crate::environment::*;

pub trait Drawable {
    unsafe fn draw(&self, shader: &Shader);
}

pub struct Renderer {
    pub camera: Camera,

    polygon_shader: Shader,
    pub mirror_shader: Shader, 

    pub polygon_ammount: usize,
    pub polygons: [Option<Polygon>; 2000], // currently not actually immediate lol

    pub line_ammount: usize,
    pub lines: [Option<Line>; 2000],
}

impl Renderer {
    pub fn new() -> Renderer {
        let camera = Camera::new();
        let polygon_shader = Shader::new_pipeline(POLYGON_VS, POLYGON_FS);
        let mirror_shader = Shader::new_pipeline(MIRROR_VS, MIRROR_FS);

        Self {
            camera,

            polygon_shader,
            mirror_shader,

            polygon_ammount: 0,
            line_ammount: 0,
            polygons: [None; 2000],
            lines: [None; 2000],
        }
    }

    pub unsafe fn draw(&mut self) {
        // draw polygons
        self.camera.send_uniforms(&self.polygon_shader);

        for polygon in 0..self.polygon_ammount {
            if let Some(polygon) = self.polygons[polygon] {
                polygon.draw(&self.polygon_shader);
            }
        }
    }

    pub fn r_draw<D: Drawable>(obj: D, shader: &Shader) {
        unsafe {
            obj.draw(&shader);
        }
    }
}
