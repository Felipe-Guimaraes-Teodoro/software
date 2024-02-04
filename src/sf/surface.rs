//Mesh that covers the entirety of the screen
//Implement like polygon.rs

use crate::sf::*;
use gl::*;
use cgmath::*;

use crate::cstr;

pub const SURFACE_VS: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;

    uniform mat4 view;
    uniform mat4 proj;
    uniform float w;
    // uniform mat4 model;

    out vec3 pos;

    void main() {
        gl_Position = proj * view * vec4(aPos * w, 1.0);

        pos = aPos;
        // gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

pub const SURFACE_FS: &str = r#"
    #version 330 core
    out vec4 FragColor;

    uniform vec3 cpos;
    uniform float zoom;

    in vec3 pos;

    vec3 hash13(float v) {
        float x;
        float y;
        float z;

        x = fract(sin(v) * 1.0);
        y = fract(sin(v + x) * 29.0);
        z = fract(sin(v + y) * 29.0);

        return vec3(x, y, z);
    }

    void main() {
        vec2 norm_coords = pos.xy * vec2(0.5);
        vec2 c = 1.0 / pow(2.0, zoom) * norm_coords - cpos.xy;

        vec2 z = vec2(0.0, 0.0);
        float i;
        for (i = 0.0; i < 1.0; i += 0.005) {
            z = vec2(
                z.x * z.x - z.y * z.y + c.x,
                z.y * z.x + z.x * z.y + c.y
            );
            if (length(z) > 4.0) {
                break;
            }
        }

        FragColor = vec4(hash13(i), 1.0);
    }
"#;

pub struct Surface {
    buf: RVertexBufferIndexed,
    shader: Shader,
    pos: Vector3<f32>,
    zoom: f32,
}

impl Surface {
    pub fn new() -> Self {
        let verts = vec![
            1.0, 1.0, 0.0, // top right
            1.0, -1.0, 0.0, // bottom right
            -1.0, -1.0, 0.0, // bottom left 
            -1.0, 1.0, 0.0, // top left 
        ];

        let indices = vec![
            0, 1, 3, //1st 
            1, 2, 3,  //2nd
        ];

        let buf = RVertexBufferIndexed::new((&verts, &indices));

        let shader = Shader::new_pipeline(SURFACE_VS, SURFACE_FS);

        Self {
            buf,
            shader,
            pos: vec3(0.0, 0.0, 0.0),
            zoom: 1.0,
        }
    }

    pub unsafe fn draw(&self, camera: &Camera, w: f32, h: f32) {
        self.shader.use_shader();
        camera.send_uniforms(&self.shader);
        self.shader.uniform_vec3f(cstr!("cpos"), &self.pos);
        self.shader.uniform_1f(cstr!("zoom"), self.zoom);
        self.shader.uniform_1f(cstr!("w"), w / h);
        BindVertexArray(self.buf.vao_id);
        DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());
        BindVertexArray(0);
    }

    pub fn update(&mut self, window: &glfw::Window) {
        let SPEED: f32 = 0.01 * 1.0 / f32::powf(2.0, self.zoom.clone());
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            self.pos.y -= 1.0 * SPEED; 
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            self.pos.x += 1.0 * SPEED;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            self.pos.y += 1.0 * SPEED; 
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            self.pos.x -= 1.0 * SPEED; 
        }
    }

    pub fn scroll(&mut self, val: f32) {
        self.zoom += val / 5.0;
    }
}
