use super::bitvec::Bitvec;

#[derive(Debug)]
pub struct LightGrid {
    x_size: u32,
    y_size: u32,

    data: Bitvec,
}

impl LightGrid {
    pub fn new(x_size: u32, y_size: u32) -> LightGrid {
        LightGrid {
            x_size,
            y_size,
            data: Bitvec::new(x_size * y_size),
        }
    }

    pub fn set(&mut self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
        for y in from_y..to_y + 1 {
            for x in from_x..to_x + 1 {
                let z = y * self.x_size + x;

                self.data.set(z)
            }
        }
    }

    pub fn unset(&mut self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
        for y in from_y..to_y + 1 {
            for x in from_x..to_x + 1 {
                let z = y * self.x_size + x;

                self.data.unset(z)
            }
        }
    }

    pub fn toggle(&mut self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
        for y in from_y..to_y + 1 {
            for x in from_x..to_x + 1 {
                let z = y * self.x_size + x;

                self.data.toggle(z)
            }
        }
    }

    pub fn on_count(&self) -> i32 {
        let mut sum = 0;
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let z = y * self.x_size + x;

                if self.data.is_set(z) {
                    sum += 1;
                }
            }
        }

        sum
    }
}
