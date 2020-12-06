use day2::*;

use std::io;
use std::io::prelude::*;

fn split_lines<R: Read>(r: R, mut buf: &mut String) -> io::Result<Vec<&str>> {
    let mut stream = io::BufReader::new(r);
    stream.read_to_string(&mut buf)?;

    let lines: Vec<&str> = buf.split(|c: char| c == '\n').collect();

    Ok(lines)
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    let lines = split_lines(std::io::stdin(), &mut buf)?;

    println!(
        "Solution A: {}",
        lines
            .into_iter()
            .map(ParseContext::new)
            .map(|mut ctx| ctx.parse_password())
            .flatten()
            .map(|pass| pass.valid())
            .filter(|&valid| valid)
            .count()
    );

    Ok(())
}
