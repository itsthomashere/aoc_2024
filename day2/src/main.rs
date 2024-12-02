fn main() {
    let input = include_str!("../input");
    let result = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().expect("is valid"))
                .collect::<Vec<_>>()
        })
        .filter(|line| !line.is_empty() && is_safe(line))
        .count();

    println!("{result}");
}

pub fn is_safe(slice: &[i32]) -> bool {
    let len = slice.len();
    let is_increasing = is_increasing(slice);
    let mut unsafe_pos: Vec<(usize, usize)> = Vec::new();

    for i in 0..(len - 1) {
        let a = slice[i];
        let b = slice[i + 1];

        if (a <= b) != is_increasing {
            unsafe_pos.push((i, i + 1));
            continue;
        }

        let diff = (a - b).abs();

        if !(1..=3).contains(&diff) {
            unsafe_pos.push((i, i + 1));
        }
    }

    if unsafe_pos.is_empty() {
        true
    } else {
        let mut result = false;
        for (first, second) in unsafe_pos {
            let mut data = slice[..first].to_vec();
            data.extend_from_slice(&slice[(first + 1)..]);
            let mut data1 = slice[..second].to_vec();
            data1.extend_from_slice(&slice[(second + 1)..]);

            result |= is_safe_once(&data) || is_safe_once(&data1)
        }

        result
    }
}

pub fn is_safe_once(slice: &[i32]) -> bool {
    let len = slice.len();
    let is_increasing = is_increasing(slice);

    for i in 0..(len - 1) {
        let a = slice[i];
        let b = slice[i + 1];

        if (a <= b) != is_increasing {
            return false;
        }
        let diff = (a - b).abs();

        if !(1..=3).contains(&diff) {
            return false;
        }
    }

    true
}

pub fn is_increasing(slice: &[i32]) -> bool {
    let len = slice.len();
    let mut increased = 0;

    for i in 0..(len - 1) {
        if slice[i] <= slice[i + 1] {
            increased += 1;
        } else {
            increased -= 1;
        }
    }

    increased > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let output = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<i32>().expect("valid"))
                    .collect::<Vec<_>>()
            })
            .filter(|line| !line.is_empty())
            .filter(|line| is_safe(line))
            .count();

        assert_eq!(output, 4)
    }
}
