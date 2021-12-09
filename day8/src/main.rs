use std::fs;

#[derive(Debug)]
struct Signals {
    input: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
}

fn read_data() -> Vec<Signals> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut data: Vec<Signals> = vec![];
    for line in contents.lines() {
        let row: Vec<Vec<Vec<char>>> = line
            .split(" | ")
            .map(|row_data| {
                row_data
                    .split(" ")
                    .map(|signal| signal.chars().into_iter().collect())
                    .collect()
            })
            .collect();

        data.push(Signals {
            input: row[0].clone(),
            output: row[1].clone(),
        })
    }
    data
}

fn count_easy_digits(data: Vec<Signals>) {
    let mut easy_digits_count = 0;
    for signal in data {
        for outputs in signal.output {
            let length = outputs.len();
            // 1, 4, 7, 8
            if length == 2 || length == 4 || length == 3 || length == 7 {
                easy_digits_count += 1;
            }
        }
    }

    println!("Easy segments appear: {}", easy_digits_count);
}

fn str_to_u8(s: &Vec<char>) -> u8 {
    s.iter()
        .map(|x| 1 << (*x as u16 - 97))
        .reduce(|a, b| a | b)
        .unwrap() as u8
}

fn fill_simple_segments(signal: &Signals, segments: &mut [u8; 7]) {
    let mut digit_one: u8 = 0;
    let mut digit_four: u8 = 0;
    let mut digit_seven: u8 = 0;

    let mut segments_counts = [0i32; 7];

    for digit in &signal.input {
        let digit_bit = str_to_u8(digit);

        // get digits with know length
        match digit_bit.count_ones() {
            2 => digit_one = digit_bit,
            3 => digit_seven = digit_bit,
            4 => digit_four = digit_bit,
            _ => (),
        }

        // count how many times segments repeat
        for i in 0..8 {
            let mask = 1 << i;
            let r = digit_bit & mask != 0;
            if r {
                segments_counts[i] += 1;
            }
        }
    }

    // if segment repeats 6 times  = b/1
    // if segment repeats 4 times  = e/4
    // if segment repeats 9 times  = f/5
    for (index, segment_count) in segments_counts.iter().enumerate() {
        match segment_count {
            6 => segments[1] = (1 << index) as u8,
            4 => segments[4] = (1 << index) as u8,
            9 => segments[5] = (1 << index) as u8,
            _ => (),
        }
    }

    // difference between digit one and seven is a/0
    segments[0] = digit_one ^ digit_seven;
    //
    segments[3] = digit_four & !digit_seven & !segments[1];
    segments[2] = digit_four & !segments[1] & !segments[3] & !segments[5];
    segments[6] = 0x7F
        & !segments[0]
        & !segments[1]
        & !segments[2]
        & !segments[3]
        & !segments[4]
        & !segments[5];
}

fn create_digits(segments: &[u8; 7]) -> [u8; 10] {
    let digit_zero =
        segments[0] | segments[1] | segments[2] | segments[4] | segments[5] | segments[6];
    let digit_one = segments[2] | segments[5];
    let digit_two = segments[0] | segments[2] | segments[3] | segments[4] | segments[6];;
    let digit_three = segments[0] | segments[2] | segments[3] | segments[5] | segments[6];
    let digit_four = segments[1] | segments[2] | segments[3] | segments[5];
    let digit_five = segments[0] | segments[1] | segments[3] | segments[5] | segments[6];
    let digit_six =
        segments[0] | segments[1] | segments[3] | segments[4] | segments[5] | segments[6];
    let digit_seven = segments[0] | segments[2] | segments[5];
    let digit_eight = segments[0]
        | segments[1]
        | segments[2]
        | segments[3]
        | segments[4]
        | segments[5]
        | segments[6];
    let digit_nine =
        segments[0] | segments[1] | segments[2] | segments[3] | segments[5] | segments[6];

    [
        digit_zero,
        digit_one,
        digit_two,
        digit_three,
        digit_four,
        digit_five,
        digit_six,
        digit_seven,
        digit_eight,
        digit_nine,
    ]
}

fn parse(data: Vec<Signals>) {
    let mut sum: i32 = 0;
    for signal in data {
        let mut segments: [u8; 7] = [0; 7];

        fill_simple_segments(&signal, &mut segments);
        let digits = create_digits(&segments);
        sum += decode_outputs(&signal, &digits);
    }

    println!("Total sum: {}", sum);
}

fn decode_outputs(signal: &Signals, digits: &[u8; 10]) -> i32 {
    let mut sum: i32 = 0;
    let mut weight = 1000;
    for output_digit in &signal.output {

        let output_digit_bit = str_to_u8(output_digit);
        for (index, digit) in digits.iter().enumerate() {
            if *digit == output_digit_bit {
                sum += weight * (index as i32);
                weight = weight / 10;
            }
        }
    }

    sum
}

fn part1() {
    count_easy_digits(read_data());
}

fn part2() {
    parse(read_data());
}

fn main() {
    part1();
    part2();
}
