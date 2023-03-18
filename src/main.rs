#![windows_subsystem = "windows"]

use glsweeper_rs::{
    clear_draw,
    game::{game_textures::GameTextures, Game},
    graphics::{shader::Shader, window::Window},
    logger,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    let mut window = Window::new(
        glsweeper_rs::WIDTH_PX as u32,
        glsweeper_rs::HEIGHT_PX as u32,
        "GL Sweeper",
    );
    window.init_gl();

    let vs_code: String = String::from_utf8(include_bytes!("../shaders/tile.vs").to_vec())?;
    let fs_code: String = String::from_utf8(include_bytes!("../shaders/tile.fs").to_vec())?;
    let tile_shader = Shader::new(vs_code, fs_code);
    let mut textures = GameTextures::new();

    let mut current_game = Game::new(glsweeper_rs::WIDTH, glsweeper_rs::HEIGHT);

    tile_shader.use_program();

    while !window.should_close() {
        clear_draw(0.2, 0.3, 0.3, 1.0);
        current_game.draw(&mut textures);
        window.update(&mut current_game);
    }

    Ok(())
}
