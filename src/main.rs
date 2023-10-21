extern crate gfx_demo;

const WINDOW_TITLE: &'static str = "Fire";
const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const CANVAS_WIDTH: u32 = 100;
const CANVAS_HEIGHT: u32 = 100;
const TICK_MS: u64 = 10;

fn main() {
    let mut i = 0;
    let mut inc = true;

    gfx_demo::gfx_demo(
        WINDOW_TITLE,
        WINDOW_WIDTH, WINDOW_HEIGHT,
        CANVAS_WIDTH, CANVAS_HEIGHT,
        TICK_MS,
        |pixels: &mut Vec<u32>| {
            if inc {
                i += 1;
            } else {
                i -= 1;
            }

            inc = match i {
                0 => true,
                255 => false,
                _ => inc
            };

            for y in 0..CANVAS_HEIGHT {
                for x in 0..CANVAS_WIDTH {
                    pixels[((y * CANVAS_WIDTH) + x) as usize] = 0xff000000u32 | (i << 16);
                }
            }
        }
    );
}
