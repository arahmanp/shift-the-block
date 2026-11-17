/// Define directions of block movement
#[derive(PartialEq, Debug)]
enum MoveDirection {
    Left,
    Right
}

/// The structure of a single block
#[derive(Debug)]
struct Block {
    start       : u16,
    end         : u16,
    direction   : MoveDirection
}

fn main() {
    println!("Hello, world!");
}
