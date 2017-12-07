mod lifegame;

use lifegame::Lifegame;

fn main() {
    let mut buf = [true, false, true, true, false, false, false, false, false];
    println!("{:?}", buf);
    let game = Lifegame::new(&mut buf, 3, 3);
    let room = game.next();
    println!("{:?}", room);

    println!("Hello, world!");
}
