use std::collections::{HashMap, HashSet};

fn main() {
    let data = include_str!("../input");
    let (rule, input) = data.split_once("\n\n").unwrap();

    let rules = parse_rule(rule);
    let input = parse_input(input);

    let result = gen_output(input, &rules);
    println!("result: {result}");
}

pub fn gen_output(input: Vec<Vec<i32>>, rules: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut result = 0;
    for val in input {
        result += check_vec_2(val, rules);
    }

    result
}

pub fn check_vec(input: Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> i32 {
    for i in 0..(input.len() - 1) {
        let a = input[i];
        let b = input[i + 1];

        if rules.get(&a).is_some_and(|set| set.contains(&b)) || rules.get(&a).is_none() {
            continue;
        } else {
            return 0;
        }
    }
    input[input.len() / 2]
}

pub fn check_vec_2(mut input: Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut valid = true;

    for i in 0..(input.len() - 1) {
        for j in i + 1..input.len() {
            let a = input[i];
            let b = input[j];

            if rules.get(&b).is_some_and(|set| set.contains(&a)) {
                input[j] = a;
                input[i] = b;
                valid = false;
            }
        }
    }

    if !valid {
        input[input.len() / 2]
    } else {
        0
    }
}

pub fn parse_rule(rule: &str) -> HashMap<i32, HashSet<i32>> {
    let mut result = HashMap::new();
    rule.lines()
        .map(|line| line.split_once('|').unwrap())
        .for_each(|(a, b)| {
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();

            let entry = result.entry(a).or_insert(HashSet::default());

            entry.insert(b);
        });

    result
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(',').map(|a| a.parse::<i32>().unwrap()).collect())
        .collect()
}
