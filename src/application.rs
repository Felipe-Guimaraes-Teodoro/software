use glfw::*;
use gl::*;
use crate::physics::RayCaster;

use crate::{sf::*, ui::Imgui};
use crate::environment::World;

use std::sync::{Arc, Mutex};

pub struct Application {
    window: PWindow,
    glfw: Glfw,
    pub ui: Imgui,
    renderer: Renderer,
    pub world: World,
    pub ray_caster: Arc<Mutex<RayCaster>>,

    slider_val: f32,
}

impl Application {
    pub fn new(mut window: PWindow, glfw: Glfw) -> Self {
        let mut world = World::new();
        let ray_caster = RayCaster::new();
        world.push_mirror(cgmath::vec3(0.0, -0.5, 0.0), 0.0); // debug mirror
        world.push_mirror(cgmath::vec3(-0.1, 0.1, 0.0), -0.6); // debug mirror

        let renderer = Renderer::new();

        let ctx = imgui::Context::create();
        let ui = Imgui::new(ctx, &mut window);

        Self {
            window,
            glfw,
            ui,
            renderer,
            world,
            ray_caster: Arc::new(Mutex::new(ray_caster)),

            slider_val: 0.0,
        } 
    }

    pub fn ui(&mut self) {
        let frame = self.ui.frame(&mut self.window);

        let fdl = frame.get_foreground_draw_list();

        self.ray_caster.lock().unwrap().draw_lines(&fdl);
        self.world.debug_mirrors(&fdl);

        let _slider = frame.slider("slider", -0.5, 0.5, &mut self.slider_val);

        frame.text(format!("{:?}", 1.0/self.renderer.camera.dt));
    } 

    pub fn update(&mut self) {
        self.renderer.camera.update();
        self.renderer.camera.input(&mut self.window, &self.glfw);



        self.world.mirrors[0].update(cgmath::vec3(-0.3, 0.3, 0.0), self.slider_val * 3.14 + 0.4);
        self.world.io(&mut self.glfw, &mut self.window);
    }

    pub fn raycaster(ray_caster: Arc<Mutex<RayCaster>>, mirrors: Vec<crate::environment::Mirror>) {
        // for i in -8..8 {
            ray_caster.lock().unwrap().cast((0.0, 400.0), 0.0, 400.0, 0, None);
        // }

        ray_caster.lock().unwrap().update(&mirrors);
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
        self.renderer.camera.mouse_callback(x, y, &self.window);
    } 
}
// fallen was here
// GOUD too my G
//

/*
 * app.renderer.add_polygon( VERTICES: *&Vec<f32>*, COR: *Vector3<f32>* );
 */


fn rand_vec3() -> cgmath::Vector3<f32> {
    cgmath::vec3(crate::util::Math::random(-1.0, 1.0), crate::util::Math::random(-1.0, 1.0), crate::util::Math::random(-1.0, 1.0))
}
