use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn parse_and_add_file() -> io::Result<i32> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    return Ok(f
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(408, parse_and_add_file().unwrap());
    }
}
