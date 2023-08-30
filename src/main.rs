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

    /// Numbers nonempty output lines (overrides -n)
    #[arg(short = 'b', long = "number-nonblank", default_value_t = false)]
    number_nonblank: bool,

    /// Supress repeated empty output lines
    #[arg(short, long = "squeeze-blank", default_value_t = false)]
    squeeze_blank: bool,

    /// Mark the end of each line with $
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

fn read_file(file: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(format!("{file}"))?;

    Ok(contents)
}

fn apply_options<'a>(file_contents: &'a mut String, args: &Args) -> &'a str {
    if args.squeeze_blank {
        *file_contents = squeeze_blank(file_contents);
    }
    if args.number_nonblank {
        *file_contents = number_nonblank(file_contents);
    } else if args.number { // if numbering nonblank only, this will be ignored
        *file_contents = number_lines(file_contents);
    } 
    if args.show_ends {
        *file_contents = show_ends(file_contents);
    }
        file_contents
}

// removes consecutive blank lines
fn squeeze_blank(text: &str) -> String {
    let mut prev_line_is_blank = false;
    text
        .lines()
        .filter_map(|line| {
            if prev_line_is_blank && line.is_empty() {
                return None
            } else if line.is_empty() {
                prev_line_is_blank = true;
            } else {
                prev_line_is_blank = false;
            }
            Some(format!("{line}\n"))
        })
        .collect()
}

// numbers each line
fn number_lines(text: &str) -> String {
    text
        .lines()
        .enumerate()
        .map(|(i, line)| format!("     {}  {line}\n", i+1))
        .collect()
}

// numbers each non-blank line
fn number_nonblank(text: &str) -> String {
    let mut count = 0;
    text
        .lines()
        .map(|line| {
            if !line.is_empty() {
                count += 1;
                return format!("     {count}  {line}\n");
            } else {
                return format!("        {line}\n");
            }
        })
        .collect()
}


// adds $ to the end of each line
fn show_ends(text: &str) -> String {
    text
        .lines()
        .map(|line| format!("{line}$\n"))
        .collect()
}

