use glfw::*;
use gl::*;
use crate::sf::*;

pub struct Application {
    window: PWindow,
    glfw: Glfw,
    // ui 
    camera: Camera,
    renderer: Renderer,
}

impl Application {
    pub fn new(window: PWindow, glfw: Glfw) -> Self {
        let camera = Camera::new();
        let mut renderer = Renderer::new();

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
            camera,
            renderer,
        } 
    }

    pub fn update(&mut self) {
        self.camera.update();
        self.camera.input(&mut self.window, &self.glfw);
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
}
// fallen was here
// GOUD too my G
