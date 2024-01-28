use crate::sf::Drawable;
use crate::sf::Buffer;
use crate::sf::TexturedQuad;
use crate::sf::{Renderer, Shader, T_QUAD_FS, T_QUAD_VS};
use crate::sf::Camera;

struct Frame {
    tex_quad: TexturedQuad,
}

pub struct Hud {
    // hud is a textured quad on the screen
    // it shows the user the current tool
    // they're using by utilizing another 
    // quad. similar to minecraft

    // we need to implement TexturedQuad    

    main_frame: Frame,
    frame_shader: Shader,
}

impl Hud {
    pub fn new() -> Self {
        let main_frame = Frame::new();
        let frame_shader = Shader::new_pipeline(T_QUAD_VS, T_QUAD_FS);

        Self {
            main_frame,
            frame_shader,
        }
    }


    pub unsafe fn draw(&self, w: f32, h: f32, camera: &Camera) {
        Renderer::r_draw(self.main_frame.tex_quad, &self.frame_shader, w, h, camera);
    }
}

impl Frame {
    pub fn new() -> Self {
        let tex_loc = "assets/qd.jpg";

        let tex_quad = TexturedQuad::with_aspect(tex_loc, 1.0, 0.5);

        Self {
            tex_quad,
        }
    }
}
