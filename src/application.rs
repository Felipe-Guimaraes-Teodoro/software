use glfw::*;
use gl::*;

use crate::{sf::*, util::Math};

static START: std::sync::Once = std::sync::Once::new();

pub struct Application {
    window: PWindow,
    glfw: Glfw,
    // ui 
    renderer: Renderer,
}

impl Application {
    pub fn new(window: PWindow, glfw: Glfw) -> Self {
        let mut renderer = Renderer::new();
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
            renderer,
        } 
    }

    pub fn update(&mut self) {
        self.renderer.camera.update();
        self.renderer.camera.input(&mut self.window, &self.glfw);

        for i in 0..1024 {
            let new_verts = &vec![
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
