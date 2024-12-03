use std::iter::Peekable;
use std::slice::Iter;

fn main() {
    let input = include_bytes!("../input");
    let x = parser_mul(input).iter().map(|v| v.x * v.y).sum::<u32>();
    println!("{x}")
}

#[derive(Debug)]
pub struct Mul {
    x: u32,
    y: u32,
}

pub fn parser_mul(input: &[u8]) -> Vec<Mul> {
    let mut peekable = input.iter().peekable();

    let mut result = Vec::new();

    while let Some(&val) = peekable.next() {
        if val == b'm' {
            if let Some(mul) = get_mul(&mut peekable) {
                result.push(mul);
            }
        }
    }
    println!("result: {result:?}");

    result
}

pub fn get_mul(src: &mut Peekable<Iter<u8>>) -> Option<Mul> {
    if !matches!(src.next(), Some(&b'u')) {
        return None;
    };
    if !matches!(src.next(), Some(&b'l')) {
        return None;
    };
    if !matches!(src.next(), Some(&b'(')) {
        return None;
    };

    let mut first_num = String::new();
    // start iterating over the first number
    while let Some(&val) = src.next() {
        if val.is_ascii_digit() {
            first_num.push(val.into());
        } else if val == b',' {
            break;
        } else {
            return None;
        }
    }

    let mut second_num = String::new();
    while let Some(&val) = src.next() {
        if val.is_ascii_digit() {
            second_num.push(val.into());
        } else if val == b')' {
            break;
        } else {
            return None;
        }
    }

    // these two parsing are valid
    let x = first_num.parse::<u32>().unwrap();
    let y = second_num.parse::<u32>().unwrap();

    Some(Mul { x, y })
}
