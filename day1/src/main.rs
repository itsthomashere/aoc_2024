fn main() {
    let input = include_str!("../input");
    let quest_1 = first_question(input);

    println!("{quest_1}");

    let quest_2 = second_question(input);

    println!("{quest_2}");
}

fn extract_column(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    input.lines().for_each(|line| {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|number| number.parse().expect("valid number"))
            .collect();

        left.push(numbers[0]);
        right.push(numbers[1]);
    });
    (left, right)
}

pub fn first_question(input: &str) -> i32 {
    let (mut left, mut right) = extract_column(input);
    left.sort_unstable();
    right.sort_unstable();

    let mut result = 0;
    for i in 0..left.len() {
        result += (left[i] - right[i]).abs();
    }
    result
}

pub fn second_question(input: &str) -> i32 {
    let (left, right) = extract_column(input);
    let mut similarity: i32 = 0;

    for number in left {
        similarity += number * right.iter().filter(|&&i| i == number).count() as i32;
    }

    similarity
}
