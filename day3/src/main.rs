use std::fs;

fn read_readings() -> Vec<Vec<char>> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut readings: Vec<Vec<char>> = vec![];

    for reading_str in contents.split("\n") {
        readings.push(reading_str.to_string().chars().collect());
    }

    readings
}

fn part1() {
    let mut common_bits = vec![vec![0; 2]; 12];

    for reading in read_readings() {
        for (index, bit) in reading.iter().enumerate() {
            match bit {
                '0' => common_bits[index][0] += 1,
                '1' => common_bits[index][1] += 1,
                _ => panic!("bit is not 0 or 1"),
            }
        }
    }

    let mut gama_rate: Vec<char> = vec![];
    let mut epsilon_rate: Vec<char> = vec![];

    for bit in common_bits {
        if bit[0] > bit[1] {
            gama_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gama_rate.push('1');
            epsilon_rate.push('0');
        }
    }

    let gama_rate_value =
        isize::from_str_radix(gama_rate.into_iter().collect::<String>().as_str(), 2).unwrap();
    let epsilon_rate_value =
        isize::from_str_radix(epsilon_rate.into_iter().collect::<String>().as_str(), 2).unwrap();

    print!(
        "Power consumption: {:?} ",
        gama_rate_value * epsilon_rate_value
    );
}

fn split_by_common(readings: Vec<Vec<char>>, index: usize, mode: usize) -> Vec<char> {
    if readings.len() == 1 {
        return readings[0].clone();
    }

    let mut common_bits: Vec<Vec<Vec<char>>> = vec![vec![]; 2];

    for reading in readings {
        match reading[index] {
            '0' => common_bits[0].push(reading),
            '1' => common_bits[1].push(reading),
            _ => panic!("bit is not 0 or 1"),
        }
    }

    if mode == 1 {
        if common_bits[1].len() >= common_bits[0].len() {
            return split_by_common(common_bits[1].clone(), index + 1, mode);
        } else {
            return split_by_common(common_bits[0].clone(), index + 1, mode);
        }
    } else {
        if common_bits[1].len() >= common_bits[0].len() {
            return split_by_common(common_bits[0].clone(), index + 1, mode);
        } else {
            return split_by_common(common_bits[1].clone(), index + 1, mode);
        }
    }
}

fn part2() {
    let oxygen_generator_reading = split_by_common(read_readings(), 0, 1);
    let co2_scrubber_reading = split_by_common(read_readings(), 0, 0);

    let oxygen_generator_value = isize::from_str_radix(
        oxygen_generator_reading
            .into_iter()
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();
    let co2_scrubber_value = isize::from_str_radix(
        co2_scrubber_reading
            .into_iter()
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();

    print!(
        "Life support rating : {:?} ",
        oxygen_generator_value * co2_scrubber_value
    );
}

fn main() {
    part1();
    part2();
}
