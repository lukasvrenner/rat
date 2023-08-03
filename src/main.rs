use std::fs;
use std::error::Error;
use std::env;
use std::process;

fn main() {
    let file = parse_args().unwrap().unwrap();
    let text = read_file(file)
        .unwrap_or_else(|err| {
            eprintln!("unable to read file due to error: {}", err);
            process::exit(1);
        });
    print!("{text}");
}

fn parse_args() -> Result<Option<String>, &'static str> {
    let mut args = env::args();
    args.next();
    Ok(args.next())

}

// reads text file
// will be modified to take arguments
fn read_file(file: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(format!("{file}"))?;

    Ok(contents)
}


