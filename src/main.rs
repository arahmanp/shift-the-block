//! # Shift the Block!
//!
//! Version 0.1.0
//!
//! A simple terminal game made by a solo inexperienced programmer, Andhika Rahman, in His free time.
//!
//! This is the original source code of Shift the Block! Game.

use std::{
    cmp,
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    event::{self, Event, KeyCode},
    style::{self, Stylize},
    terminal,
};

/// Width of the window required for the game.
const WIDTH: u16 = 64;

/// Height of the window required for the game.
const HEIGHT: u16 = 25; // Board (22) + Header (2) + Separator (1)

/// Define directions of block movement.
#[derive(PartialEq, Debug, Clone, Copy)]
enum MoveDirection {
    Left,
    Right,
}

/// Define the possible result when player placed a block
#[derive(Debug, PartialEq)]
enum PlaceResult {
    Continue,
    Stop,
}

/// The structure of a single block.
#[derive(Debug, Clone, Copy)]
struct Block {
    /// Starting column position of a block.
    start: u16,

    /// Column position where the block ends.
    end: u16,

    /// Current block movement direction.
    direction: MoveDirection,
}

impl Block {
    /// A constructor. Returns a new block with the start value, end value,
    /// and direction specified via the parameters.
    fn new(start: u16, end: u16, direction: MoveDirection) -> Self {
        Block {
            start,
            end,
            direction,
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
            }
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
            // The first block has width 32 character. Start at column 0 and end at column 31
            current_block: Block::new(0, 31, MoveDirection::Right),

            // Because there's no previous block in the first row, we just initialize the previous block
            // with dummy values
            prev_block: Block::new(0, 0, MoveDirection::Right),

            // Player did not place any block yet at the beginning of the game
            counter: 0,
        }
    }

    /// A temporary function for first-time rendering.
    fn start(&self) -> io::Result<()> {
        // Access the standard output
        let mut stdout = io::stdout();

        // Determine which row the renderer will start to draw the pixels
        let start_row = HEIGHT - 2 * (self.counter + 1);

        // Start draw the first block
        for row in start_row..=(start_row + 1) {
            for col in (self.current_block.start)..=(self.current_block.end) {
                stdout
                    .queue(cursor::MoveTo(col, row))?
                    .queue(style::PrintStyledContent(" ".on_white()))?;
            }
        }

        // Flush the standard output to apply commands
        stdout.flush()?;

        Ok(())
    }

    /// A function to do shift operation to the active block.
    fn shift_block(&mut self) -> io::Result<()> {
        // Shift current block
        self.current_block.shift();

        // Directly render new state

        // Access the stdout
        let mut stdout = io::stdout();

        // Which row the renderer will start to draw the pixels
        let start_row = HEIGHT - 2 * (self.counter + 1);

        // We use algorithm called "Lazy Rendering" (i named it by myself 😁)
        // The idea is simple: We don't redraw all blocks from scratch in every frame
        // instead, we just render the current active block. Because all previous blocks
        // won't move again in the future. We can also optimize the rendering process, by
        // just redrawing the previous column and the new one, we can ignore pixel between them.
        // However, a block must be fully drawn first to apply this rendering method
        let (prev_col, new_col) = match self.current_block.direction {
            MoveDirection::Left => (self.current_block.end + 1, self.current_block.start),
            MoveDirection::Right => (self.current_block.start - 1, self.current_block.end),
        };

        for row in start_row..=(start_row + 1) {
            stdout
                .queue(cursor::MoveTo(prev_col, row))?
                .queue(style::Print(" "))?
                .queue(cursor::MoveTo(new_col, row))?
                .queue(style::PrintStyledContent(" ".on_white()))?;
        }

        // Everytime rendering process done, move cursor to the inside of current block
        // Look is important too!
        stdout.queue(cursor::MoveTo(self.current_block.start, start_row))?;

        // Flush the stdout to apply the queued commands
        stdout.flush()?;

        Ok(())
    }

    /// A function to handle event when player places a block
    fn place_block(&mut self) -> io::Result<PlaceResult> {
        // If counter = 0, any block placement is valid and allowed
        if self.counter == 0 {
            self.counter += 1;

            // Set previous block to current block
            self.prev_block = self.current_block;

            // Create new block with same size as the first block
            self.current_block = Block::new(0, 31, MoveDirection::Right);
        } else {
            // Check whether intersection occurs
            if cmp::max(self.current_block.start, self.prev_block.start)
                > cmp::min(self.current_block.end, self.prev_block.end)
            {
                // Mark dead block as red

                // Access the standard output
                let mut stdout = io::stdout();

                // Get the current active row
                let start_row = HEIGHT - 2 * (self.counter + 1);

                // Redraw current block with red background
                for row in start_row..=(start_row + 1) {
                    for col in (self.current_block.start)..=(self.current_block.end) {
                        stdout
                            .queue(cursor::MoveTo(col, row))?
                            .queue(style::PrintStyledContent(" ".on_red()))?;
                    }
                }

                // Move cursor to (0, 0)
                stdout.queue(cursor::MoveTo(0, 0))?;

                // Flush the standard output to apply commands
                stdout.flush()?;

                // Return stop signal
                return Ok(PlaceResult::Stop);
            }

            self.counter += 1;

            // Get the length of intersection
            let intersection_len = cmp::min(self.current_block.end, self.prev_block.end)
                - cmp::max(self.current_block.start, self.prev_block.start)
                + 1;

            // Save state of current block to a temporary variable
            let tmp = self.current_block;

            // Create new block, start at position 0 with length intersection_length
            self.current_block = Block::new(0, intersection_len - 1, MoveDirection::Right);

            // Set previous block to current block
            self.prev_block = tmp;
        }

        // Directly draw the new block

        // Access the standard output
        let mut stdout = io::stdout();

        // Determine which row the renderer will start to draw the pixels
        let start_row = HEIGHT - 2 * (self.counter + 1);

        // Start draw new block
        for row in start_row..=(start_row + 1) {
            for col in (self.current_block.start)..=(self.current_block.end) {
                stdout
                    .queue(cursor::MoveTo(col, row))?
                    .queue(style::PrintStyledContent(" ".on_white()))?;
            }
        }

        // Flush the standard output to apply commands
        stdout.flush()?;

        Ok(PlaceResult::Continue)
    }
}

fn main() -> io::Result<()> {
    // Let's test the board!

    // Define frame delay
    let mut delay: u64 = 512;

    // Create new board
    let mut board = Board::new();

    // Enable raw mode
    terminal::enable_raw_mode()?;

    // Clear screen
    io::stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    // Render board for the first time
    board.start()?;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Enter => {
                        let result = board.place_block()?;

                        if result == PlaceResult::Stop {
                            sleep(Duration::from_secs(5));
                            break;
                        }

                        delay /= 2;
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        board.shift_block()?;

        sleep(Duration::from_millis(delay));
    }

    // Disable raw mode
    terminal::disable_raw_mode()?;

    // Clear screen and move cursor to (0, 0) before closing the game
    io::stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::MoveTo(0, 0))?;

    // Test summary:
    // 1. Board initialization (Passed ✅)
    // 2. Block shifting (Passed ✅)
    // 3. Board rendering (Passed ✅)
    // 4. Block placement (Passed ✅)

    Ok(())
}
