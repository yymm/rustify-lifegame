mod xorshift;
mod lifegame;

use xorshift::XorShift;
use lifegame::Lifegame;

const ROW: usize = 400;
const COL: usize = 400;

#[no_mangle]
pub fn update(ptr: *mut bool, len: usize) {
    let buf: &mut [bool] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let mut game = Lifegame::new(buf, ROW, COL);
    buf.clone_from_slice(&game.next());
}

#[no_mangle]
pub fn create() -> *mut [bool; ROW * COL] {
    let tmp = [true; ROW * COL];
    let mut buf = Box::new(tmp);
    unsafe { &mut *buf }
}

#[no_mangle]
pub fn initialize(ptr: *mut bool, len: usize) {
    let buf: &mut [bool] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let mut rand = XorShift::new();
    for i in 0..len {
        buf[i] = if rand.gen_norm() < 0.5 { true } else { false };
    }
}

#[no_mangle]
pub fn get_element(i: usize, ptr: *mut bool, len: usize) -> bool {
    let buf: &[bool] = unsafe { std::slice::from_raw_parts(ptr, len) };
    if i < 0 || i > len - 1 { false } else { buf[i] }
}
