use json_parser::{JSONError, parse_json_file, serialize_jsonvalue};
use std::fs;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name="json_parser",
    version,
    author,
    about="A Rust project that implements a JSON parser using the Pest parsing library.",
    long_about=None
)]

struct Cli{
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        input: PathBuf,

        #[arg(short, long)]
        out: Option<PathBuf>,

        #[arg(short, long)]
        ast: bool,
    },

    Credits,
}

fn main() -> Result<(), JSONError> {

    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { input, out, ast } => {
            let text = fs::read_to_string(&input)?;
            let value = parse_json_file(&text)?;

            if ast {
                println!("AST:\n{value:#?}");
            }

            match out {
                Some(path) => {
                    let serialized = serialize_jsonvalue(&value);
                    fs::write(&path, serialized)?;
                    println!("Wrote serialized output to {}", path.display());
                }
                None => {
                    
                }
            }
        }

        Commands::Credits => {
            let name = env!("CARGO_PKG_NAME");
            let ver = env!("CARGO_PKG_VERSION");
            let authors = env!("CARGO_PKG_AUTHORS");
            println!("{name} v{ver}");
            println!("Authors: {authors}");  
        }
    }

    // let input = fs::read_to_string("./src/input.json")?;
    // println!("{input}");

    // let json_value = parse_json_file(&input)?;
    // println!("Ast from input:\n{json_value:?}");

    // let serialized = serialize_jsonvalue(&json_value);
    // fs::write("./src/output.json", serialized)?;

    Ok(())
}
