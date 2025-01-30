use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const SCALE: f64 = 10.0;

#[macroquad::main("Heightmap Renderer")]
async fn main() {
    let perlin = Perlin::new(42);
    let mut pixels = vec![0u8; WIDTH * HEIGHT];

    // Generate the heightmap
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let nx = x as f64 / WIDTH as f64;
            let ny = y as f64 / HEIGHT as f64;
            let value = perlin.get([nx * SCALE, ny * SCALE]);
            let normalized = ((value + 1.0) / 2.0 * 255.0) as u8;
            pixels[y * WIDTH + x] = normalized;
        }
    }

    // Convert heightmap to texture
    let texture = Texture2D::from_rgba8(WIDTH as u16, HEIGHT as u16, &pixels_to_rgba(&pixels));

    loop {
        clear_background(BLACK);

        // Draw the heightmap texture
        draw_texture(&texture, 0.0, 0.0, WHITE);

        next_frame().await;
    }
}

fn pixels_to_rgba(pixels: &[u8]) -> Vec<u8> {
    pixels.iter().flat_map(|&p| vec![p, p, p, 255]).collect()
}
