use std::path::Path;

use glsweeper_rs::{
    graphics::{
        shader::Shader,
        window::Window, texture,
    },
    logger, game::Game, clear_draw
};

const WIDTH: i32 = 40;
const HEIGHT: i32 = 40;

const WIDTH_PX: i32 = 800;
const HEIGHT_PX: i32 = 800;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    let mut window = Window::new(WIDTH_PX as u32, HEIGHT_PX as u32, "GL Sweeper");
    window.init_gl();

    let shader_program =
        Shader::new("shaders/vertex.glsl", "shaders/fragment.glsl");
    shader_program.use_program();

    let current_game = Game::new(WIDTH, HEIGHT);

    let texture = texture::Texture::new(Path::new("textures/tile_unrevealed.png"));
    texture.bind();

    // window.set_wireframe_mode(true);

    while !window.should_close() {
        clear_draw(0.2, 0.3, 0.3, 1.0);
        current_game.draw();
        window.update();
    }

    Ok(())
}



