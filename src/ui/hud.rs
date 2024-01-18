use crate::sf::Drawable;

pub struct Hud {
    // hud is a textured quad on the screen
    // it shows the user the current tool
    // they're using by utilizing another 
    // quad. similar to minecraft

    // we need to implement TexturedQuad    
}

impl Hud {
    pub fn new() -> Self {
        Self {}
    }

}


impl Drawable for Hud {
    unsafe fn draw(&self, shader: &crate::sf::Shader) {
        shader.use_shader();
    }
}
