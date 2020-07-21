use aoc::utils::*;
use ndarray::{Array2, Axis, s};
use std::collections::HashMap;

struct Build {
    letters: Vec<char>,
    order: Array2<i32>,
    jobs: HashMap<char, usize>,
    workers: usize,
    done: Vec<char>,
    tics: usize
}

impl Build {
    fn assign_jobs (&mut self) {
        while self.jobs.len() < self.workers {
            let next_i = get_next_i(&self.order);
            match next_i {
                Some(i) => {
                    self.jobs.insert(self.letters[i], build_cost(&self.letters[i], &self.letters).unwrap());
                    self.order[[i, i]] = 1;
                    println!("Adding job for '{}'", self.letters[i]);
                },
                None => break,
            }
        }
    }
    fn tic(&mut self) {
        self.assign_jobs();
        for (key, val) in self.jobs.iter_mut() {
            *val -= 1;
            if *val == 0 {
                let i = self.letters.iter().position(|c| c == key).unwrap();
                remove_deps(&mut self.order, i);
                self.done.push(*key);
                println!("Removed job for '{}'", key);
            }
        }
        self.jobs.retain(|_, &mut val| val > 0);
        self.tics += 1;
    }
}

fn build_cost (letter: &char, all_letters: &Vec<char>) -> Option<usize> {
    if all_letters.contains(letter) {
        Some(all_letters.iter().position(|c| c == letter).unwrap() + 61)
    } else {
        None
    }
}

fn get_next_i (order: &Array2<i32>) -> Option<usize> {
    order.sum_axis(Axis(0)).iter().position(|&x| x == 0)
}

fn remove_deps (order: &mut Array2<i32>, i: usize) {
    order.slice_mut(s!(i, ..)).fill(0);
    order[[i, i]] = 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{arr2};
    #[test]
    fn test_costs() {
        assert_eq!(build_cost(&'A', &vec!['A', 'B', 'C', 'D']), Some(61));
        assert_eq!(build_cost(&'B', &vec!['A', 'B', 'C', 'D']), Some(62));
        assert_eq!(build_cost(&'C', &vec!['A', 'B', 'C', 'D']), Some(63));
        assert_eq!(build_cost(&'D', &vec!['A', 'B', 'C', 'D']), Some(64));
        assert_eq!(build_cost(&'F', &vec!['A', 'B', 'C', 'D']), None);
    }
    #[test]
    fn test_order() {
        assert_eq!(get_next_i(&arr2(&[[1, 0, 0], [0, 0, 0], [1, 1, 0]])), Some(2));
        assert_eq!(get_next_i(&arr2(&[[1, 0, 0], [0, 0, 1], [1, 1, 0]])), None);
    }
    #[test]
    fn test_remove_deps() {
        let mut order = arr2(&[[1, 0, 0], [0, 0, 0], [1, 1, 0]]);
        remove_deps(&mut order, 2);
        assert_eq!(order, arr2(&[[1, 0, 0], [0, 0, 0], [0, 0, 1]]))
    }
}

fn main() {
    let input_data: Vec<_> = read_inputs("inputs/day-7.txt");
    let order_def = parse_string_vec::<char>(&input_data, r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.");
    let mut all_letters: Vec<char> = Vec::new();
    for od in &order_def {
        if !all_letters.contains(&od[0]) {
            all_letters.push(od[0].clone());
        }
        if !all_letters.contains(&od[1]) {
            all_letters.push(od[1].clone());
        }
    }
    all_letters.sort();
    println!("All letters = {:?}", all_letters);
    let mut order_matrix = Array2::<i32>::zeros((all_letters.len(), all_letters.len()));
    for od in &order_def {
        let i_row = all_letters.iter().position(|&c| c == od[0]).unwrap();
        let i_col = all_letters.iter().position(|&c| c == od[1]).unwrap();
        order_matrix[[i_row, i_col]] = 1;
    }
    println!("Order matrix:");
    println!("{:?}", order_matrix);
    let mut builder = Build{
        letters: all_letters.clone(),
        order: order_matrix.clone(),
        jobs: HashMap::new(),
        workers: 5,
        done: Vec::new(),
        tics: 0
    };
    // let dependencies = order_matrix.sum_axis(Axis(1));
    // println!("{:?}", dependencies);
    // let mut current_i = dependencies.iter().position(|&x| x == 0).unwrap();
    let mut final_order: Vec<usize> = Vec::new();
    let mut iters: usize = 0;
    while final_order.len() < all_letters.len() {
        let current_i = get_next_i(&order_matrix).unwrap();
        // println!("found zeros at {:?}", current_i);
        // println!("row slice before = {:?}", order_matrix.slice(s![current_i, ..]));
        // println!("col slice before = {:?}", order_matrix.slice(s![.., current_i]));
        final_order.push(current_i);
        remove_deps(&mut order_matrix, current_i);
        // println!("row slice after  = {:?}", order_matrix.slice(s![current_i, ..]));
        // println!("col slice after  = {:?}", order_matrix.slice(s![.., current_i]));
        iters += 1;
        if iters > 1000 {
            println!("Stopping as iters = {}", iters);
            break;
        }
    }
    let final_str: String = final_order.iter().map(|&i| all_letters[i]).collect();
    println!("Final string = {}", final_str);
    while builder.done.len() < all_letters.len() {
        // println!("{:?}", builder.tics);
        builder.tic();
        if builder.tics > 10000 {
            break;
        }
    }
    println!("Final tics = {}", builder.tics);
}
