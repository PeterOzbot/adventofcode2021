use std::fs;

use models::{OperatorMode, Package, PackageType, PackageValue};
mod models;

fn read_data() -> Vec<char> {
    let file_name = "input";
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    contents.chars().collect()
}

fn to_binary(hex_data: Vec<char>) -> Vec<u8> {
    let mut binary_data = vec![];

    for hex in hex_data {
        let binary_string = match hex {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        };
        binary_data.append(
            &mut binary_string
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
    }

    binary_data
}

fn to_package_type(slice: &[u8]) -> PackageType {
    let int_value = to_u64(slice);
    match int_value {
        0 => PackageType::Sum,
        1 => PackageType::Product,
        2 => PackageType::Minimum,
        3 => PackageType::Maximum,
        4 => PackageType::LiteralValue,
        5 => PackageType::GreaterThan,
        6 => PackageType::LessThan,
        7 => PackageType::EqualTo,
        _ => unreachable!(),
    }
}

fn to_operator_mode(slice: &[u8]) -> OperatorMode {
    let int_value = to_u64(slice);
    match int_value {
        0 => OperatorMode::Bits(0),
        1 => OperatorMode::Packages(0),
        _ => unreachable!(),
    }
}

fn to_u64(slice: &[u8]) -> u64 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u64)
}

fn read_package(package: &[u8]) -> (Package, usize) {
    let version = to_u64(&package[0..3]);
    let id = to_package_type(&package[3..6]);
    let mut read_bits = 6; // 3(version) + 3(id)

    let (package_value, read_bits_value) = match id {
        PackageType::LiteralValue => read_literal_value(&package[read_bits..]),
        _ => read_operator(&package[read_bits..]),
    };

    // add bits from value
    read_bits += read_bits_value;

    (
        Package {
            version: version,
            id: id,
            value: package_value,
        },
        read_bits,
    )
}

fn read_operator(package_slice: &[u8]) -> (PackageValue, usize) {
    let mut operator_mode = to_operator_mode(&package_slice[0..1]);
    // 1bit(operator mode)
    let mut read_bits = 1;

    operator_mode = match operator_mode {
        OperatorMode::Bits(_) => {
            // 15bits(size)
            read_bits += 15;
            OperatorMode::Bits(to_u64(&package_slice[1..16]))
        }
        OperatorMode::Packages(_) => {
            // 11(size)
            read_bits += 11;
            OperatorMode::Packages(to_u64(&package_slice[1..12]))
        }
    };

    // read inner packages
    let (packages, current_read_bits) =
        read_operator_packages(&package_slice[read_bits..], operator_mode);

    // bits(from reading packages)
    read_bits += current_read_bits;

    (PackageValue::Packages(packages), read_bits)
}

fn read_operator_packages(
    package_slice: &[u8],
    operator_mode: OperatorMode,
) -> (Vec<Package>, usize) {
    let mut packages: Vec<Package> = vec![];

    let mut read_bits = 0;
    let mut finished = false;

    while !finished {
        let (package, package_read_bits) = read_package(&package_slice[read_bits..]);

        read_bits += package_read_bits;
        packages.push(package);

        match operator_mode {
            OperatorMode::Bits(size) => finished = size == (read_bits as u64),
            OperatorMode::Packages(size) => finished = size == (packages.len() as u64),
        }
    }

    (packages, read_bits)
}

fn read_literal_value(package_slice: &[u8]) -> (PackageValue, usize) {
    let mut number_binary: Vec<u8> = vec![];

    let mut bit = &package_slice[0];
    number_binary.extend_from_slice(&package_slice[1..5]);

    // 1bit(bit indicating if last block) + 5bit(value)
    let mut read_bits = 5;

    // read blocks until bit iz 0
    while *bit == 1 {
        bit = &package_slice[read_bits];
        number_binary.extend_from_slice(&package_slice[read_bits + 1..read_bits + 5]);

        // 1bit(bit indicating if last block) + 5bit(value)
        read_bits += 5;
    }

    // convert all bits to value
    let value = to_u64(&number_binary[..]);
    (PackageValue::LiteralValue(value), read_bits)
}

fn count_versions(package: &Package) -> u64 {
    let mut version_sum = package.version;

    if let PackageValue::Packages(packages) = &package.value {
        for sub_package in packages {
            version_sum += count_versions(&sub_package);
        }
    }

    version_sum
}

fn part1() {
    let (package, _) = read_package(&to_binary(read_data()));
    println!("Version sum: {}", count_versions(&package));
}

fn calculate(package: &Package) -> u64 {
    match package.id {
        PackageType::Sum => sum(&package.value),
        PackageType::Product => product(&package.value),
        PackageType::Minimum => min(&package.value),
        PackageType::Maximum => max(&package.value),
        PackageType::LiteralValue => literal_value(&package.value),
        PackageType::GreaterThan => greater_than(&package.value),
        PackageType::LessThan => less_than(&package.value),
        PackageType::EqualTo => equal(&package.value),
    }
}

fn sum(package_value: &PackageValue) -> u64 {
    if let PackageValue::Packages(packages) = package_value {
        let mut sum = 0;
        for package in packages {
            sum += calculate(&package)
        }
        sum
    } else {
        panic!("should be literal value package")
    }
}

fn product(package_value: &PackageValue) -> u64 {
    if let PackageValue::Packages(packages) = package_value {
        let mut product = 1;
        for package in packages {
            product *= calculate(&package)
        }
        product
    } else {
        panic!("should be literal value package")
    }
}

fn min(package_value: &PackageValue) -> u64 {
    if let PackageValue::Packages(packages) = package_value {
        let mut min = u64::MAX;
        for package in packages {
            let value = calculate(&package);
            if min > value {
                min = value;
            }
        }
        min
    } else {
        panic!("should be literal value package")
    }
}

fn max(package_value: &PackageValue) -> u64 {
    if let PackageValue::Packages(packages) = package_value {
        let mut max = 0;
        for package in packages {
            let value = calculate(&package);
            if max < value {
                max = value;
            }
        }
        max
    } else {
        panic!("should be literal value package")
    }
}

fn greater_than(package_value: &PackageValue) -> u64 {
    if let PackageValue::Packages(packages) = package_value {
        let value1 = calculate(&packages[0]);
        let value2 = calculate(&packages[1]);

        if value1 > value2 {
            1
        } else {
            0
        }
    } else {
        panic!("should be literal value package")
    }
}

fn less_than(package_value: &PackageValue) -> u64 {
    if let PackageValue::Packages(packages) = package_value {
        let value1 = calculate(&packages[0]);
        let value2 = calculate(&packages[1]);

        if value1 < value2 {
            1
        } else {
            0
        }
    } else {
        panic!("should be literal value package")
    }
}

fn equal(package_value: &PackageValue) -> u64 {
    if let PackageValue::Packages(packages) = package_value {
        let value1 = calculate(&packages[0]);
        let value2 = calculate(&packages[1]);

        if value1 == value2 {
            1
        } else {
            0
        }
    } else {
        panic!("should be literal value package")
    }
}

fn literal_value(package_value: &PackageValue) -> u64 {
    if let PackageValue::LiteralValue(literal_value) = package_value {
        *literal_value
    } else {
        panic!("should be literal value package")
    }
}

fn part2() {
    let (package, _) = read_package(&to_binary(read_data()));
    println!("Package result: {}", calculate(&package));
}

fn main() {
    part1();
    part2()
}
