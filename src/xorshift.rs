use std::u32;

pub struct XorShift {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XorShift {
    pub fn new() -> XorShift {
        XorShift {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123,
        }
    }

    pub fn gen(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w = self.w;
        self.w = (w ^ (w << 19)) ^ (t ^ (t >> 8));
        self.w
    }

    pub fn gen_norm(&mut self) -> f64 {
        self.gen() as f64 / u32::MAX as f64
    }
}
