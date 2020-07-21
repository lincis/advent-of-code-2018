use std::env;
#[macro_use(s)]
extern crate ndarray;

pub mod days {
    use aoc::utils::*;
    use distance::levenshtein;
    use ndarray::Array2;
    pub fn day2() {
        let input_data = read_inputs("inputs/day-2.txt");
        let occurences = count_equal_chars(input_data.clone());
        let mut cnt2 = 0;
        let mut cnt3 = 0;
        for i in 0..occurences.len() {
            if occurences[i] == 2 {
                cnt2 += 1;
            } else if occurences[i] == 3 {
                cnt3 += 1;
            }
        }
        println!("checksum = {}", cnt2 * cnt3);
        for i in 0..input_data.len() {
            for j in 0..input_data.len() {
                if i == j {
                    continue;
                }
                if 1 == levenshtein(&input_data[i], &input_data[j]) {
                    println!("close match: {} vs {}", input_data[i], input_data[j]);
                    let a: Vec<_> = input_data[i].chars().collect();
                    let b: Vec<_> = input_data[j].chars().collect();
                    let mut m = Vec::new();
                    for k in 0..a.len() {
                        if a[k] == b[k] {
                            m.push(a[k]);
                        }
                    }
                    println!("common chars = {}", m.into_iter().collect::<String>());
                }
            }
        }
    }
    pub fn day3() {
        let input_data: Vec<_> = read_inputs("inputs/day-3.txt")
            .into_iter()
            .map(|s| fabric::parse_definition(&s))
            .collect();
        let dims = fabric::get_dims(&input_data);
        println!("Dims = {:?}", dims);
        let mut fabric: Array2<i32> = Array2::zeros((dims.nrow, dims.ncol));
        for i in 0..input_data.len() {
            fabric::add_cut(&mut fabric, &input_data[i]);
        }
        println!(
            "Sq inches used more than once = {}",
            fabric.map(|&x| if x > 1 { 1 } else { 0 }).sum()
        );
        for i in 0..input_data.len() {
            let slice = fabric.slice(s![
                input_data[i].y..(input_data[i].y + input_data[i].height),
                input_data[i].x..(input_data[i].x + input_data[i].width)
            ]);
            if slice.sum() == (input_data[i].width * input_data[i].height) as i32 {
                println!("Cut that does not overlap = {}", input_data[i].id);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    match args[1].as_str() {
        "day2" => days::day2(),
        "day3" => days::day3(),
        // "day4" => days::day4(),
        &_ => (),
    }
}
