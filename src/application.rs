use glfw::*;
use gl::*;
use crate::physics::RayCaster;
use crate::ui::Hud;

use crate::{sf::*, ui::Imgui};
use crate::environment::World;

use std::sync::{Arc, Mutex};

pub struct Application {
    window: PWindow,
    glfw: Glfw,
    pub ui: Imgui,
    pub renderer: Renderer,

    width: i32,
    height: i32,

    pub surface: Surface,
}

impl Application {
    pub fn new(mut window: PWindow, glfw: Glfw) -> Self {
        let renderer = Renderer::new();

        let ctx = imgui::Context::create();
        let ui = Imgui::new(ctx, &mut window);

        let surface = Surface::new();

        Self {
            window,
            glfw,
            ui,
            renderer,

            surface,

            width: 800,
            height: 800,
        } 
    }

    pub fn ui(&mut self) {
        let frame = self.ui.frame(&mut self.window);

        let fdl = frame.get_foreground_draw_list();

        frame.text(format!("{:?}", 1.0/self.renderer.camera.dt));
    } 

    pub fn update(&mut self) {
        self.renderer.camera.update();
        self.renderer.camera.input(&mut self.window, &self.glfw);
        self.surface.update(&self.window);
    }

    pub unsafe fn render(&mut self) {
        ClearColor(0.0, 0.0, 0.0, 0.0); 
        Clear(COLOR_BUFFER_BIT);

        self.surface.draw(&self.renderer.camera, self.width as f32, self.height as f32);
        self.renderer.draw(); 
        self.ui.draw();
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn glfw_mut(&mut self) -> &mut Glfw {
        &mut self.glfw
    }

    pub fn mouse(&mut self, x: f32, y:f32) {
        self.renderer.camera.mouse_callback(x, y, &self.window);
    } 

    pub fn set_framebuffer_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }
}
// fallen was here
// GOUD too my G
//

/*
 * app.renderer.add_polygon( VERTICES: *&Vec<f32>*, COR: *Vector3<f32>* );
 */


fn rand_vec3() -> cgmath::Vector3<f32> {
    cgmath::vec3(crate::util::Math::random(0.0, 1.0), crate::util::Math::random(0.0, 1.0), crate::util::Math::random(0.0, 1.0))
}
