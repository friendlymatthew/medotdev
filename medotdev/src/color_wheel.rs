use rand::distributions::{Distribution, Uniform};

const COLORS: [&'static str; 4] = ["#2e2459", "#ffc832", "#0b7261", "#a72145"];

pub struct ColorWheel {
    prev: Option<usize>,
    uniform: Uniform<usize>,
}

impl ColorWheel {
    pub fn new() -> ColorWheel {
        ColorWheel {
            prev: None,
            uniform: Uniform::from(0..COLORS.len()),
        }
    }

    pub fn spit(&mut self) -> &'static str {
        let mut rng = rand::thread_rng();
        let index = match self.prev {
            Some(prev_idx) => {
                let mut new_index = self.uniform.sample(&mut rng);
                if new_index >= prev_idx {
                    new_index = (new_index + 1) % COLORS.len();
                }

                new_index
            }
            None => self.uniform.sample(&mut rng),
        };

        self.prev = Some(index);
        COLORS[index]
    }
}
