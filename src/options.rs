use crate::Args;

pub fn apply_options<'a>(file_contents: &'a mut String, args: &Args) -> &'a str {
    if args.squeeze_blank {
        *file_contents = squeeze_blank(file_contents);
    }
    if args.number_nonblank {
        *file_contents = number_nonblank(file_contents);
    } else if args.number_lines { // if numbering nonblank, this will be ignored
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
        .map(|(i, line)| format!("{0:6}  {line}\n", i+1)) // currently causes test to fail
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
                return format!("{0:6}  {line}\n", count);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbered_lines() {
    let numbered = "     1  this is a line
     2  this is another line
     3  this is another line
     4  
     5  
     6  
     7  here is some text 
     8  
     9  
    10  this is a line\n"; // note the indentation of the 10
    let unnumbered = "\
this is a line
this is another line
this is another line



here is some text 


this is a line";    
    assert_eq!(numbered, number_lines(unnumbered));
    }

    #[test]
    fn numbered_nonblank() { // note the hidden whitespaces in the "empty" lines
    let numbered_nonblank = "     1  this is a line
     2  this is another line
     3  this is another line
        
        
        
     4  here is some text 
        
        
     5  this is a line\n";
    let unnumbered = "\
this is a line
this is another line
this is another line



here is some text 


this is a line";    
    assert_eq!(numbered_nonblank, number_nonblank(unnumbered));

    }
    
    #[test]
    fn squeezed_blanks() {
        let input_text = "\
this is a line


this is a line with two blanks

this is a line with only one blank
this is a line with no blanks";
        
        let squeezed = "\
this is a line

this is a line with two blanks

this is a line with only one blank
this is a line with no blanks\n";
        
        assert_eq!(squeezed, squeeze_blank(input_text));
    }

}
