use models::{Data, Pixel, Range};
use std::{collections::HashSet, fs};

mod models;

fn read_data() -> Data {
    let file_name = "input";
    let input = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut parsed_algorithm = [false; 512];
    for (index, algorithm_char) in input.lines().next().unwrap().chars().enumerate() {
        if algorithm_char == '#' {
            parsed_algorithm[index] = true;
        }
    }

    let mut lit_pixels: HashSet<Pixel> = HashSet::new();
    for (pixel_y, image_line) in input.lines().skip(2).enumerate() {
        for (pixel_x, image_pixel) in image_line.chars().enumerate() {
            if image_pixel == '#' {
                lit_pixels.insert(Pixel {
                    x: pixel_x as i16,
                    y: pixel_y as i16,
                });
            }
        }
    }

    Data {
        algo: parsed_algorithm,
        lit_pixels: lit_pixels,
    }
}

fn process_cycles(data: Data, cycles: u8) {
    let mut lit_pixels = data.lit_pixels;
    for cycle_index in 0..cycles {
        lit_pixels = cycle(lit_pixels, data.algo, cycle_index);
    }

    println!("Number of lit pixels: {:}", lit_pixels.len());
}

fn cycle(lit_pixels: HashSet<Pixel>, algo: [bool; 512], cycle_index: u8) -> HashSet<Pixel> {
    let mut new_lit_pixels = HashSet::new();
    let range = get_range(&lit_pixels);

    for x in (range.min_pixel.x - 1)..(range.max_pixel.x + 2) {
        for y in (range.min_pixel.y - 1)..(range.max_pixel.y + 2) {
            let mut idx: String = String::from("");

            for t in 0..9 {
                let new_pixel = Pixel {
                    x: x + t % 3 - 1,
                    y: y + t / 3 - 1,
                };

                if range.in_range(&new_pixel) {
                    idx += if lit_pixels.contains(&new_pixel) {
                        "1"
                    } else {
                        "0"
                    };
                } else {
                    idx += if cycle_index % 2 == 1 { "1" } else { "0" };
                }
            }

            let algo_index = usize::from_str_radix(&idx, 2).unwrap();
            if algo[algo_index] {
                new_lit_pixels.insert(Pixel { x: x, y: y });
            }
        }
    }
    new_lit_pixels
}

fn get_range(lit_pixels: &HashSet<Pixel>) -> Range {
    let mut min_x = i16::MAX;
    let mut min_y = i16::MAX;
    let mut max_x = i16::MIN;
    let mut max_y = i16::MIN;

    for pixel in lit_pixels.iter() {
        if min_x > pixel.x {
            min_x = pixel.x;
        }
        if min_y > pixel.y {
            min_y = pixel.y;
        }

        if max_x < pixel.x {
            max_x = pixel.x;
        }
        if max_y < pixel.y {
            max_y = pixel.y;
        }
    }

    Range {
        min_pixel: Pixel { x: min_x, y: min_y },
        max_pixel: Pixel { x: max_x, y: max_y },
    }
}

fn part1() {
    process_cycles(read_data(), 2);
}
fn part2() {
    process_cycles(read_data(), 50);
}

fn main() {
    part1();
    part2();
}
