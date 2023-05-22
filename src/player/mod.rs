use bracket_lib::terminal::{to_cp437, BTerm, BLACK, YELLOW};

pub struct Player {
    pub y: i32,
    velocity: f32,
}

impl Player {
    pub fn new(y: i32) -> Self {
        Player { y, velocity: 0.0 }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    pub fn apply_gravity(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }
    }

    pub fn flap(&mut self) {
        self.velocity += -2.0;
    }

    pub fn is_within_y_range(&self, top: i32, bottom: i32) -> bool {
        let player_above_gap = self.y < bottom;
        let player_below_gap = self.y > top;
        player_above_gap && player_below_gap
    }
}
