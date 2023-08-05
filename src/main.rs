use std::fs;
use std::error::Error;
use std::process;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file
    file: String,

    /// Number all output lines
    #[arg(short, long, default_value_t = false)]
    number: bool,


}

fn main() {
    let args = Args::parse();
    let contents = read_file(&args.file).unwrap_or_else(|err| {
        eprintln!("error: could not read file: {err}");
        process::exit(1);

    });
    let output = apply_options(&contents, &args);
    println!("{output}");
}

// reads text file
fn read_file(file: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(format!("{file}"))?;

    Ok(contents)
}

fn apply_options(file_contents: &str, args: &Args) -> String {
    if args.number {
        file_contents
            .lines()
            .enumerate()
            .map(|(i, line)| format!("{i}  {line}\n"))
            .collect()
           
    } else {
        file_contents.to_string()
    }
}


