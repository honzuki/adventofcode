use std::fs::File;

use crate::cpu::{Screen, LIT, SCREEN_HEIGHT, SCREEN_WIDTH};

const SCALE: usize = 30;

pub struct GifGen {
    encoder: gif::Encoder<File>,
}

impl GifGen {
    pub fn new() -> GifGen {
        let image = File::create("screen.gif").unwrap();
        let mut encoder = gif::Encoder::new(
            image,
            (SCREEN_WIDTH * SCALE) as u16,
            (SCREEN_HEIGHT * SCALE) as u16,
            &[],
        )
        .unwrap();
        encoder.set_repeat(gif::Repeat::Infinite).unwrap();

        GifGen { encoder }
    }

    pub fn encode_frame(&mut self, screen: Screen) {
        let mut pixels = vec![0u8; SCREEN_WIDTH * SCREEN_HEIGHT * SCALE * SCALE];

        for row in 0..SCREEN_HEIGHT {
            for column in 0..SCREEN_WIDTH {
                let idx = (SCREEN_WIDTH * row) + column;
                let value = if screen[idx] == LIT { 3 } else { 0 };

                for row_scale in 0..SCALE {
                    for column_scale in 0..SCALE {
                        let row = (row * SCALE) + row_scale;
                        let column = (column * SCALE) + column_scale;

                        let scaled_idx = ((SCREEN_WIDTH * SCALE) * row) + column;
                        pixels[scaled_idx] = value;
                    }
                }
            }
        }

        let frame = gif::Frame::from_palette_pixels(
            (SCREEN_WIDTH * SCALE) as u16,
            (SCREEN_HEIGHT * SCALE) as u16,
            &pixels,
            &[
                25, 25, 25, 25, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
                75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
            ],
            None,
        );

        self.encoder.write_frame(&frame).unwrap();
    }
}
