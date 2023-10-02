use std::fs;
use std::process;
use clap::Parser;

mod options;
use options::apply_options;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to file
    file: String,

    /// Number all output lines
    #[arg(short, long, default_value_t = false)]
    number_lines: bool,

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
    
    // reads file and exits if fails
    let mut file_contents = fs::read_to_string(&args.file)
        .unwrap_or_else(|err| {
        eprintln!("error: could not read file: {err}");
        process::exit(1);

    });
    
    /* takes user-inputted options from the shell and
    applies them to the file contents */
    apply_options(&mut file_contents, &args);
    print!("{}", file_contents); // does not print a new line because text ends in new line
}



