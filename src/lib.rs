use std::ptr;

use graphics::gl_wrapper::VAO;

pub mod game;
pub mod graphics;
pub mod logger;

pub const DEFAULT_WIDTH: isize = 11;
pub const DEFAULT_HEIGHT: isize = 11;

pub const MAX_WIDTH: isize = 950;
pub const MAX_HEIGHT: isize = 950;
pub const MIN_WIDTH: isize = 1;
pub const MIN_HEIGHT: isize = 1;

pub const WIDTH_INCREMENT: isize = 5;
pub const HEIGHT_INCREMENT: isize = 5;

pub const DEFAULT_WINDOW_WIDTH: u32 = 800;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 800;

pub fn clear_draw(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn draw_element(vao: &VAO, count: i32) {
    vao.bind();
    unsafe {
        gl::DrawElements(gl::TRIANGLES, count, gl::UNSIGNED_INT, ptr::null());
    }
}

pub fn load_images() -> Vec<u8> {
    let mut files = vec! {
        include_bytes!("../textures/tile_unrevealed.png").to_vec(),
        include_bytes!("../textures/flag.png").to_vec(),
        include_bytes!("../textures/tile_revealed_0.png").to_vec(),
        include_bytes!("../textures/tile_revealed_1.png").to_vec(),
        include_bytes!("../textures/tile_revealed_2.png").to_vec(),
        include_bytes!("../textures/tile_revealed_3.png").to_vec(),
        include_bytes!("../textures/tile_revealed_4.png").to_vec(),
        include_bytes!("../textures/tile_revealed_5.png").to_vec(),
        include_bytes!("../textures/tile_revealed_6.png").to_vec(),
        include_bytes!("../textures/tile_revealed_7.png").to_vec(),
        include_bytes!("../textures/tile_revealed_8.png").to_vec(),
        include_bytes!("../textures/mine_revealed.png").to_vec(),
        include_bytes!("../textures/mine_exploded.png").to_vec(),
        include_bytes!("../textures/flag_wrong.png").to_vec(),
    };

    let mut data: Vec<u8> = Vec::new();

    for file in files.iter_mut() {
        let mut image = image::load_from_memory(file).expect("Failed to load texture image.");
        let rgba = image.into_rgb8();
        data.append(&mut rgba.to_vec());
    }

    data
}
