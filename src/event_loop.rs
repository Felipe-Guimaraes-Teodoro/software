use glfw::*;
use crate::application::*;

pub fn run() {
    // initialize window
    let mut glfw = glfw::init(fail_on_errors!()).unwrap(); 
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(
        glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core)
    );

    let (mut window, events) = glfw.create_window(300, 300, "Hello, world!", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

   //load gl functions
    gl::load_with(|s| window.get_proc_address(s) as * const _);

    let mut app = Application::new(window, glfw);

    while !app.window_mut().should_close() {
        app.update();
        unsafe {
            app.render();
        }

        app.window_mut().swap_buffers();
        app.glfw_mut().poll_events();
        
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(app.window_mut(), event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
