use rayon::prelude::*;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let init = include_str!("../input");
    let data: Vec<_> = init
        .lines()
        .map(|f| f.trim().as_bytes().to_vec())
        .filter(|f| !f.is_empty())
        .collect();

    let mut test = Data1::new(data.clone());
    test.parse();
    assert_eq!(test.get_count(), test.parse_multithread());

    let iterations = 10000;

    // Single-threaded timing
    let mut total_time_st = 0;
    for _ in 0..iterations {
        let mut data = Data1::new(data.clone());
        let now = Instant::now();
        data.parse();
        total_time_st += now.elapsed().as_nanos();
    }
    println!(
        "Average single-threaded time: {} ns",
        total_time_st / iterations
    );

    // Multi-threaded timing
    let mut total_time_mt = 0;
    for _ in 0..iterations {
        let data = Data1::new(data.clone());
        let now = Instant::now();
        data.parse_multithread();
        total_time_mt += now.elapsed().as_nanos();
    }
    println!(
        "Average multi-threaded time: {} ns",
        total_time_mt / iterations
    );

    println!(
        "improvement: {:?}%",
        (total_time_st as f64) / (total_time_mt as f64) * 100.0 - 100.0
    )
}

#[derive(Debug)]
struct Data1 {
    bound_x: usize,
    bound_y: usize,
    data: Arc<Vec<Vec<u8>>>,
    count: usize,
    count_atomic: AtomicUsize,
}

