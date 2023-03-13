use std::path::Path;

use glsweeper_rs::{
    clear_draw,
    game::Game,
    graphics::{shader::Shader, texture::Texture, window::Window},
    logger,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    let mut window =
        Window::new(glsweeper_rs::WIDTH_PX as u32, glsweeper_rs::HEIGHT_PX as u32, "GL Sweeper");
    window.init_gl();

    let tile_shader = Shader::new("shaders/tile.vs", "shaders/tile.fs");
    
    let mut texture_unrevealed_tile = Texture::new(Path::new("textures/tile_unrevealed.png"), 0);
    let mut texture_flag = Texture::new(Path::new("textures/flag.png"), 0);
    
    let mut current_game = Game::new(glsweeper_rs::WIDTH, glsweeper_rs::HEIGHT);

    tile_shader.use_program();

    while !window.should_close() {
        clear_draw(0.2, 0.3, 0.3, 1.0);
        current_game.draw(&mut texture_unrevealed_tile, &mut texture_flag);
        window.update(&mut current_game);
    }

    Ok(())
}
