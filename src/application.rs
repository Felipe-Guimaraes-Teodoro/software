use glfw::*;
use gl::*;

use std::sync::{Arc, Mutex};

use crate::{sf::*, ui::Imgui, util::Math};
use crate::environment::World;

pub struct Application {
    window: PWindow,
    glfw: Glfw,
    pub ui: Imgui,
    renderer: Renderer,
    world: Arc<Mutex<World>>, 
}

impl Application {
    pub fn new(mut window: PWindow, glfw: Glfw) -> Self {
        let world = Arc::new(Mutex::new(World::new()));
        world.lock().unwrap().push_mirror(cgmath::vec3(0.0, 0.0, 0.0), 0.0); // debug mirror
        let mut renderer = Renderer::new(Arc::clone(&world));

        let ctx = imgui::Context::create();
        let ui = Imgui::new(ctx, &mut window);
        // let mut ray_caster = RayCaster::new(&renderer); // abstracao para os raios

        for i in 0..1024 {
            renderer.add_polygon(
                &vec![
                    0.5 - i as f32 / 200.0, 0.0, 0.0,
                    0.0, 0.5 + i as f32 / 200.0, 0.0,
                    -0.5, 0.5, 0.0
                ],
                cgmath::vec3(i as f32 / 1024.0, i as f32 / 1024.0, 1.0),
            );
        }

        Self {
            window,
            glfw,
            ui,
            renderer,
            world,
        } 
    }

    pub fn ui(&mut self) {
        let frame = self.ui.frame(&mut self.window);

        frame.text("Hello, world!");
    } 

    pub fn update(&mut self) {
        self.renderer.camera.update();
        self.renderer.camera.input(&mut self.window, &self.glfw);

        for i in 0..1024 {
            let new_verts = vec![
                0.5 - i as f32 / 200.0, 0.0, Math::random(-1.0, 1.0),
                0.0 - Math::random(-0.03, 0.03), 0.5 + i as f32 / 200.0, Math::random(-1.0, 1.0), 
                -0.5, 0.5 + Math::random(-20.0, 20.0), Math::random(-1.0, 1.0),
            ];
            self.renderer.update_polygon(i, new_verts);
        }
    }

    pub unsafe fn render(&mut self) {
        ClearColor(0.0, 0.0, 0.0, 0.0); 
        Clear(COLOR_BUFFER_BIT);

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
}
// fallen was here
// GOUD too my G
//

/*
 * app.renderer.add_polygon( VERTICES: *&Vec<f32>*, COR: *Vector3<f32>* );
 */
