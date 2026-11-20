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

/// Width of the container required for the game.
const WINDOW_WIDTH: u16 = 66; // Real used space (64) + border (1 * 2)

/// Height of the container required for the game.
const WINDOW_HEIGHT: u16 = 27; // Board (22) + Header (2) + Separator (1) + border (1 * 2)

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
        if self.direction == MoveDirection::Left && self.start == 1 {
            self.direction = MoveDirection::Right;
        }

        if self.direction == MoveDirection::Right && self.end == WINDOW_WIDTH - 2 {
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
    /// Position of current active block that can be controlled by the player.
    current_block: Block,

    /// Position of an already placed block before current block.
    prev_block: Block,

    /// A counter represents how many blocks are already placed.
    counter: u16,
}

impl Board {
    /// A constructor. Returns a board with default value.
    fn new() -> Self {
        Board {
            // The first block has width 32 character. Start at column 0 and end at column 31
            current_block: Block::new(1, 32, MoveDirection::Right),

            // Because there's no previous block in the first row, we just initialize the previous block
            // with dummy values
            prev_block: Block::new(1, 1, MoveDirection::Right),

            // Player did not place any block yet at the beginning of the game
            counter: 0,
        }
    }

    /// A function for first-time board rendering.
    fn start(&self) -> io::Result<()> {
        // Access the standard output
        let mut stdout = io::stdout();

        // Determine which row the renderer will start to draw the pixels
        let start_row = WINDOW_HEIGHT - 2 * (self.counter + 1) - 1;

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
        let start_row = WINDOW_HEIGHT - 2 * (self.counter + 1) - 1;

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

    /// A function to handle event when player places a block.
    fn place_block(&mut self) -> io::Result<(PlaceResult, f64)> {
        // Access the standard output
        let mut stdout = io::stdout();

        // Create penalty value of this block placement
        let mut penalty = 0.0;

        // If counter = 0, any block placement is valid and allowed
        if self.counter == 0 {
            // Set previous block to current block
            self.prev_block = self.current_block;

            // Create new block with same size as the first block
            self.current_block = Block::new(1, 32, MoveDirection::Right);

            // Increase the counter
            self.counter += 1;
        } else {
            // Check whether intersection occurs
            // This condition below if there's no intersection
            if cmp::max(self.current_block.start, self.prev_block.start)
                > cmp::min(self.current_block.end, self.prev_block.end)
            {
                // Set penalty to 1
                penalty = 1.0;
                
                // Mark dead block as red

                // Get the current active row
                let start_row = WINDOW_HEIGHT - 2 * (self.counter + 1) - 1;

                // Redraw current block with red background
                for row in start_row..=(start_row + 1) {
                    for col in (self.current_block.start)..=(self.current_block.end) {
                        stdout
                            .queue(cursor::MoveTo(col, row))?
                            .queue(style::PrintStyledContent(" ".on_red()))?;
                    }
                }

                // Move cursor to (0, WINDOW_HEIGHT)
                stdout.queue(cursor::MoveTo(0, WINDOW_HEIGHT))?;

                // Flush the standard output to apply commands
                stdout.flush()?;

                // Return stop signal
                return Ok((PlaceResult::Stop, penalty));
            }

            // Get the length of intersection. Needed for creating new block and calculating penalty
            let intersection_len = cmp::min(self.current_block.end, self.prev_block.end)
                - cmp::max(self.current_block.start, self.prev_block.start)
                + 1;

            // Calculate penalty
            let prev_block_len = (self.prev_block.end - self.prev_block.start + 1) as f64;

            penalty = 1.0 - (intersection_len as f64) / prev_block_len;

            // Delete block outside the intersection
            let start_row = WINDOW_HEIGHT - 2 * (self.counter + 1) - 1;

            if self.current_block.start < self.prev_block.start {
                for row in start_row..=(start_row + 1) {
                    for col in (self.current_block.start)..(self.prev_block.start) {
                        stdout
                            .queue(cursor::MoveTo(col, row))?
                            .queue(style::Print(" "))?;
                    }
                }

                self.current_block.start = self.prev_block.start;
            } else if self.current_block.end > self.prev_block.end {
                for row in start_row..=(start_row + 1) {
                    for col in (self.prev_block.end + 1)..=(self.current_block.end) {
                        stdout
                            .queue(cursor::MoveTo(col, row))?
                            .queue(style::Print(" "))?;
                    }
                }

                self.current_block.end = self.prev_block.end;
            }

            // Save state of current block to a temporary variable
            let tmp = self.current_block;

            // Create new block, start at position 0 with length intersection_length
            self.current_block = Block::new(1, intersection_len, MoveDirection::Right);

            // Set previous block to current block
            self.prev_block = tmp;

            // Increase the counter
            self.counter += 1;
        }

        // Directly draw the new block

        // Determine which row the renderer will start to draw the pixels
        let start_row = WINDOW_HEIGHT - 2 * (self.counter + 1) - 1;

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

        Ok((PlaceResult::Continue, penalty))
    }
}

/// The structure oh player.
#[derive(Debug)]
struct Player {
    /// Current player's score.
    score: f64
}

impl Player {
    /// A contructor that returns new player with initial score = 100.
    fn new() -> Self {
        Player {
            score: 100.0
        }
    }

