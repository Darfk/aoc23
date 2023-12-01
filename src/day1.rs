
use advent::read_input_lines;

pub fn part1() -> u32 {
    let mut accum: u32 = 0;
    for (i, line) in read_input_lines("input/day1/input.txt").iter().enumerate() {
        let digits = line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<char>>();

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();

        let mut combined = String::new();
        combined.push(*first);
        combined.push(*last);
        let n: u32 = combined.parse().unwrap();

        println!(
            "{}: first: {}, last: {}, combined: {}",
            i + 1,
            first,
            last,
            combined
        );

        accum += n;
    }
    return accum;
}

pub fn part2() -> u32 {
    #[derive(Debug, Clone)]
    struct Digit {
        text: String,
        value: u32,
        indices: Vec<usize>,
    }

    let mut digits = "1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine"
        .split('|')
        .enumerate()
        .map(|(i, text)| Digit {
            text: text.to_string(),
            value: i as u32 % 9 + 1,
            indices: Vec::new(),
        })
        .collect::<Vec<_>>();

    impl Digit {
        fn reset(&mut self) {
            self.indices.clear();
        }
    }

    let mut accum: u32 = 0;

    for (i, line) in read_input_lines("input/day1/input.txt")
        .iter()
        .enumerate()
    {
        digits.iter_mut().for_each(Digit::reset);
        for digit in digits.iter_mut() {
            digit.indices = line.match_indices(&digit.text).map(|m| m.0).collect();
        }
        // dbg!(&digits);

        let mut first: Option<(usize, &Digit)> = None;
        let mut last: Option<(usize, &Digit)> = None;

        for digit in digits.iter() {
            if digit.indices.len() == 0 {
                continue;
            }
            let first_index = digit.indices.first().unwrap();
            if first.is_none() || first.is_some_and(|f| first_index.lt(&f.0)) {
                first = Some((*first_index, &digit));
            }

            let last_index = digit.indices.last().unwrap();
            if last.is_none() || last.is_some_and(|l| last_index.gt(&l.0)) {
                last = Some((*last_index, &digit));
            }
        }

        let first = first.unwrap().1.value;
        let last = last.unwrap().1.value;
        let number: u32 = format!("{}{}", first, last).parse().unwrap();
        accum += number;

        println!("{}: {},{} = {}", i + 1, first, last, number);
    }
    return accum;
}
