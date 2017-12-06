pub struct Lifegame {
    row: usize,
    col: usize,
    pub room: Vec<bool>,
}

impl Lifegame {
    pub fn new(buf: &mut [bool], row: usize, col: usize) -> Lifegame {
        Lifegame {
            row: row,
            col: col,
            room: buf.to_vec(),
        }
    }

    pub fn next(&mut self) -> &[bool] {
        self.room[0] = false;
        &self.room
    }
}
