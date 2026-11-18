//! # Shift the Block!
//! 
//! Version 0.1.0
//! 
//! A simple terminal game made by a solo inexperienced programmer, Andhika Rahman, in His free time.
//! 
//! This is the original source code of Shift the Block! Game.

/// Width of the window required for the game.
const WIDTH: u16 = 64;

/// Height of the window required for the game.
const HEIGHT: u16 = 27;     // Board (24) + Header (2) + Separator (1)

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
    direction   : MoveDirection,

    /// A flag to check whether a block is just placed on the board. 
    /// We need this because a new block has different rendering pipeline 
    /// with older one.
    is_just_placed: bool
}

impl Block {
    /// A constructor. Returns a new block with the start value, end value, 
    /// and direction specified via the parameters.
    fn new(start: u16, end: u16, direction: MoveDirection) -> Self {
        Block {
            start,
            end,
            direction,
            is_just_placed: true
        }
    }

    /// A method that handles the block movement. In each shift, the block will 
    /// move one step either to the left or right, depends on its current direction. 
    /// This operation also depends on window width, because the block must know it for 
    /// direction switching.
    fn shift(&mut self) {
        // Switch the direction when touching left and right window edge
        if self.direction == MoveDirection::Left && self.start == 0 {
            self.direction = MoveDirection::Right;
        }

        if self.direction == MoveDirection::Right && self.end == WIDTH - 1 {
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

/// The structure of a board in the game.
#[derive(Debug)]
struct Board {
    /// Current active block that can be controlled by the player.
    current_block: Block,

    /// An already placed block before current block.
    prev_block: Block,

    /// A counter represents how many blocks are already placed.
    counter: u16,

    /// A flag to check whether a block is just placed on the board. 
    /// We need this because a new block has different rendering pipeline 
    /// with older one.
    is_just_placed_new_block: bool
}

impl Board {
    /// A constructor. Returns a board with default value.
    fn new() -> Self {
        Board {
            // The first block has width half of window width (or 32 character in this case)
            // Start at column 0 and end at column WIDTH / 2 - 1 (start: 0, end: 31)
            current_block: Block::new(0, WIDTH / 2 - 1, MoveDirection::Right),

            // Because there's no previous block in the first row, we just initialize the previous block
            // with dummy values
            prev_block: Block::new(0, 0, MoveDirection::Right),

            // Player did not place any block yet at the beginning of the game
            counter: 0,

            // Yes, we just placed the first block
            is_just_placed_new_block: true
        }
    }

    /// A function to do shift operation to the active block.
    fn shift_block(&mut self) {
        self.current_block.shift();
    }
}

fn main() {
    //
}
