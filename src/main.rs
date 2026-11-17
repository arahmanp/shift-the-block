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

impl Block {
    /// A constructor. Returns a new block with the start value, end value, 
    /// and direction specified via the parameters.
    fn new(start: u16, end: u16, direction: MoveDirection) -> Self {
        Block {
            start,
            end,
            direction
        }
    }

    /// A method that handles the block movement. In each shift, the block will 
    /// move one step either to the left or right, depends on its current direction. 
    /// This operation also depends on window width, because the block must know it for 
    /// direction switching.
    fn shift(&mut self, window_width: u16) {
        // Switch the direction when touching left and right window edge
        if self.direction == MoveDirection::Left && self.start == 0 {
            self.direction = MoveDirection::Right;
        }

        if self.direction == MoveDirection::Right && self.end == window_width {
            self.direction = MoveDirection::Left;
        }

        // Shift the block based on its direction
        match self.direction {
            MoveDirection::Left => {
                self.start -= 1;
                self.end -= 1;
            },
            MoveDirection::Right => {
                self.start += 1;
                self.end += 1;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
