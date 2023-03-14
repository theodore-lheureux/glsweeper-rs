use glfw::{Action, Context, Key, WindowEvent};
use log::info;
use std::sync::mpsc::Receiver;

use crate::{game::Game, WIDTH, HEIGHT};

pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::Window,
    pub events: Receiver<(f64, WindowEvent)>,
    wireframe: bool,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_resizable(false);
        window.set_cursor_pos_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Normal);
        window.set_mouse_button_polling(true);

        Window {
            glfw,
            window,
            events,
            wireframe: false,
        }
    }

    pub fn init_gl(&mut self) {
        self.window.make_current();
        gl::load_with(|symbol| 
            self.window.get_proc_address(symbol) as *const _
        );
        unsafe {
            gl::Viewport(
                0,
                0,
                self.window.get_framebuffer_size().0 as i32,
                self.window.get_framebuffer_size().1 as i32,
            );
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn process_events(&mut self, game: &mut Game) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                },
                WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    if self.wireframe {
                        self.wireframe = false;
                        unsafe {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                        }
                    } else {
                        self.wireframe = true;
                        unsafe {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                        }
                    }
                },
                WindowEvent::Key(Key::R, _, Action::Press, _) => {
                    *game = Game::new(WIDTH, HEIGHT);
                },
                WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                    let x_px = self.window.get_cursor_pos().0 as i32;
                    let y_px = self.window.get_cursor_pos().1 as i32;

                    game.space_click(x_px, y_px);
                },
                WindowEvent::MouseButton(button, action, _) => {
                    match button {
                        glfw::MouseButtonLeft => {
                            if action == glfw::Action::Press {
                                let x = self.window.get_cursor_pos().0 as i32;
                                let y = self.window.get_cursor_pos().1 as i32;

                                game.left_click(x, y);
                                info!("Clicked tile at ({}, {})", x, y);
                            }
                        },
                        glfw::MouseButtonRight => {
                            if action == glfw::Action::Press {
                                let x = self.window.get_cursor_pos().0 as i32;
                                let y = self.window.get_cursor_pos().1 as i32;

                                game.right_click(x, y);
                            }
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    pub fn get_framebuffer_size(&self) -> (i32, i32) {
        self.window.get_framebuffer_size()
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }

    pub fn update(&mut self, game: &mut Game) {
        self.process_events(game);
        self.window.swap_buffers();
        self.glfw.poll_events();
        unsafe {
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn set_wireframe_mode(&mut self, wireframe: bool) {
        self.wireframe = wireframe;
        if wireframe {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
        } else {
            unsafe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }
        }
    }

}
