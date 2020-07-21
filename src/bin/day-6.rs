use std::iter::FromIterator;
use aoc::utils::*;
use ndarray::{Array, Array2, Axis, aview1, stack};
use ndarray_stats::{QuantileExt, interpolate::Nearest};
use noisy_float::types::n64;

#[macro_use(s)]
extern crate ndarray;

fn main () {
    let input_data: Vec<_> = read_inputs("inputs/day-6.txt");
    let coordinates: Vec<Vec<i32>> = parse_string_vec::<i32>(&input_data, r"(\d+), (\d+)");
    let coordinates_nd = Array::from_shape_vec((coordinates.len(), 2), coordinates.into_iter().flatten().collect::<Vec<i32>>()).unwrap();
    // println!("{:?}", coordinates_nd);
    let quantiles = coordinates_nd.clone().quantiles_axis_mut(Axis(0), &aview1(&[n64(0.), n64(1.)]), &Nearest).unwrap();
    // println!("{:?}", coordinates_nd);
    println!("{:?}", quantiles);
    let w = (quantiles[[1, 0]] - quantiles[[0, 0]] + 1) as usize;
    let h = (quantiles[[1, 1]] - quantiles[[0, 1]] + 1) as usize;
    let w_cor = quantiles[[0, 0]];
    let h_cor = quantiles[[0, 1]];
    println!("Grid size = {} x {}", w, h);
    let mut grid = Array2::<i32>::zeros((h, w));
    let mut grid2 = Array2::<i32>::zeros((h, w));
    for i in 0 .. coordinates_nd.dim().0 {
        let x = coordinates_nd[[i, 0]] - w_cor;
        let y = coordinates_nd[[i, 1]] - h_cor;
        // println!("c = {}, {}, y = {}, x = {}", coordinates_nd[[i, 0]], coordinates_nd[[i, 1]], y, x);
        let mut slice = grid.slice_mut(s![y, x]);
        slice += i as i32 + 1;
    }
    for i in 0 .. grid.dim().0 {
        for j in 0 .. grid.dim().1 {
            // let distances = (coordinates_nd.clone() - Array2::<i32>::from_shape_vec((1, 2), vec![i as i32 + h_cor, j as i32 + w_cor]).unwrap()).mapv(i32::abs).sum_axis(Axis(1));
            let distances = Array::from_iter(coordinates_nd.axis_iter(Axis(0)).map(|x| i32::abs(x[0] - w_cor - j as i32) + i32::abs(x[1] - h_cor - i as i32)));
            // println!("{}, {} distances = {:?}", i, j, distances);
            let min_distance = distances.min().unwrap();
            // println!("{}, {} min_distance = {:?}", i, j, min_distance);
            grid2[[i, j]] = distances.sum();
            if min_distance == &0 {
                // println!("skip i = {}, j = {}", i + h_cor as usize, j + w_cor as usize);
                // grid2[[i, j]] = i32::MAX;
                continue;
            }
            if distances.into_iter().filter(|x| x == &min_distance).collect::<Vec<_>>().len() == 1 {
                grid[[i, j]] = 1 + distances.argmin().unwrap() as i32;
            }
        }
    }
    println!("{:?}", grid);
    let borders = stack![Axis(0), grid.slice(s![0, ..]), grid.slice(s![h - 1, ..]), grid.slice(s![.., 0]), grid.slice(s![.., w - 1])];
    println!("Borders = {:?}", borders);
    let mut max_area = 0;
    for i in 1 .. (coordinates_nd.dim().0 + 1) {
        if borders.into_iter().filter(|&x| x == &(i as i32)).collect::<Vec<_>>().len() > 0 {
            continue;
        }
        let curr_area = grid.into_iter().filter(|&x| x == &(i as i32)).collect::<Vec<_>>().len();
        if curr_area > max_area {
            println!("i = {}, area = {}", i, curr_area);
            max_area = curr_area;
        }
    }
    println!("Max area = {}", max_area);
    println!("Area w total distance < 10000 = {}", grid2.into_iter().filter(|&x| x < &10000).collect::<Vec<_>>().len());
}