impl Data1 {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        assert!(!data.is_empty());
        for i in 0..(data.len() - 1) {
            assert!(!data[i].is_empty());
            assert_eq!(data[i].len(), data[i + 1].len())
        }
        Self {
            bound_x: data[0].len(),
            bound_y: data.len(),
            data: data.into(),
            count: 0,
            count_atomic: 0.into(),
        }
    }

    pub fn parse(&mut self) {
        for x in 0..self.bound_x {
            for y in 0..self.bound_y {
                if self.data[y][x] == b'X' {
                    if self.check_combination((x, y), (0, 1), b"MAS") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (1, 0), b"MAS") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (-1, 0), b"MAS") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (0, -1), b"MAS") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (1, -1), b"MAS") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (1, 1), b"MAS") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (-1, 1), b"MAS") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (-1, -1), b"MAS") {
                        self.count += 1;
                    }
                } else if self.data[y][x] == b'S' {
                    if self.check_combination((x, y), (0, 1), b"AMX") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (1, 0), b"AMX") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (-1, 0), b"AMX") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (0, -1), b"AMX") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (1, 1), b"AMX") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (1, -1), b"AMX") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (-1, 1), b"AMX") {
                        self.count += 1;
                    }
                    if self.check_combination((x, y), (-1, -1), b"AMX") {
                        self.count += 1;
                    }
                }
            }
        }
    }

    pub fn parse_multithread(self) -> usize {
        let directions: [(i16, i16); 8] = [
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
        ];

        self.data
            .par_iter()
            .enumerate()
            .map(|(y, inner)| {
                let sum = inner
                    .par_iter()
                    .enumerate()
                    .map(|(x, inner_x)| {
                        let mut counter = 0;
                        for dirt in directions {
                            if *inner_x == b'X' && self.check_combination((x, y), dirt, b"MAS") {
                                counter += 1;
                            }

                            if *inner_x == b'S' && self.check_combination((x, y), dirt, b"AMX") {
                                counter += 1;
                            }
                        }
                        counter
                    })
                    .sum::<usize>();
                sum
            })
            .sum::<usize>()
            / 2
    }

    pub fn get_count(&self) -> usize {
        self.count / 2
    }

    fn check_combination_atomic(
        source: Arc<Vec<Vec<u8>>>,
        bound_x: usize,
        bound_y: usize,
        cursor: (usize, usize),
        direction: (i16, i16),
        data: &[u8],
    ) -> bool {
        let (mut x, mut y) = (cursor.0 as i16, cursor.1 as i16);
        for val in data {
            if x + direction.0 < 0
                || y + direction.1 < 0
                || x + direction.0 >= bound_x as i16
                || y + direction.1 >= bound_y as i16
            {
                return false;
            }

            x += direction.0;
            y += direction.1;

            if source[y as usize][x as usize] != *val {
                return false;
            }
        }

        true
    }

    pub fn check_combination(
        &self,
        cursor: (usize, usize),
        direction: (i16, i16),
        data: &[u8],
    ) -> bool {
        let (mut x, mut y) = (cursor.0 as i16, cursor.1 as i16);
        for val in data {
            if x + direction.0 < 0
                || y + direction.1 < 0
                || x + direction.0 >= self.bound_x as i16
                || y + direction.1 >= self.bound_x as i16
            {
                return false;
            }

            x += direction.0;
            y += direction.1;

            if self.data[y as usize][x as usize] != *val {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
struct Data2 {
    bound_x: usize,
    bound_y: usize,
    data: Vec<Vec<u8>>,
    count: usize,
}

impl Data2 {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        assert!(!data.is_empty());
        for i in 0..(data.len() - 1) {
            assert!(!data[i].is_empty());
            assert_eq!(data[i].len(), data[i + 1].len())
        }
        Self {
            bound_x: data[0].len(),
            bound_y: data.len(),
            data,
            count: 0,
        }
    }

    pub fn parse(&mut self) {
        for x in 0..self.bound_x {
            for y in 0..self.bound_y {
                if self.data[y][x] == b'A' && self.check_xmas((x, y)) {
                    self.count += 1;
                }
            }
        }
    }

    pub fn get_count(&self) -> usize {
        self.count
    }

    pub fn check_xmas(&self, (x, y): (usize, usize)) -> bool {
        assert_eq!(self.data[y][x], b'A');
        let x = x as i16;
        let y = y as i16;

        if x + 1 >= self.bound_x as i16 || x - 1 < 0 || y - 1 < 0 || y + 1 >= self.bound_y as i16 {
            return false;
        }

        let a = [
            self.data[(y - 1) as usize][(x - 1) as usize] as char,
            self.data[y as usize][x as usize] as char,
            self.data[(y + 1) as usize][(x + 1) as usize] as char,
        ];

        let b = [
            self.data[(y - 1) as usize][(x + 1) as usize] as char,
            self.data[y as usize][x as usize] as char,
            self.data[(y + 1) as usize][(x - 1) as usize] as char,
        ];

        if (a == ['M', 'A', 'S'] || a == ['S', 'A', 'M'])
            && (b == ['M', 'A', 'S'] || b == ['S', 'A', 'M'])
        {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let data = "MMMSXXMASM
                    MSAMXMSMSA
                    AMXSXMAAMM
                    MSAMASMSMX
                    XMASAMXAMM
                    XXAMMXXAMA
                    SMSMSASXSS
                    SAXAMASAAA
                    MAMMMXMMMM
                    MXMXAXMASX";
        let data: Vec<_> = data
            .lines()
            .map(|f| f.trim().as_bytes().to_vec())
            .filter(|f| !f.is_empty())
            .collect();

        let mut data = Data1::new(data);
        data.parse();
        let count = data.get_count();
        assert_eq!(count, 18);
    }
    #[test]
    fn test_default_2() {
        let data = "MMMSXXMASM
                    MSAMXMSMSA
                    AMXSXMAAMM
                    MSAMASMSMX
                    XMASAMXAMM
                    XXAMMXXAMA
                    SMSMSASXSS
                    SAXAMASAAA
                    MAMMMXMMMM
                    MXMXAXMASX";
        let data: Vec<_> = data
            .lines()
            .map(|f| f.trim().as_bytes().to_vec())
            .filter(|f| !f.is_empty())
            .collect();

        let mut data = Data2::new(data);
        data.parse();
        let count = data.get_count();
        assert_eq!(count, 9);
    }
}
