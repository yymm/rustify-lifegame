pub struct Lifegame {
    row: i32,
    col: i32,
    pub room: Vec<bool>,
}

impl Lifegame {
    pub fn new(buf: &mut [bool], row: usize, col: usize) -> Lifegame {
        Lifegame {
            row: row as i32,
            col: col as i32,
            room: buf.to_vec(),
        }
    }

    pub fn next(&self) -> Vec<bool> {
        self.room
            .iter()
            .enumerate()
            .map(|(n, _)| self.get_next_state(n))
            .collect()
    }

    fn get_next_state(&self, n: usize) -> bool {
        let y = n as i32 / self.col;
        let x = n as i32 - y * self.col;
        let num_of_living = [(x - 1, y + 1),
                             (x, y + 1),
                             (x + 1, y + 1),
                             (x - 1, y),
                             (x + 1, y),
                             (x - 1, y - 1),
                             (x, y - 1),
                             (x + 1, y - 1)]
            .iter()
            .fold(0,
                  |sum, &(x, y)| if self.get_cell(x, y) { sum + 1 } else { sum });
        match num_of_living {
            3 => true,
            2 if self.room[n] => true,
            _ => false,
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.col || y >= self.row {
            false
        } else {
            let n = y * self.col + x;
            self.room[n as usize]
        }
    }
}

#[test]
fn test_reproduction() {
    let mut buf = [true, true, false, true, false, false, false, false, false];
    let game = Lifegame::new(&mut buf, 3, 3);
    let room = game.next();
    assert_eq!(&room,
               &[true, true, false, true, true, false, false, false, false]);
}

#[test]
fn test_live() {
    let mut buf = [false, false, false, false, false, true, true, false, false, true, true, false,
                   false, false, false, false];
    let game = Lifegame::new(&mut buf, 4, 4);
    let room = game.next();
    assert_eq!(&room,
               &[false, false, false, false, false, true, true, false, false, true, true, false,
                 false, false, false, false]);
}

#[test]
fn test_underpopulation() {
    let mut buf = [false, false, false, false, true, true, false, false, false];
    let game = Lifegame::new(&mut buf, 3, 3);
    let room = game.next();
    assert_eq!(&room,
               &[false, false, false, false, false, false, false, false, false]);
}

#[test]
fn test_overpupulation() {
    let mut buf = [true, true, true, true, true, false, false, false, false];
    let game = Lifegame::new(&mut buf, 3, 3);
    let room = game.next();
    assert_eq!(&room,
               &[true, false, true, true, false, true, false, false, false]);
}
