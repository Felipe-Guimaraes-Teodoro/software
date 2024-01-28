use crate::sf::Drawable;
use crate::sf::Buffer;
use crate::sf::TexturedQuad;
use crate::sf::{Renderer, Shader, T_QUAD_FS, T_QUAD_VS};
use crate::sf::Camera;

struct Frame {
    tex_quad: TexturedQuad,
    child_buttons: Vec<Button>,
}

#[derive(PartialEq, Debug)]
enum ButtonState {
    Hovered,
    None,
}
struct Button {
    // parent_frame: Option<Frame>,
    tex_quad: TexturedQuad,
    state: ButtonState,
    scale: f32,
}

pub struct Hud {
    // hud is a textured quad on the screen
    // it shows the user the current tool
    // they're using by utilizing another 
    // quad. similar to minecraft

    // we need to implement TexturedQuad    

    main_frame: Frame,
    hud_shader: Shader,

    pub w: f32, pub h: f32,
    mouse_x: f32, mouse_y: f32,
}

impl Hud {
    pub fn new() -> Self {
        let main_frame = Frame::new_hud_custom();
        let hud_shader = Shader::new_pipeline(T_QUAD_VS, T_QUAD_FS);

        Self {
            main_frame,
            hud_shader,
            w: 800.0, h: 800.0,
            mouse_x: 0.0, mouse_y: 0.0,
        }
    }


    pub unsafe fn draw(&mut self, w: f32, h: f32, camera: &Camera) {
        Renderer::r_draw(self.main_frame.tex_quad, &self.hud_shader, w, h, 1.0, 0.0, camera);

        for button in &mut self.main_frame.child_buttons {
            button.tex_quad.draw(&self.hud_shader, w, h, button.scale, 0.0, camera);

            button.update();
        }
    }

    pub fn mouse(&mut self, x: f32, y: f32) {
        self.mouse_y = y;
        self.mouse_x = x;

        for button in &mut self.main_frame.child_buttons {
            if button.in_bounds(x, y, self.w, self.h) {
                button.state = ButtonState::Hovered;
            } else {
                button.state = ButtonState::None;
            } 
        }
    }
}

use cgmath::*;
impl Frame {
    pub fn new_hud_custom() -> Self {
        let tex_loc = "assets/qd.jpg";

        let mut tex_quad = TexturedQuad::with_aspect(tex_loc, 1.0, 0.5);
        tex_quad.pos = vec3(0.0, -1.0, 0.0);

        let child_buttons = vec![
            Button::new(),
        ];

        Self {
            tex_quad,
            child_buttons,
        }
    }
}

use crate::util::Geometry;
impl Button {
    pub fn new() -> Self {
        let tex_loc = "assets/bt.jpg";

        let mut tex_quad = TexturedQuad::with_aspect(tex_loc, 0.05, 0.05);
        tex_quad.pos = vec3(0.0, -0.724, 0.0);

        Self {
            tex_quad,
            state: ButtonState::None,
            scale: 1.0,
        }
    } 

    pub fn in_bounds(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        // dbg!(y);
        let (x_pos, y_pos) = (self.tex_quad.pos.x * (w / 2.0), self.tex_quad.pos.y * (h / 2.0));

        let verts = vec![
             (0.5 * self.tex_quad.aspect_x * w / 2.0) + x_pos,  (0.5 * self.tex_quad.aspect_y * h / 2.0) + y_pos,
             (0.5 * self.tex_quad.aspect_x * w / 2.0) + x_pos,  (-0.5 * self.tex_quad.aspect_y * h / 2.0) + y_pos,
             (-0.5 * self.tex_quad.aspect_x * w / 2.0) + x_pos,  (-0.5 * self.tex_quad.aspect_y * h / 2.0) + y_pos,
             (-0.5 * self.tex_quad.aspect_x * w / 2.0) + x_pos,  (0.5 * self.tex_quad.aspect_y * h / 2.0) + y_pos,
        ];

        Geometry::in_point_inside_polygon2d(y - h / 1.38, x, &verts, w, h)
    } 

    pub fn update(&mut self) {
        match self.state {
            ButtonState::Hovered => {
                // use SOD here!
                self.scale = 1.5; 
            },

            ButtonState::None => {
                self.scale = 1.0;
                self.state = ButtonState::None;
            },
        }
    }
}
