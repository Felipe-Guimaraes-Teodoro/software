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
    pub world: World,
    hud: Hud, 

    slider_val: f32,

    width: i32,
    height: i32,
}

impl Application {
    pub fn new(mut window: PWindow, glfw: Glfw) -> Self {
        let world = World::new();
        let hud = Hud::new();

        let renderer = Renderer::new();

        let ctx = imgui::Context::create();
        let ui = Imgui::new(ctx, &mut window);

        Self {
            window,
            glfw,
            ui,
            renderer,
            world,
            hud,

            slider_val: 0.0,

            width: 800,
            height: 800,
        } 
    }

    pub fn ui(&mut self) {
        let frame = self.ui.frame(&mut self.window);

        let fdl = frame.get_foreground_draw_list();

        let caster_handle = crate::physics::GLOBAL_CASTER.clone();
        let mut locked_caster = caster_handle.lock().unwrap();
        locked_caster.draw_lines(&fdl);
        self.world.debug_mirrors(&fdl);

        let _slider = frame.slider("slider", -0.5, 0.5, &mut self.slider_val);

        frame.text(format!("{:?}", 1.0/self.renderer.camera.dt));
    } 

    pub fn update(&mut self) {
        self.renderer.camera.update();
        // we don't want to move the camera (for now)
        // self.renderer.camera.input(&mut self.window, &self.glfw);

        let caster_handle = crate::physics::GLOBAL_CASTER.clone();
        let mut locked_caster = caster_handle.lock().unwrap();
        let mirrors  =self.world.mirrors.clone();
        locked_caster.update(&mirrors);

        // self.world.mirrors[0].update(cgmath::vec3(0.0, 0.0, 0.0), self.slider_val * 6.28);
        self.world.io(&mut self.glfw, &mut self.window);
    }

    pub unsafe fn render(&mut self) {
        ClearColor(0.0, 0.0, 0.0, 0.0); 
        Clear(COLOR_BUFFER_BIT);

        self.renderer.draw(); 
        self.renderer.draw_world(&self.world);
        self.ui.draw();
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn glfw_mut(&mut self) -> &mut Glfw {
        &mut self.glfw
    }

    pub fn mouse(&mut self, x: f32, y:f32) {
        // we also dont want mouse moving camera
        // self.renderer.camera.mouse_callback(x, y, &self.window);
    } 

    pub fn set_framebuffer_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;

        self.world.set_framebuffer_size(width as f32, height as f32);
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