    /// A function for first-time player's score rendering.
    fn start(&self) -> io::Result<()> {
        // Access the standard output
        let mut stdout = io::stdout();

        let text = format!("Score: {:03.2}", self.score);

        // Print player's score
        stdout
            .queue(cursor::MoveTo(1, 1))?
            .queue(style::Print(text))?;

        stdout.flush()?;
        
        Ok(())
    }

    /// A function for calculating new score and print the updated score to terminal.
    fn update_score(&mut self, penalty: f64) -> io::Result<()> {
        // Calculate new score
        self.score -= self.score * penalty;

        // Print the updated score
        let mut stdout = io::stdout();

        let text = format!("Score: {:03.2}", self.score);

        // Print player's score
        stdout
            .queue(cursor::MoveTo(1, 1))?
            .queue(style::Print(text))?;

        stdout.flush()?;

        Ok(())
    }
}

fn print_border() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout
        .queue(cursor::MoveTo(0, 0))?
        .queue(style::Print("┌"))?
        .queue(cursor::MoveTo(WINDOW_WIDTH - 1, 0))?
        .queue(style::Print("┐"))?
        .queue(cursor::MoveTo(0, 4))?
        .queue(style::Print("├"))?
        .queue(cursor::MoveTo(WINDOW_WIDTH - 1, 4))?
        .queue(style::Print("┤"))?
        .queue(cursor::MoveTo(0, WINDOW_HEIGHT - 1))?
        .queue(style::Print("└ "))?
        .queue(cursor::MoveTo(WINDOW_WIDTH - 1, WINDOW_HEIGHT - 1))?
        .queue(style::Print("┘"))?;

    for col in 1..(WINDOW_WIDTH - 1) {
        stdout
            .queue(cursor::MoveTo(col, 0))?
            .queue(style::Print("─"))?
            .queue(cursor::MoveTo(col, 4))?
            .queue(style::Print("─"))?
            .queue(cursor::MoveTo(col, WINDOW_HEIGHT - 1))?
            .queue(style::Print("─"))?;
    }

    for row in 1..(WINDOW_HEIGHT - 1) {
        if row != 4 {
            stdout
                .queue(cursor::MoveTo(0, row))?
                .queue(style::Print("│"))?
                .queue(cursor::MoveTo(WINDOW_WIDTH - 1, row))?
                .queue(style::Print("│"))?;
        }
    }

    stdout.flush()?;

    Ok(())
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

    // Print window border
    print_border()?;

    // Render board for the first time
    board.start()?;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Enter => {
                        let (result, penalty) = board.place_block()?;

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
