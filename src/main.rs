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
    /// Starting column position of a block.
    start       : u16,

    /// Column position where the block ends.
    end         : u16,

    /// Current block movement direction.
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
    // Test the block system.

    // Define window width for testing environment.
    let window_width = 5_u16;




    // Test 1: Test the contructor.

    // Create new block with these initial value:
    // - start: 0
    // - end: 2
    // - direction: Right
    let mut block = Block::new(0, 2, MoveDirection::Right);

    // Print the initial state of the block.
    // Expected output:
    // - start: 0
    // - end: 2
    // - direction: Right
    println!("{:#?}", block);




    // Test 2: Test the shifting system.

    // Shift the block.
    // Expected state:
    // - start: 1
    // - end: 3
    // - direction: Right
    block.shift(window_width);

    // Print the updated state of the block.
    // Expected output:
    // - start: 1
    // - end: 3
    // - direction: Right
    println!("{:#?}", block);




    // Test 3: Test the direction switching mechanism.

    // Shift the block until touching the window edge.
    // Expected state:
    // - start: 3
    // - end: 5
    // - direction: Right
    block.shift(window_width);
    block.shift(window_width);

    // Print the updated state of the block.
    // Expected output:
    // - start: 3
    // - end: 5
    // - direction: Right
    println!("{:#?}", block);

    // Shift once to check the direction switching.
    // Expected state:
    // - start: 2
    // - end: 4
    // - direction: Left
    block.shift(window_width);

    // Print the updated state of the block.
    // Expected output:
    // - start: 2
    // - end: 4
    // - direction: Left
    println!("{:#?}", block);




    // Test summary:
    // - Test 1 (Passed ✅)
    // - Test 2 (Passed ✅)
    // - Test 3 (Passed ✅)
}
