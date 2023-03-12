use glfw::{Action, Context, Key, WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

            window.set_framebuffer_size_polling(true);
            window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        // window.set_cursor_mode(glfw::CursorMode::Disabled);

        Self { glfw, window, events }
    }

    pub fn init_gl(&mut self) {
        self.window.make_current();
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
        unsafe {
            gl::Viewport(0, 0, self.window.get_framebuffer_size().0 as i32, self.window.get_framebuffer_size().1 as i32);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window.swap_buffers();
    }

}