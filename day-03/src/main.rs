use std::fs;
use num_bigint::BigUint;
use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    process(&content);
    process2(&content);
}

fn process(content: &str) {
    let r = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut full_content: String = "".to_owned();
    for line in content.lines() {
        full_content.push_str(line);
    }

    let mut result = BigUint::ZERO;
    for c in r.captures_iter(&full_content) {
        let left = &c[1];
        let right = &c[2];
        result += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
    }

    println!("result {}", result);
}

fn process2(content: &str) {
    let r = Regex::new(r"(do\(\))|(don't\(\))|(mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();

    let mut full_content: String = "".to_owned();
    for line in content.lines() {
        full_content.push_str(line);
    }

    let mut dont = false;
    let mut result = BigUint::ZERO;
    for c in r.captures_iter(&full_content) {
        if !&c.get(1).is_none() {
            dont = false;
        }

        if !&c.get(2).is_none() {
            dont = true;
        }

        if !dont && !&c.get(4).is_none() {
            let left = &c[4];
            let right = &c[5];
            result += left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap();
        }
    }

    println!("result2 {}", result);
}