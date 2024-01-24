use glfw::*;
use crate::application::*;
use crate::physics::GLOBAL_CASTER;

pub fn run() {
    // initialize window
    let mut glfw = glfw::init(fail_on_errors!()).unwrap(); 
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(
        glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core)
    );

    let (mut window, events) = glfw.create_window(800, 800, "Hello, world!", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();


    // glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    gl::load_with(|s| window.get_proc_address(s) as * const _);

    let mut app = Application::new(window, glfw);

    while !app.window_mut().should_close() {
        app.ui();
        // let raycaster = GLOBAL_CASTER.clone();
        // let mirrors = app.world.mirrors.clone();
        // crate::GLOBAL_POOL.execute(move || {
        //     if raycaster.lock().unwrap().can_draw() {
        //         Application::raycaster(raycaster, mirrors);
        //     } else {
        //         // nothing
        //     }
        // });
        app.update();
        unsafe {
            app.render();
        }

        app.window_mut().swap_buffers();
        app.glfw_mut().poll_events();
        
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut app, event);
        }
    }
}

fn handle_window_event(app: &mut Application, event: glfw::WindowEvent) {
    match event {
        WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            app.window_mut().set_should_close(true)
        },

        WindowEvent::Key(Key::LeftAlt, _, Action::Press, _) => {
            if app.window_mut().get_cursor_mode() == CursorMode::Disabled {
                app.window_mut().set_cursor_mode(CursorMode::Normal)
            } else {
                app.window_mut().set_cursor_mode(CursorMode::Disabled)
            }
        },

        WindowEvent::MouseButton(button, action, _) => {
            app.ui.on_mouse_click(button, action);
        },
        WindowEvent::Scroll(x, y) => {
            app.ui.on_mouse_scroll(x as f32, y as f32);
            app.world.scroll_wheel(y as f32); 
        },
        WindowEvent::CursorPos(x, y) => {
            let (x, y) = (x as f32, y as f32);
            app.mouse(x, y);
            app.ui.on_mouse_move(x, y);
        },

        WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        },

        _ => {},
    }
}
