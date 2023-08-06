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

    /// Supress repeated empty output lines
    #[arg(short, long = "squeeze-blank", default_value_t = false)]
    squeeze_blank: bool,

    /// Display $ at the end of each line
    #[arg(short = 'E', long = "show-ends", default_value_t = false)]
    show_ends: bool,

}

fn main() {
    let args = Args::parse();
    let mut contents = read_file(&args.file).unwrap_or_else(|err| {
        eprintln!("error: could not read file: {err}");
        process::exit(1);

    });
    apply_options(&mut contents, &args);
    print!("{contents}");
}

// reads text file
fn read_file(file: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(format!("{file}"))?;

    Ok(contents)
}

fn apply_options<'a>(file_contents: &'a mut String, args: &Args) -> &'a str {
    if args.number {
        *file_contents = number(file_contents);
           
    } 
    if args.squeeze_blank {
        *file_contents = squeeze_blank(file_contents);
    }
    if args.show_ends {
        *file_contents = show_ends(file_contents);
    }
        file_contents
    
}

fn number(text: &str) -> String {
    text
        .lines()
        .enumerate()
        .map(|(i, line)| format!("     {}  {line}\n", i+1))
        .collect()
}

fn squeeze_blank(text: &str) -> String {
    todo!();
}

fn show_ends(text: &str) -> String {
    text
        .lines()
        .map(|line| format!("{line}$\n"))
        .collect()
}

