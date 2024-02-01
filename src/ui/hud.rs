use crate::sf::Drawable;
use crate::sf::Buffer;
use crate::sf::TexturedQuad;
use crate::sf::{Renderer, Shader, T_QUAD_FS, T_QUAD_VS};
use crate::sf::Camera;
use crate::environment::World;

use glfw::Window;

struct Frame {
    tex_quad: TexturedQuad,
    child_buttons: Vec<Button>,
    rot: f32,
    scale: f32,

    sod: SecondOrderDynamics,

    deployed: bool,
}

#[derive(PartialEq, Debug)]
enum ButtonState {
    Hovered,
    None,
}

use crate::util::SecondOrderDynamics;
struct Button {
    // parent_frame: Option<Frame>,
    tex_quad: TexturedQuad,
    flip_flop: bool,
    state: ButtonState,
    scale: f32,
    sod: SecondOrderDynamics,
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


    pub unsafe fn draw(
        &mut self, 
        w: f32, 
        h: f32, 
        camera: &Camera, 
        window: &Window, 
        world: &mut World,
    ) {
        Renderer::r_draw(
            self.main_frame.tex_quad, 
            &self.hud_shader, 
            w, 
            h, 
            self.main_frame.scale, 
            self.main_frame.rot, 
            camera,
        );

        self.main_frame.deployed = self.main_frame.child_buttons[0].flip_flop;
        self.main_frame.update();

        for button in &mut self.main_frame.child_buttons {
            button.tex_quad.draw(&self.hud_shader, w, h, button.scale, button.tex_quad.rot, camera);
            world.hud_button_callback(); 

            button.update(&window);
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
            scale: 1.0,
            rot: 0.0,

            deployed: false,
            sod: SecondOrderDynamics::new(5.0, 0.5, 1.0, vec3(0.0, -1.0, 0.0)),
        }
    }

    pub fn update(&mut self) {
        match self.deployed {
            false => {
                let y = self.sod.update(0.01, vec3(0.0, -1.0, 0.0));
                self.tex_quad.pos = y;
            },

            true => {
                let y = self.sod.update(0.01, vec3(0.0, -1.23, 0.0));
                self.tex_quad.pos = y;
            },
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
            flip_flop: false,
            state: ButtonState::None,
            scale: 1.0,
            sod: SecondOrderDynamics::new(5.0, 0.5, 1.0, vec3(1.0, -0.724, 0.0)),
        }
    } 

    pub fn in_bounds(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        // dbg!(y);
        let (x_pos, y_pos) = ((self.tex_quad.pos.x * w / 2.0), (self.tex_quad.pos.y * h / 2.0));

        let bound_x = 50.0 + x_pos;
        let neg_bound_x = -50.0 + x_pos;
        let bound_y = 50.0 - y_pos;
        let neg_bound_y = -50.0 - y_pos;

        let verts = vec![
             bound_y, bound_x,
             bound_y, neg_bound_x,
             neg_bound_y, neg_bound_x,
             neg_bound_y, bound_x,
        ];

        Geometry::in_point_inside_polygon2d(x, y, &verts, w, h)
    } 

    pub fn update(&mut self, window: &Window) {
        match self.state {
            ButtonState::Hovered => {
                let mut goal_pos = vec3(1.5, -0.724, 0.0);
                if self.flip_flop == true {
                    goal_pos = vec3(1.5, -0.924, 0.0);
                } else {
                    goal_pos = vec3(1.5, -0.724, 0.0);
                }
                let y = self.sod.update(0.01, goal_pos);
                self.tex_quad.pos.y = y.y; 
                
                if window.get_mouse_button(glfw::MouseButton::Button1) == glfw::Action::Press {
                    self.flip_flop = !self.flip_flop;
                    if self.flip_flop == true {
                        self.tex_quad.rot = 3.1415;
                    } else {
                        self.tex_quad.rot = 0.0; 
                    }
                    self.state = ButtonState::None;
                }

                self.scale = y.x; 
            },

            ButtonState::None => {
                let mut goal_pos = vec3(1.5, -0.724, 0.0);
                if self.flip_flop == true {
                    goal_pos = vec3(1.0, -0.924, 0.0);
                } else {
                    goal_pos = vec3(1.0, -0.724, 0.0);
                }
                let y = self.sod.update(0.01, goal_pos);
                self.scale = y.x;
                self.tex_quad.pos.y = y.y; 
                self.state = ButtonState::None;
            },
        }
    }
}
