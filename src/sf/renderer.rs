use crate::sf::*;

pub struct Renderer {
    pub camera: Camera,

    line_shader: Shader,
    pub polygon_ammount: usize,
    pub polygons: [Option<Polygon>; 2000], // currently not actually immediate lol

    pub line_ammount: usize,
    pub lines: [Option<Line>; 2000],
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
