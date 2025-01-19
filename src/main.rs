use clap::Parser;

#[derive(Parser)]
#[command(name = "mim")]
#[command(about = "Prints all command line arguments")]
struct Cli {
    /// The words to be printed
    #[arg(required = true, num_args = 1..)]
    words: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    
    // Print all words passed as arguments
    for word in &cli.words {
        println!("{}", word);
    }
}