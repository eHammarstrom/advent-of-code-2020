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
    let (solution_a, solution_b) = lines
        .into_iter()
        .map(ParseContext::new)
        .map(|mut ctx| ctx.parse_password())
        .flatten()
        .map(|pass| (pass.valid_a(), pass.valid_b()))
        .fold((0, 0), |(acc_a, acc_b), (a, b)| {
            (
                if a { acc_a + 1 } else { acc_a },
                if b { acc_b + 1 } else { acc_b },
            )
        });

    println!("Solution A: {}", solution_a);
    println!("Solution B: {}", solution_b);

    Ok(())
}
