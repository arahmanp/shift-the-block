//! # Shift the Block!
//! 
//! Version 0.1.0
//! 
//! A simple terminal game made by a solo inexperienced programmer, Andhika Rahman, in His free time.
//! 
//! This is the original source code of Shift the Block! Game.

/// Define directions of block movement.
#[derive(PartialEq, Debug)]
enum MoveDirection {
    Left,
    Right
}

/// The structure of a single block.
#[derive(Debug)]
struct Block {
    start       : u16,
    end         : u16,
    direction   : MoveDirection
}

fn main() {
    println!("Hello, world!");
}
