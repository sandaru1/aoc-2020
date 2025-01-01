use regex::Regex;
use std::fs;
use std::collections::HashMap;

fn can_carry(nodes:&HashMap<String,Vec<(usize,String)>>,bag:&str)->bool {
    if bag == "shiny gold" {
        return true;
    }
    if let Some(bags) = nodes.get(bag) {
        bags.iter().filter(|(_,bag)| {
            can_carry(nodes,&bag)
        }).count() > 0
    } else {
        false
    }
}

fn num_bags(nodes:&HashMap<String,Vec<(usize,String)>>,bag:&str) -> usize {
    if let Some(bags) = nodes.get(bag) {
        bags.iter().map(|(num,bag)| {
            (num_bags(&nodes,&bag) + 1) as usize * num
        }).sum::<usize>()
    } else {
        0
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let nodes:HashMap<_,_> = contents.lines().filter_map(|s| {
        let re = Regex::new(r"(.*) bags contain (.*)\.").unwrap();
        match re.captures(s) {
            Some(caps) => {
                let bag = &caps[1];
                let inside:Vec<_> = caps[2].split(", ").filter_map(|s| {
                    let re = Regex::new(r"([0-9]) (.*) bag").unwrap();
                    if let Some(caps) = re.captures(s) {
                        Some((caps[1].parse::<usize>().unwrap(),caps[2].to_owned()))
                    } else {
                        None
                    }
                }).collect();
                
                Some((bag.to_owned(),inside.to_owned()))
            }
            None => {
                println!("Invalid regex");
                None
            }
        }
    }).collect();

    let total = nodes.keys().filter(|bag| can_carry(&nodes,&bag)).count() - 1;

    println!("{}",total);

    println!("{}",num_bags(&nodes,&"shiny gold"));
    
}
