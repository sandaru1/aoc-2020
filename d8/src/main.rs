use regex::Regex;
use std::fs;
use std::collections::HashSet;

fn get_acc(ops:&Vec<(String,i32)>)->(bool,i32) {
    let mut visited:HashSet<usize> =  HashSet::new();
    let mut acc = 0;
    let mut pos:usize = 0;
    while !visited.contains(&pos) && pos<ops.len() {
        visited.insert(pos);
        match ops[pos].0.as_str() {
            "nop" => {
                pos += 1;
            }
            "acc" => {
                acc += ops[pos].1;
                pos += 1;
            }
            "jmp" => {
                pos = (pos as i32 + ops[pos].1) as usize;
            }
            &_ => {
                println!("Invalid op");
            }
        };
    }
    (pos<ops.len(),acc)
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut ops:Vec<_> = contents.lines().filter_map(|s| {
        let re = Regex::new(r"(nop|acc|jmp) (.*)").unwrap();
        match re.captures(s) {
            Some(caps) => {
                Some((caps[1].to_owned(),caps[2].parse::<i32>().unwrap()))
            }
            _ => None
        }
    }).collect();

    println!("{:?}",get_acc(&ops).1);

    for (i,(op,_)) in ops.clone().iter().enumerate() {
        if op == "nop" {
            ops[i].0 = String::from("jmp");
        } else if op == "jmp" {
            ops[i].0 = String::from("nop");
        }
        let (looped,acc) = get_acc(&ops);
        if looped == false {
            println!("{}",acc);
            break;
        }
        ops[i].0 = op.to_string();
    }
}
