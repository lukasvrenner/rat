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
    #[arg(short, long, default_value_t = false)]
    squeeze_blank: bool,


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

fn apply_options(file_contents: &mut String, args: &Args) -> String {
    if args.number {
        *file_contents = number(file_contents);
           
    } 
    if args.squeeze_blank {
        *file_contents = squeeze_blank(file_contents);
    }
        file_contents.to_string()
    
}

fn number(text: &str) -> String {
    text
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{}  {line}\n", i+1))
        .collect()
}

fn squeeze_blank(text: &str) -> String {
    todo!();
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     let file_contents: String = String::from("
// This is a string
//
// It has multiple lines
//
//
// It even has larger gaps
//                                                  ");
//     assert_eq!(number(file_contents), String::from("
// 1  This is a string
// 2  
// 3  It has multiple lines
// 4
// 5  It even has larger gaps
//                                                    "));
// }
