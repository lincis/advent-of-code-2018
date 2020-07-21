use std::fs::read_to_string;

fn remove_poly_colisions (polymer: &mut Vec<char>) {
    let mut i: usize = 1;
    loop {
        while
            polymer[i].to_lowercase().next().unwrap() == polymer[i - 1].to_lowercase().next().unwrap() &&
            polymer[i] != polymer[i - 1]
        {
            let _a = polymer.remove(i - 1);
            let _b = polymer.remove(i - 1);
            // println!("Removing {} and {}", _a, _b);
            if i > 1 {
                // println!("i = {}", i);
                i -= 1;
            }
            if i >= polymer.len() {
                break;
            }
        }
        i += 1;
        if i >= polymer.len() {
            break;
        }
    }
}

fn main() {
    let mut polymer: Vec<char> = read_to_string("inputs/day-5.txt")
        .unwrap()
        .chars()
        .collect();
    println!("Popping trailing newline {}", polymer.pop().unwrap());
    println!("{} elements remain in polymer", polymer.len());
    remove_poly_colisions(&mut polymer);
    println!("{} elements remain in polymer", polymer.len());
    let mut unique_chars: Vec<char> = polymer.clone().into_iter().map(|c| c.to_lowercase().next().unwrap()).collect();
    unique_chars.sort();
    unique_chars.dedup();
    println!("Unique chars = {:?}", unique_chars);
    for c in unique_chars {
        let mut reduced_polymer: Vec<char> = polymer.clone().into_iter().filter(|x| x.to_lowercase().next().unwrap() != c).collect();
        remove_poly_colisions(&mut reduced_polymer);
        println!("{} elements remain in polymer after removing {} and collapsing", reduced_polymer.len(), c);
    }
}
