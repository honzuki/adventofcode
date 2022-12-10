pub const SCREEN_WIDTH: usize = 40;
pub const SCREEN_HEIGHT: usize = 6;
const SPRITE_SIZE: usize = 3;

pub type Screen = [char; SCREEN_HEIGHT * SCREEN_WIDTH];

pub const LIT: char = '#';
pub const DARK: char = '.';

pub struct CPU {
    reg_x: i32,
    cycle: usize,
    signal_strength: i32,

    screen: Screen,
    screen_hooker: Option<crate::gif::GifGen>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg_x: 1,
            cycle: 0,
            signal_strength: 0,
            screen: [DARK; SCREEN_HEIGHT * SCREEN_WIDTH],
            screen_hooker: None,
        }
    }

    fn tick(&mut self) {
        // draw to screen
        let idx = self.cycle % (SCREEN_HEIGHT * SCREEN_WIDTH);
        let draw_idx = (self.cycle % SCREEN_WIDTH) as i32 + 1;
        self.screen[idx] = if draw_idx >= self.reg_x && draw_idx < (self.reg_x + SPRITE_SIZE as i32)
        {
            LIT
        } else {
            DARK
        };

        if let Some(handler) = &mut self.screen_hooker {
            handler.encode_frame(self.screen.clone());
        }

        // update cycle
        self.cycle += 1;

        // find signal strength
        if (self.cycle % 40) == 20 {
            self.signal_strength += self.cycle as i32 * self.reg_x;
        }
    }

    pub fn add_x(&mut self, val: i32) {
        self.tick();
        self.tick();
        self.reg_x += val;
    }

    pub fn noop(&mut self) {
        self.tick();
    }

    pub fn get_signal_strength(&self) -> i32 {
        self.signal_strength
    }

    pub fn display_screen(&self) -> String {
        let mut result = String::new();

        for row in 0..SCREEN_HEIGHT {
            for column in 0..SCREEN_WIDTH {
                let idx = (SCREEN_WIDTH * row) + column;
                result.push(self.screen[idx]);
            }
            result.push('\n');
        }

        result
    }

    pub fn hook_screen(&mut self, handler: crate::gif::GifGen) {
        self.screen_hooker = Some(handler);
    }
}
