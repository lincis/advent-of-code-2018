use ndarray::{Array2, s};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::sync::Arc;

fn calc_fuel_level(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    ((rack_id * y as i32 + serial) * rack_id / 100) % 10 - 5
}

fn fill_cell_mx(serial: i32) -> Array2<i32> {
    let mut cell = Array2::<i32>::zeros((300, 300));
    for x in 0..300 {
        for y in 0..300 {
            cell[[y, x]] = calc_fuel_level(x, y, serial);
        }
    }
    cell
}

fn get_n_by_n_sum(coords: (usize, usize), n: usize, cell_mx: &Array2<i32>) -> i32 {
    cell_mx.slice(s![coords.1 .. coords.1 + n, coords.0 .. coords.0 + n]).sum()
}

fn get_max_3_by_3_sum(cell_mx: &Array2<i32>) -> (usize, usize) {
    let mut max_sum = i32::MIN;
    let mut x: usize = 0;
    let mut y: usize = 0;
    for i in 0 .. cell_mx.nrows() - 3 {
        for j in 0 .. cell_mx.ncols() - 3 {
            let sum = get_n_by_n_sum((i, j), 3, cell_mx);
            if sum > max_sum {
                max_sum = sum;
                x = i;
                y = j;
            }
        }
    }
    (x, y)
}

fn get_max_n_by_n_sum(cell_mx: Array2<i32>) -> (usize, usize, usize) {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();
    let cell_mx = Arc::new(cell_mx);
    for i_n in 0 .. 300 {
        let tx = tx.clone();
        let cell_mx = cell_mx.clone();
        pool.execute(move|| {
            let mut max_sum = i32::MIN;
            let mut x: usize = 0;
            let mut y: usize = 0;
            for i in 0 .. 300 - i_n {
                for j in 0 .. 300 - i_n {
                    let sum = get_n_by_n_sum((i, j), i_n, &cell_mx);
                    if sum > max_sum {
                        max_sum = sum;
                        x = i;
                        y = j;
                    }
                }
            }
            tx.send((max_sum, x, y, i_n)).unwrap();
        });
    }
    let mut max_sum = i32::MIN;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut n: usize = 0;
    for (new_sum, new_x, new_y, new_n) in rx.iter().take(300) {
        if new_sum > max_sum {
            max_sum = new_sum;
            x = new_x;
            y = new_y;
            n = new_n;
        }
    }
    (x, y, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fuel_level () {
        assert_eq!(calc_fuel_level(3, 5, 8), 4);
        assert_eq!(calc_fuel_level(122, 79, 57), -5);
        assert_eq!(calc_fuel_level(217, 196, 39), 0);
        assert_eq!(calc_fuel_level(101, 153, 71), 4);
    }
    #[test]
    fn test_33 () {
        assert_eq!(get_n_by_n_sum((33,45), 3, &fill_cell_mx(18)), 29);
        assert_eq!(get_n_by_n_sum((21,61), 3, &fill_cell_mx(42)), 30);
        assert_eq!(get_max_3_by_3_sum(&fill_cell_mx(18)), (33,45));
        assert_eq!(get_max_3_by_3_sum(&fill_cell_mx(42)), (21,61));
    }
    // #[test]
    // fn test_nn () {
    //     assert_eq!(get_n_by_n_sum((90,269), 16, &fill_cell_mx(18)), 113);
    //     assert_eq!(get_n_by_n_sum((232,251), 12, &fill_cell_mx(42)), 119);
    //     assert_eq!(get_max_n_by_n_sum(fill_cell_mx(18)), (90,269,16));
    //     assert_eq!(get_max_n_by_n_sum(fill_cell_mx(42)), (232,251,12));
    // }
}

fn main () {
    let power_cells = fill_cell_mx(8772);
    println!("Max 33 power level at = {:?}", get_max_3_by_3_sum(&power_cells));
    println!("Max nn power level at = {:?}", get_max_n_by_n_sum(power_cells));
}
