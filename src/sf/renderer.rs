use crate::{sf::*, cstr};
use gl::*;
use cgmath::*;

pub struct Renderer {
    pub camera: Camera,

    line_shader: Shader,
    polygon_ammount: usize,
    immediate_polygons: [Option<Polygon>; 2000],
}

impl Renderer {
    pub fn new() -> Self {
        let camera = Camera::new();
        let line_shader = Shader::new_pipeline(POLYGON_VS, POLYGON_FS);

        Self {
            camera,

            line_shader,
            polygon_ammount: 0,
            immediate_polygons: [None; 2000],
        }
    }

    pub fn add_polygon(&mut self, verts: &Vec<f32>, color: Vector3<f32>) {
        let poly = Polygon::new(verts, color);

        self.immediate_polygons[self.polygon_ammount] = Some(poly);
        self.polygon_ammount += 1;
    }

    pub unsafe fn draw(&mut self) {
        // draw polygons
        for i in 0..=self.polygon_ammount {
            if self.immediate_polygons[i].is_some() {
                self.camera.send_uniforms(&self.line_shader);
                self.immediate_polygons[i].unwrap().draw(&self.line_shader);
            }
        } 
    }
}

const POLYGON_VS: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    // uniform mat4 view;
    // uniform mat4 proj;
    // uniform mat4 model;

    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const POLYGON_FS: &str = r#"
    #version 330 core
    out vec4 FragColor;

    uniform vec3 color;

    void main() {
       FragColor = vec4(color, 1.0f);
    }
"#;

#[derive(Copy, Clone)]
struct Polygon {
    buf: RVertexBuffer,
    pub col: Vector3<f32>,
    len: i32,
}

impl Polygon {
    pub fn new(verts: &Vec<f32>, col: Vector3<f32>) -> Self {
        let buf = RVertexBuffer::new(&verts);
        
        Self {
            buf,
            col,
            len: verts.len() as i32,
        } 
    }

    pub unsafe fn draw(&mut self, shader: &Shader) {
        shader.use_shader();
        shader.uniform_vec3f(cstr!("color"), &self.col);
        BindVertexArray(self.buf.vao_id);
        DrawArrays(TRIANGLES, 0, self.len);
        BindVertexArray(0);
    }

    // pub fn push() // adds a vertex to the polygon 
}
