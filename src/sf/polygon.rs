use crate::{sf::*, cstr};
use cgmath::*;
use gl::*;

pub const POLYGON_VS: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    uniform mat4 view;
    uniform mat4 proj;
    uniform mat4 model;

    void main() {
        gl_Position = proj * view * vec4(aPos, 1.0);
        // gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

pub const POLYGON_FS: &str = r#"
    #version 330 core
    out vec4 FragColor;

    uniform vec3 color;

    void main() {
       FragColor = vec4(color, 1.0f);
    }
"#;

#[derive(Copy, Clone)]
pub struct Polygon {
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

    pub fn push(v: &[f32; 3]) { // pushes a vertex to the tip of the polygon
        dbg!(v);
        panic!("Polygon::push() is not yet implemented: Polygon must own it's vertices");
    }

    pub fn update(&mut self, new_verts: &Vec<f32>) {
        self.buf.update(new_verts);
    }
}
