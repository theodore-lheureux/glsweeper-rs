pub fn tile_position(
    x_px: f64,
    y_px: f64,
    width_tiles: isize,
    height_tiles: isize,
    window_width: f64,
    window_height: f64,
) -> (isize, isize) {
    let width_tiles = width_tiles as f64;
    let height_tiles = height_tiles as f64;
    let (offset_x, offset_y) = (
        (window_width - window_height) / 2.0,
        (window_height - window_width) / 2.0,
    );

    if window_width > window_height {
        let x = x_px - offset_x;
        (
            (x / window_height * width_tiles) as isize,
            (height_tiles - y_px / window_height * height_tiles) as isize,
        )
    } else {
        let y = y_px - offset_y;
        (
            (x_px / window_width * width_tiles) as isize,
            (height_tiles - y / window_width * height_tiles) as isize,
        )
    }
}

pub fn random_coords(width: isize, height: isize) -> (isize, isize) {
    (
        (rand::random::<usize>() % width as usize) as isize,
        (rand::random::<usize>() % height as usize) as isize,
    )
}
