use glfw::*;
use gl::*;
use crate::physics::RayCaster;

use std::sync::{Arc, RwLock};

use crate::{sf::*, ui::Imgui, util::Math};
use crate::environment::World;

pub struct Application {
    window: PWindow,
    glfw: Glfw,
    pub ui: Imgui,
    renderer: Renderer,
    world: World,
    ray_caster: RayCaster,

    slider_val: f32,
}

impl Application {
    pub fn new(mut window: PWindow, glfw: Glfw) -> Self {
        let mut world = World::new();
        let ray_caster = RayCaster::new();
        world.push_mirror(cgmath::vec3(0.0, 0.0, 0.0), 0.0); // debug mirror

        let mut renderer = Renderer::new();

        let ctx = imgui::Context::create();
        let ui = Imgui::new(ctx, &mut window);

        renderer.add_polygon(
            &vec![
                0.5, 0.0, 0.0,
                0.0, 0.5, 0.0,
                -0.5, 0.5, 0.0,
            ],
            cgmath::vec3(0.0, 0.0, 0.0),
        );

        Self {
            window,
            glfw,
            ui,
            renderer,
            world,
            ray_caster,

            slider_val: 0.0,
        } 
    }

    pub fn ui(&mut self) {
        let frame = self.ui.frame(&mut self.window);

        // let ofs = 0.0;

        for i in -64..64 {
            let ofs = i as f32 / 1024.0;
            self.ray_caster.cast((0.0, 0.5 + ofs), 0.0, 0.5, &frame.get_foreground_draw_list(), 0);
        }

        let _slider = frame.slider("slider", -0.5, 0.5, &mut self.slider_val);

        let m_pos = frame.io().mouse_pos;
    } 

    pub fn fdl(&mut self) -> imgui::DrawListMut {
        let frame = self.ui.frame(&mut self.window);
        frame.get_foreground_draw_list()
    }

    pub fn update(&mut self) {
        self.renderer.camera.update();
        self.renderer.camera.input(&mut self.window, &self.glfw);

        self.ray_caster.update(&self.world.mirrors);

        self.world.mirrors[0].update(cgmath::vec3(0.0, 0.0, 0.0), self.slider_val * 3.14);
    }

    pub unsafe fn render(&mut self) {
        ClearColor(0.0, 0.0, 0.0, 0.0); 
        Clear(COLOR_BUFFER_BIT);

        self.renderer.draw(); 
        // self.renderer.draw_world(&self.world);
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
}
// fallen was here
// GOUD too my G
//

/*
 * app.renderer.add_polygon( VERTICES: *&Vec<f32>*, COR: *Vector3<f32>* );
 */
