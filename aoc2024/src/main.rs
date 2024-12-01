use clap::{Parser, Subcommand};

mod days;
mod utils;

#[derive(Parser)]
#[command(name = "Advent of Code 2024")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = "1.0")]
#[command(about = "Solve Advent of Code 2024 challenges", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the solution for a specific day and step
    Run {
        /// Day to run (1-25)
        #[arg()]
        day: u8,

        /// Step to run (1 or 2)
        #[arg()]
        step: u8,

        /// Optional input file
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Test all solutions
    Test,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { day, step, input } => {
            if let Some((step1, step2)) = days::get_day_solution(*day) {
                let input = match &input {
                    Some(path) => path.clone(),
                    None => format!("inputs/day{:02}.txt", day),
                };
                let data = utils::read_input(input.as_str()).expect("Failed to read input file");

                let result = match step {
                    1 => step1(&data),
                    2 => step2(&data),
                    _ => return,
                };

                println!("Result (Day {}, Step {}): {}", day, step, result);
            } else {
                eprintln!("Solution for day {} is not implemented yet!", day);
            }
        }
        Commands::Test => {
            println!("Running tests for all days and steps...");
            // Implement test runner
        }
    }
}
