//! # Shift the Block!
//! 
//! Version 0.1.0
//! 
//! A simple terminal game made by a solo inexperienced programmer, Andhika Rahman, in His free time.
//! 
//! This is the original source code of Shift the Block! Game.

use std::{cmp, io::{self, Write}, thread::sleep, time::Duration};

use crossterm::{ExecutableCommand, QueueableCommand, cursor, event::{self, Event, KeyCode}, style::{self, Stylize}, terminal};

/// Width of the window required for the game.
const WIDTH: u16 = 64;

/// Height of the window required for the game.
const HEIGHT: u16 = 27;     // Board (24) + Header (2) + Separator (1)

/// Define directions of block movement.
#[derive(PartialEq, Debug, Clone, Copy)]
enum MoveDirection {
    Left,
    Right
}

/// Define the possible result when player placed a block
#[derive(Debug, PartialEq)]
enum PlaceResult {
    Continue,
    Stop
}

/// The structure of a single block.
#[derive(Debug, Clone, Copy)]
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
        }
    }

    /// A function to do shift operation to the active block.
    fn shift_block(&mut self) {
        self.current_block.shift();
    }

    /// A function to render the board to the terminal.
    // The correct order of execution: process input -> update state (shift the block) -> render
    fn render(&mut self) -> io::Result<()> {
        // We use Lazy Execution in crossterm for performance purpose

        // Access the stdout
        let mut stdout = io::stdout();

        // Which row the renderer will start to draw the pixels
        let start_row = HEIGHT - 2 * (self.counter + 1);

        // The new, just placed, block has different render handling
        // And also for block which has one character width (start = end)
        if self.current_block.is_just_placed || self.current_block.start == self.current_block.end {
            for row in start_row..=(start_row + 1) {
                for col in (self.current_block.start)..=(self.current_block.end) {
                    stdout
                        .queue(cursor::MoveTo(col, row))?
                        .queue(style::PrintStyledContent(" ".on_white()))?;
                }
            }

            // Flag current block has been rendered before
            // This is fine for already rendered, 1-character width, block
            self.current_block.is_just_placed = false;
        } else {
            // We use algorithm called "Lazy Rendering" (i named it by myself 😁)
            // The idea is simple: We don't redraw all blocks from scratch in every frame
            // instead, we just render the current active block. Because all previous blocks
            // won't move again in the future. We also optimize again the rendering process, by
            // just redrawing the previous column and the new one, we can ignore pixel between them.
            // However, this only can be applicated for a block which has width more than one character
            // ie. start != end
            let (prev_col, new_col) = match self.current_block.direction {
                MoveDirection::Left => (
                    self.current_block.end + 1,
                    self.current_block.start
                ),
                MoveDirection::Right => (
                    self.current_block.start - 1,
                    self.current_block.end,
                )
            };

            for row in start_row..=(start_row + 1) {
                stdout
                    .queue(cursor::MoveTo(prev_col, row))?
                    .queue(style::Print(" "))?
                    .queue(cursor::MoveTo(new_col, row))?
                    .queue(style::PrintStyledContent(" ".on_white()))?;
            }
        }

        // Flush the stdout to apply the queued commands
        stdout.flush()?;

        Ok(())
    }

    /// A function to handle event when player places a block
    fn place_block(&mut self) -> PlaceResult {
        // If counter = 0, any block placement is valid and allowed
        if self.counter == 0 {
            self.counter += 1;

            // Set previous block to current block
            self.prev_block = self.current_block;

            // Create new block with same size as the first block
            self.current_block = Block::new(0, WIDTH / 2 - 1, MoveDirection::Right);

            return PlaceResult::Continue;
        } else {
            self.counter += 1;

            // Check whether intersection occurs
            if cmp::max(self.current_block.start, self.prev_block.start) > cmp::min(self.current_block.end, self.prev_block.end) {
                return PlaceResult::Stop;
            }

            // Get the length of intersection
            let intersection_len = cmp::min(self.current_block.end, self.prev_block.end) - 
                cmp::max(self.current_block.start, self.prev_block.start) + 1;

            // Save state of current block to a temporary variable
            let tmp = self.current_block;

            // Create new block, start at position 0 with length intersection_length
            self.current_block = Block::new(
                0,
                intersection_len - 1,
                MoveDirection::Right
            );

            // Set previous block to current block
            self.prev_block = tmp;

            return PlaceResult::Continue;
        }
    }
}

fn main() -> io::Result<()> {
    // Let's test the board!

    // Define frame delay
    let delay: u64 = 512;

    // Create new board
    let mut board = Board::new();

    // Enable raw mode
    terminal::enable_raw_mode()?;

    // Clear screen
    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    // Render board for the first time
    board.render()?;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Enter => {
                        let result = board.place_block();

                        if result == PlaceResult::Stop {
                            break;
                        }
                    },
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        board.shift_block();

        board.render()?;

        sleep(Duration::from_millis(delay));
    }

    // Disable raw mode
    terminal::disable_raw_mode()?;

    Ok(())
}
