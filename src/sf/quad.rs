use gl::*;
use crate::sf::RVertexBufferTextured;
use crate::sf::Buffer;
use crate::sf::Drawable;
use crate::sf::Camera;
use crate::cstr;

use cgmath::*;

pub const T_QUAD_VS: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    layout (location = 2) in vec2 aTexCoord;

    uniform vec3 pos;
    uniform float scale;

    uniform mat4 view;
    uniform mat4 proj;

    uniform float width;
    uniform float height;

    out vec3 Color;
    out vec2 TexCoord;

    void main() {
        vec3 corrected_pos = vec3(pos.x * (width / height), pos.y, pos.z);
        gl_Position = proj * view * vec4(aPos * scale + corrected_pos, 1.0);

        Color = aColor;
        TexCoord = vec2(aTexCoord.x, aTexCoord.y);
    }
"#; 

pub const T_QUAD_FS: &str = r#"
    #version 330 core
    out vec4 FragColor;

    in vec3 Color;
    in vec2 TexCoord;

    uniform sampler2D texture1;

    void main() {
        vec4 col = texture(texture1, TexCoord);

        float whiteThreshold = 1.6;
        float transition = smoothstep(
            whiteThreshold - 0.25, 
            whiteThreshold + 0.55, 
            dot(col.rgb, vec3(1.0))
        );

        FragColor = mix(col, vec4(col.rgb, 0.0), transition);
    }
"#; 

#[derive(Copy, Clone, Debug)]
pub struct TexturedQuad {
    buf: RVertexBufferTextured,
    pub pos: Vector3<f32>,
    pub aspect_x: f32,
    pub aspect_y: f32,
}

impl TexturedQuad {
    pub fn new(path_to_image: &str) -> Self {
        let verts = vec![
            // positions       // colors        // texture coords
             0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
             0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
            -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
        ];
        let inds = vec![
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];
        let image = path_to_image;

        let buf = RVertexBufferTextured::new((&verts, &inds, &image));

        Self {
            pos: vec3(0.0, 0.0, 0.0),
            buf,
            aspect_x: 1.0,
            aspect_y: 1.0,
        }
    }

    pub fn with_aspect(path_to_image: &str, w: f32, h: f32) -> Self {
        let verts = vec![
            // positions       // colors        // texture coords
             0.5 * w,  0.5 * h, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
             0.5 * w, -0.5 * h, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
            -0.5 * w, -0.5 * h, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
            -0.5 * w,  0.5 * h, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
        ];
        let inds = vec![
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];
        let image = path_to_image;

        let buf = RVertexBufferTextured::new((&verts, &inds, &image));

        Self {
            pos: vec3(0.0, 0.0, 0.0),
            buf,
            aspect_x: w,
            aspect_y: h,
        }
    }

    // pub fn clear(&mut self) {
    //     self.buf.clear();
    // } 
}


impl Drawable for TexturedQuad {
    unsafe fn draw(&self, shader: &super::Shader, w: f32, h: f32, s: f32, r: f32, camera: &Camera) {
        BindTexture(TEXTURE_2D, self.buf.texture_id);
        shader.use_shader();
        // pass uniforms
        
        shader.uniform_vec3f(cstr!("pos"), &self.pos);
        shader.uniform_vec3f(cstr!("color"), &vec3(0.51, 0.55, 0.8));
        shader.uniform_1f(cstr!("width"), w);
        shader.uniform_1f(cstr!("height"), h);
        shader.uniform_1f(cstr!("scale"), s);
        shader.uniform_1f(cstr!("rotation"), r);
        camera.send_uniforms(&shader);

        BindVertexArray(self.buf.vao_id);
        DrawElements(TRIANGLES, 6, UNSIGNED_INT, std::ptr::null());
        BindVertexArray(0);
        BindTexture(TEXTURE_2D, 0);
    }
}
