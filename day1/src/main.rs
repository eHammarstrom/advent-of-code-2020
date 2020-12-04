use day1::*;

fn main() -> std::io::Result<()> {
    let nums = nums_from_readable(std::io::stdin())?;

    println!("Solution A: {}", a(nums.to_owned())?);
    println!("Solution B: {}", b(nums)?);

    Ok(())
}
