use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn part1(contents:&str) {
    let answers: Vec<Vec<_>> = contents
        .split("\n\n").map(|s|
            s.chars()
                .filter(|c| c!=&'\n')
                .collect::<HashSet<_>>()
                .into_iter()
                .collect()
        ).collect();

    let total = answers.iter().fold(0,|sum, i| sum + i.len());

    println!("{}",total);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let answers: Vec<Vec<_>> = contents
        .split("\n\n").map(|s|
            s.split("\n")
            .collect::<Vec<_>>()
    ).collect();

    let mut total = 0;
    for a in answers.iter() {
        let mut cnts:HashMap<char,usize> = HashMap::new();
        for b in a.iter() {
            for c in b.chars() {
                if let Some(count) = cnts.get(&c) {
                    cnts.insert(c, count + 1);
                } else {
                    cnts.insert(c, 1);
                };    
            }
        }
        total += cnts.iter().filter(|(ch,cnt)| **cnt==a.len()).count();
    }

    println!("{}",total);
}
