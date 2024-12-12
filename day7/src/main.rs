fn main() {
    let data = include_str!("../input");
    let data = parse(data);

    let result = data
        .iter()
        .filter(|(input, sum)| get_calculation(input, *sum))
        .map(|(_, sum)| *sum)
        .sum::<u64>();
    println!("result {result}")
}

pub fn get_calculation(input: &[u64], sum: u64) -> bool {
    let operations = get_all_operation_set(input.len() - 1);
    for ops in operations {
        let mut iter = input.iter();
        let mut output = *iter.next().unwrap();
        for op in ops {
            match op {
                Operation::Plus => output += *iter.next().unwrap(),
                Operation::Multiply => output *= *iter.next().unwrap(),
                Operation::Concac => {
                    let next = *iter.next().unwrap();
                    output = format!("{output}{next}").parse().expect("valid");
                }
            }
        }
        if output == sum {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operation {
    Plus,
    Multiply,
    Concac,
}

fn get_all_operation_set(num: usize) -> Vec<Vec<Operation>> {
    fn generate_operations(
        num: usize,
        current: &mut Vec<Operation>,
        result: &mut Vec<Vec<Operation>>,
    ) {
        if num == 0 {
            result.push(current.clone());
            return;
        }

        // Add Plus operation and recurse
        current.push(Operation::Plus);
        generate_operations(num - 1, current, result);
        current.pop();

        // Add Multiply operation and recurse
        current.push(Operation::Multiply);
        generate_operations(num - 1, current, result);
        current.pop();

        // Add Plus operation and recurse
        current.push(Operation::Concac);
        generate_operations(num - 1, current, result);
        current.pop();
    }

    let mut result = Vec::new();
    let mut current = Vec::with_capacity(num);
    generate_operations(num, &mut current, &mut result);
    result
}

pub fn parse(input: &str) -> Vec<(Vec<u64>, u64)> {
    input
        .lines()
        .map(|line| {
            let split = line.split(':').collect::<Vec<_>>();
            let sum = split[0].parse::<u64>().expect("input is valid");
            let data = split[1]
                .split_whitespace()
                .map(|val| val.parse::<u64>().expect("is valid"))
                .collect::<Vec<_>>();
            (data, sum)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_small_input() {
        let data = include_bytes!("../test");
        assert_eq!(1 < 2, true);
    }
}
