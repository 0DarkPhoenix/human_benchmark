use anyhow::Result;
use std::io::{self, Write};

mod benchmarks;
mod components;

use benchmarks::*;
pub use components::utils;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Human Benchmark Test Suite ===\n");

    loop {
        display_menu();

        let choice = get_user_input("Enter your choice: ")?;
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "1" | "reaction" | "reaction-time" => {
                println!("Running Reaction Time test...");
                reaction_time::run().await?;
            }
            "2" | "typing" => {
                println!("Running Typing test...");
                typing::run().await?;
            }
            "3" | "sequence" | "sequence-memory" => {
                let max_level = get_numeric_input("Enter max level (default: 10): ", 10)?;
                println!("Running Sequence Memory test up to level {}...", max_level);
                sequence_memory::run(max_level).await?;
            }
            "4" | "aim" | "aim-trainer" => {
                println!("Running Aim Trainer...");
                aim_trainer::run().await?;
            }
            "5" | "number" | "number-memory" => {
                let max_digits = get_numeric_input("Enter max digits (default: 10): ", 10)?;
                println!("Running Number Memory test up to {} digits...", max_digits);
                number_memory::run(max_digits).await?;
            }
            "6" | "chimp" | "chimp-test" => {
                println!("Running Chimp Test...");
                chimp_test::run().await?;
            }
            "7" | "verbal" | "verbal-memory" => {
                println!("Running Verbal Memory...");
                verbal_memory::run().await?;
            }
            "8" | "visual" | "visual-memory" => {
                println!("Running Visual Memory test...");
                visual_memory::run().await?;
            }
            "9" | "quit" | "exit" | "q" => {
                println!("Goodbye!");
                break;
            }
            "" => {
                // Empty input, just continue
                continue;
            }
            _ => {
                println!("Invalid choice. Please try again.\n");
                continue;
            }
        }

        println!("\n{}\n", "=".repeat(50));
    }

    Ok(())
}

fn display_menu() {
    println!("Available Tests:");
    println!("  1. Reaction Time    - Test your reaction time");
    println!("  2. Typing          - Test your typing speed");
    println!("  3. Sequence Memory - Test your sequence memory");
    println!("  4. Aim Trainer     - Test your aim accuracy");
    println!("  5. Number Memory   - Test your number memory");
    println!("  6. Chimp Test      - Test your inner chimp memory");
    println!("  7. Verbal Memory   - Test your verbal memory");
    println!("  8. Visual Memory   - Test your visual memory");
    println!("  9. Quit            - Exit the program");
    println!();
}

fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn get_numeric_input(prompt: &str, default: u32) -> Result<u32> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim();
    if input.is_empty() {
        return Ok(default);
    }

    match input.parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => {
            println!("Invalid number, using default value: {}", default);
            Ok(default)
        }
    }
}
