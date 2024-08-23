use rig::providers::openai;
use rig::completion::Prompt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Player {
    X,
    O,
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Board {
    cells: [Player; 9],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [Player::Empty; 9],
        }
    }

    fn make_move(&mut self, position: usize, player: Player) -> Result<(), String> {
        if position < 1 || position > 9 {
            return Err("Invalid position. Choose a number between 1 and 9.".to_string());
        }
        let index = position - 1;
        if self.cells[index] != Player::Empty {
            return Err("This cell is already occupied.".to_string());
        }
        self.cells[index] = player;
        Ok(())
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell != Player::Empty)
    }

    fn has_winner(&self) -> Option<Player> {
        const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],            // Diagonals
        ];

        for combo in WINNING_COMBINATIONS.iter() {
            if self.cells[combo[0]] != Player::Empty
                && self.cells[combo[0]] == self.cells[combo[1]]
                && self.cells[combo[1]] == self.cells[combo[2]]
            {
                return Some(self.cells[combo[0]]);
            }
        }
        None
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str("┌───┬───┬───┐\n");
        for i in 0..3 {
            result.push_str("│");
            for j in 0..3 {
                let index = i * 3 + j;
                let symbol = match self.cells[index] {
                    Player::X => " X ".to_string(),
                    Player::O => " O ".to_string(),
                    Player::Empty => format!(" {} ", index + 1),
                };
                result.push_str(&symbol);
                if j < 2 {
                    result.push_str("│");
                }
            }
            result.push_str("│\n");
            if i < 2 {
                result.push_str("├───┼───┼───┤\n");
            }
        }
        result.push_str("└───┴───┴───┘\n");
        result
    }
}

fn parse_ai_response(response: &str) -> Result<usize, String> {
    // First, try to parse the entire response as a number
    if let Ok(num) = response.trim().parse::<usize>() {
        return Ok(num);
    }

    // If that fails, try to find the first number in the response
    for word in response.split_whitespace() {
        if let Ok(num) = word.parse::<usize>() {
            return Ok(num);
        }
    }

    // If we still can't find a number, return an error
    Err("Could not find a valid move in the AI's response".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let openai_client = openai::Client::from_env();
    let ai_player = openai_client.model("gpt-3.5-turbo").build();

    let mut board = Board::new();
    let mut current_player = Player::X;

    println!("Welcome to Tic-Tac-Toe! You are X, and the AI is O.");
    println!("Enter a number from 1-9 to make your move.");

    loop {
        println!("\nCurrent board:");
        println!("{}", board.to_string());

        match current_player {
            Player::X => {
                print!("Your move (X): ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let position: usize = input.trim().parse()?;
                if let Err(e) = board.make_move(position, Player::X) {
                    println!("Error: {}. Try again.", e);
                    continue;
                }
            }
            Player::O => {
                println!("AI is thinking...");
                let prompt = format!(
                    "You are playing Tic-Tac-Toe as O. Here's the current board state:\n{}\nWhat's your next move? Respond with just the number (1-9) of the position you want to play.",
                    board.to_string()
                );
                let ai_response = ai_player.prompt(&prompt).await?;
                let position = parse_ai_response(&ai_response);
                match position {
                    Ok(pos) => {
                        if let Err(e) = board.make_move(pos, Player::O) {
                            println!("AI made an invalid move: {}. It forfeits its turn.", e);
                            continue;
                        }
                        println!("AI chose position {}", pos);
                    }
                    Err(e) => {
                        println!("Failed to parse AI's move: {}. AI forfeits its turn.", e);
                        continue;
                    }
                }
            }
            Player::Empty => unreachable!(),
        }

        if let Some(winner) = board.has_winner() {
            println!("\nFinal board:");
            println!("{}", board.to_string());
            println!("Player {:?} wins!", winner);
            break;
        }

        if board.is_full() {
            println!("\nFinal board:");
            println!("{}", board.to_string());
            println!("It's a draw!");
            break;
        }

        current_player = match current_player {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::Empty => unreachable!(),
        };
    }

    Ok(())
}