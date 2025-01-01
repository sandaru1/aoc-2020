use std::fs;
use regex::Regex;

fn part1(contents:String) {
    let cnt = contents.lines().filter(|s| {
        let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): (.*)").unwrap();
        match re.captures(s) {
            Some(caps) => {
                let min:i32 = caps[1].parse::<i32>().unwrap();
                let max:i32 = caps[2].parse::<i32>().unwrap();
                let ch = caps[3].chars().next().unwrap();
                let password:&str = &caps[4];
                let mut cnt = 0;
                for c in password.chars() {
                    if c == ch {
                        cnt+=1;
                    }
                }
                cnt >= min && cnt <= max
            }
            None => {
                println!("Invalid regex");
                false
            }
        }
    }).count();

    println!("{}",cnt);
}

fn part2(contents:String) {
    let cnt = contents.lines().filter(|s| {
        let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): (.*)").unwrap();
        match re.captures(s) {
            Some(caps) => {
                let first:usize = caps[1].parse::<usize>().unwrap();
                let second:usize = caps[2].parse::<usize>().unwrap();
                let ch = caps[3].chars().next().unwrap();
                let chars:Vec<char> = caps[4].chars().collect();
                (chars[first-1] == ch && chars[second-1]!=ch) || (chars[first-1] != ch && chars[second-1]==ch)
            }
            None => {
                println!("Invalid regex");
                false
            }
        }
    }).count();

    println!("{}",cnt);
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    part2(contents);
}
