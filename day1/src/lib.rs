use std::io;
use std::io::prelude::*;
use std::io::{ Error, ErrorKind };

pub fn nums_from_readable<T: Read>(f: T) -> io::Result<Vec<u64>> {
    let mut stdin_reader = io::BufReader::new(f);
    let mut file_str: String = String::new();

    stdin_reader.read_to_string(&mut file_str)?;

    let nums: Vec<u64> = file_str
        .split(|c: char| c == '\n')
        .map(str::parse::<u64>)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();

    Ok(nums)
}

pub fn a(mut nums: Vec<u64>) -> io::Result<u64> {
    let exhaustion_error: Error =
        Error::new(ErrorKind::InvalidInput, "Exhausted all data");
    let mut start = 0;
    let mut end = nums.len() - 1;
    let mut sum = 0;

    nums.sort();

    while sum != 2020 {
        sum = nums[start] + nums[end];

        if sum > 2020 {
            end -= 1;
        } else if sum < 2020 {
            start += 1;
        }

        if start == end {
            return Err(exhaustion_error);
        }
    }

    Ok(nums[start] * nums[end])
}

pub fn b(mut nums: Vec<u64>) -> io::Result<u64> {
    let exhaustion_error: io::Result<u64> =
        Err(Error::new(ErrorKind::InvalidInput, "Exhausted all data"));
    let mut start = 0;
    let mut mid = start + 1;
    let mut end = nums.len() - 1;
    let mut sum = 0;

    nums.sort();

    while sum != 2020 {
        sum = nums[start] + nums[mid] + nums[end];

        if sum > 2020 {
            /* Don't let end mid, or start collide when
             * trying to decrease them in the following
             * order: end, mid, start
             */
            if end - 1 == mid {
                if mid - 1 == start {
                    start -= 1;
                } else {
                    mid -= 1;
                }
            } else {
                end -= 1;
            }
        } else if sum < 2020 {
            /* Don't let start, mid, or end collide when
             * trying to increase them in the following
             * order: start, mid, end
             */
            if start + 1 == mid {
                if mid + 1 == end {
                    end += 1;
                } else {
                    mid += 1;
                }
            } else {
                start += 1;
            }
        }

        if start == mid {
            return exhaustion_error
        } else if mid == end {
            return exhaustion_error
        }
    }


    Ok(nums[start] * nums[mid] * nums[end])
}
