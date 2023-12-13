use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
  
    let first_file = File::open("input.txt")?; //This is our first file, that we are reading
    let read = BufReader::new(firstfile);

    let mut write_to_this_file = File::create("output.txt")?; //This is our second file that we want to write those line we read from the first section

    for line in read.lines() {
        let line = line?;
        writeln!(write_to_this_file, "{}", line)?; //Here we are reading each line in the first file, and we are writing it to the second file.
    }

    
}
