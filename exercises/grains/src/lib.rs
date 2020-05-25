pub fn square(s: u32) -> u64 {
    if s <= 0 || s > 64 {
        panic!("Square must be between 1 and 64");
    } else {
        let b:u64 = 2;
        b.pow(s-1)
    }
}

pub fn total() -> u64 {
    let b:u64 = 2;
    let board: Vec<u64> = (0..=63).map(|x| b.pow(x)).collect();
    board.iter().sum()
}
