use crate::{sf::*, cstr};

use cgmath::*;
use gl::*;

pub const MIRROR_VS: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    // uniform mat4 view;
    // uniform mat4 proj;
    // uniform mat4 model;
    uniform vec3 pos;

    void main() {
        // gl_Position = proj * view * vec4(aPos, 1.0);
        gl_Position = vec4(aPos + pos, 1.0);
    }
"#;

pub const MIRROR_FS: &str = r#"
    #version 330 core
    out vec4 FragColor;

    uniform vec3 color;

    void main() {
       FragColor = vec4(color, 1.0f);
    }
"#;

#[derive(Clone, Copy, Debug)]
pub struct Mirror {
    buf: RVertexBufferIndexed,
    pos: Vector3<f32>,
    angle: f32,
}

impl Mirror {
    pub fn new(pos: Vector3<f32>, angle: f32,) -> Self {
        let verts = vec![
            0.25, 0.5, 0.0, // top right
            0.25, -0.5, 0.0, // bottom right
            -0.25, -0.5, 0.0, // bottom left 
            -0.25, 0.5, 0.0, // top left 
        ];

        let indices = vec![
            0, 1, 3, //1st 
            1, 2, 3,  //2nd
        ];

        let buf = RVertexBufferIndexed::new((&verts, &indices));

        Self {
            buf,
            pos,
            angle
        }
    }

    pub unsafe fn draw(&self, shader: &Shader) {
        shader.use_shader();
        shader.uniform_vec3f(cstr!("pos"), &self.pos);
        shader.uniform_vec3f(cstr!("color"), &vec3(0.51, 0.55, 0.8));
        BindVertexArray(self.buf.vao_id);
        DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());
        BindVertexArray(0);
    }
}

