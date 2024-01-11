use glfw::*;
use gl::*;
use crate::physics::{RayCaster, CastResult};

use crate::{sf::*, ui::Imgui};
use crate::environment::World;

use std::sync::{Arc, Mutex};

pub struct Application {
    window: PWindow,
    glfw: Glfw,
    pub ui: Imgui,
    renderer: Renderer,
    world: World,
    ray_caster: Arc<Mutex<RayCaster>>,

    slider_val: f32,
}

impl Application {
    pub fn new(mut window: PWindow, glfw: Glfw) -> Self {
        let mut world = World::new();
        let ray_caster = RayCaster::new();
        world.push_mirror(cgmath::vec3(0.0, 0.0, 0.0), 0.0); // debug mirror
        // world.push_mirror(cgmath::vec3(0.0, 0.0, 0.0), 1.57); // debug mirror

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

        let ofs = 0.0;

        // for i in -64..64 {
        //     let ofs = i as f32;
        //     self.ray_caster.cast((0.0, 400.0 + ofs), 0.0, 800.0, &fdl, 0, None);
        // }

        self.ray_caster.lock().unwrap().draw_lines(&fdl);

        let _slider = frame.slider("slider", -0.5, 0.5, &mut self.slider_val);

        frame.text(format!("{:?}", 1.0/self.renderer.camera.dt));

        let m_pos = frame.io().mouse_pos;

        self.ray_caster.lock().unwrap().clear_lines();
    } 

    pub fn update(&mut self) {
        self.renderer.camera.update();
        self.renderer.camera.input(&mut self.window, &self.glfw);

        self.ray_caster.lock().unwrap().update(&self.world.mirrors);


        let clone = self.ray_caster.clone();
        for i in -64..64 {
            let clone = clone.clone();
            // crate::GLOBAL_POOL.execute(move || {
                let mut ray_caster = clone.lock().unwrap();
                
                ray_caster.cast(CastResult {
                    start_pos: (0.0, 400.0 + i as f32),
                    angle: 0.0,
                    length: 800.0,
                    depth: 0,
                    ignore_mirror: None,
                    previous_lines: vec![],
                });
            // });
        }
        
        

        self.world.mirrors[0].update(cgmath::vec3(0.0, 0.5, 0.0), self.slider_val * 3.14);
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
        self.renderer.camera.mouse_callback(x, y, &self.window);
    } 
}
// fallen was here
// GOUD too my G
//

/*
 * app.renderer.add_polygon( VERTICES: *&Vec<f32>*, COR: *Vector3<f32>* );
 */
