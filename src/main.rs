use gfx_demo;
use rand;

const WINDOW_TITLE: &'static str = "Fire";
const WINDOW_WIDTH: usize = 640;
const WINDOW_HEIGHT: usize = 480;
const CANVAS_WIDTH: usize = 100;
const CANVAS_HEIGHT: usize = 100;
const CANVAS_SIZE: usize = CANVAS_WIDTH * CANVAS_HEIGHT;
const TICK_MS: u64 = 10;

const RAND_RANGE: u32 = 25;

fn init_fire<R: rand::Rng>(rng: &mut R, row: &mut [u32]) {
    let mut r = rng.gen_range(0..255);

    for p in row.iter_mut() {
        *p = 0xff000000u32 | r << 16;

        let min = if r < RAND_RANGE / 2 {
            0
        } else {
            r - RAND_RANGE / 2
        };
        let max = if r > 255 - RAND_RANGE / 2 {
            255
        } else {
            r + RAND_RANGE / 2
        };

        r = rng.gen_range(min..=max);
    }
}

fn next_fire<R: rand::Rng>(rng: &mut R, row: &mut [u32]) {
    for p in row.iter_mut() {
        let mut r = (*p & 0x00ff0000u32) >> 16;

        let min = if r < RAND_RANGE / 2 {
            0
        } else {
            r - RAND_RANGE / 2
        };
        let max = if r > 255 - RAND_RANGE / 2 {
            255
        } else {
            r + RAND_RANGE / 2
        };

        r = rng.gen_range(min..=max);
        *p = 0xff000000u32 | (r << 16);
    }
}

fn propagate(row: &mut [u32], row_below: &[u32]) -> bool {
    let len = row.len();
    let mut any_non_zero = false;

    for (index, p) in row.iter_mut().enumerate() {
        let mut total = (row_below[index] & 0x00ff0000u32) >> 16;
        let mut n = 1;

        if index > 0 {
            total += (row_below[index - 1] & 0x00ff0000u32) >> 16;
            n += 1;
        }

        if index < len - 1 {
            total += (row_below[index + 1] & 0x00ff0000u32) >> 16;
            n += 1;
        }

        let avg = (((total / n) as f64) * 0.95) as u32;
        any_non_zero = any_non_zero || avg > 0;

        *p = 0xff000000u32 | ((avg & 0xff) << 16);
    }

    any_non_zero
}

fn main() {
    let mut new_pixels = vec![0xff000000u32; CANVAS_WIDTH * CANVAS_HEIGHT];
    let mut init = true;
    let mut rng = rand::thread_rng();

    gfx_demo::gfx_demo(
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        TICK_MS,
        |pixels: &mut Vec<u32>| {
            let mut bottom_row = &mut new_pixels[CANVAS_SIZE - CANVAS_WIDTH..];

            if init {
                init = false;
                init_fire(&mut rng, &mut bottom_row);
            } else {
                next_fire(&mut rng, &mut bottom_row);
            }

            'propagate: for row_index in (0..CANVAS_HEIGHT - 1).rev() {
                let rows = &mut new_pixels[row_index * CANVAS_WIDTH .. (row_index + 2) * CANVAS_WIDTH];
                let (row, row_below) = rows.split_at_mut(CANVAS_WIDTH);

                if !propagate(row, row_below) {
                    break 'propagate;
                }
            }

            pixels.copy_from_slice(&new_pixels);
        },
    )
    .unwrap();
}
