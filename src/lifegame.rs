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

    pub fn next(&self) -> Vec<bool> {
        self.room
            .iter()
            .enumerate()
            .map(|(n, _)| self.get_next_state(n))
            .collect()
    }

    fn get_next_state(&self, n: usize) -> bool {
        let y = (n / self.col) as usize;
        let x = n - y * self.col;
        println!("{} {} {}", x, y, n);
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

    fn get_cell(&self, x: usize, y: usize) -> bool {
        println!("{}, {}", x, y);
        true
    }
}

#[test]
fn test_reproduction() {
    let buf = [false, false, false, false, false, false, false, false, false];
    let game = Lifegame::new(&mut buf, 3, 3);
    let room = game.next();
    assert_eq!(&room,
               [false, false, false, false, false, false, false, false, false]);
}

#[test]
fn test_live() {}

#[test]
fn test_underpopulation() {}

#[test]
fn test_overpupulation() {}
