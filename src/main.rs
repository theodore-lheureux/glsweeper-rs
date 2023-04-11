// #![windows_subsystem = "windows"]

use glsweeper_rs::{
    clear_draw,
    game::Game,
    graphics::{shader::Shader, texture::Texture, window::Window},
    logger,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    let mut window = Window::new(
        glsweeper_rs::DEFAULT_WINDOW_WIDTH,
        glsweeper_rs::DEFAULT_WINDOW_HEIGHT,
        "GL Sweeper",
    );
    window.set_icon(include_bytes!("../icon.png").to_vec());
    window.init_gl();

    let vs_code: String = String::from_utf8(include_bytes!("../shaders/tile.vs").to_vec())?;
    let fs_code: String = String::from_utf8(include_bytes!("../shaders/tile.fs").to_vec())?;
    let tile_shader = Shader::new(vs_code, fs_code);

    let mut texture_atlas = Texture::new(glsweeper_rs::load_images(), 0);

    texture_atlas.bind(0);

    let mut current_game = Game::new(glsweeper_rs::DEFAULT_WIDTH, glsweeper_rs::DEFAULT_HEIGHT);

    tile_shader.use_program();

    while !window.should_close() {
        clear_draw(0.3, 0.3, 0.3, 1.0);
        current_game.draw();
        window.update(&mut current_game);
    }

    Ok(())
}
