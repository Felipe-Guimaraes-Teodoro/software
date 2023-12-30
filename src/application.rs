use glfw::*;
use gl::*;
use crate::sf::*;

pub struct Application {
    window: PWindow,
    glfw: Glfw,
    // ui 
    camera: Camera,
}

impl Application {
    pub fn new(window: PWindow, glfw: Glfw) -> Self {
        let camera = Camera::new();

        Self {
            window,
            glfw,
            camera,
        } 
    }

    pub fn update(&mut self) {
        self.camera.update();
        self.camera.input(&mut self.window, &self.glfw);
        
        println!("{:?}", self.camera.pos);
    }

    pub unsafe fn render(&mut self) {
        ClearColor(1.0, 1.0, 1.0, 1.0); 
        Clear(COLOR_BUFFER_BIT);

        // rendering code
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
