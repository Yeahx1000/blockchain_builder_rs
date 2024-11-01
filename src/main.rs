mod blockchain;
mod cli;

use blockchain::Blockchain;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let mut blockchain =
        Blockchain::load_from_file("blockchain.json").unwrap_or_else(|_| Blockchain::new(2));

    match &cli.command {
        Commands::Init => {
            blockchain = Blockchain::new(2);
            println!("Blockchain initialized with genesis block");
        }
        Commands::Add { data } => {
            blockchain.add_block(data.clone());
            println!("Added new block with data: {}", data);
        }
        Commands::View => {
            for block in &blockchain.chain {
                println!("{:?}", block);
            }
        }
        Commands::Validate => {
            let is_valid = blockchain.is_valid_chain();
            println!("Is blockchain valid? {}", is_valid);
        }
        Commands::SetDifficulty { level } => {
            blockchain.difficulty = *level;
            println!("Set mining difficulty to {}", level);
        }
    }

    blockchain
        .save_to_file("blockchain.json")
        .expect("Failed to save blockchain");
}
