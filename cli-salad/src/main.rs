use clap::Parser;
use cli_salad::create_fruit_salad_with_fruits;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    /// Number of fruits to include in the salad
    #[clap(short, long)]
    number: usize,

    /// List of fruits to include in the salad
    #[clap(short, long, value_parser, required = true)]
    fruits: Vec<String>,
}

fn main() {
    let opts = Opts::parse();

    match create_fruit_salad_with_fruits(opts.number, &opts.fruits) {
        Ok(mut salad) => {
            salad.sort();
            println!(
                "Created Fruit salad with {} fruits (alphabetical order): {:?}",
                opts.number, salad
            );
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
