#[macro_use(s)]
extern crate ndarray;
pub mod utils {
    use std::fs;
    use regex::Regex;
    pub fn read_inputs(filename: &str) -> Vec<String> {
        fs::read_to_string(filename).unwrap().split("\n").map(|s| s.to_string()).filter(|s| s.len() > 0).collect::<Vec<_>>()
    }

    pub fn parse_string_vec<T: std::str::FromStr>(input_strings: &Vec<String>, pattern: &str) -> Vec<Vec<T>> {
        let re = Regex::new(pattern).unwrap();
        input_strings.into_iter().map(|string| parse_string(&string, &re)).filter(|x| x.len() > 0).collect()
    }

    pub fn parse_string<T: std::str::FromStr>(input_string: &String, re: &Regex) -> Vec<T> {
        match re.captures(input_string) {
            Some(string) => string.iter().filter_map(|s|
                s.unwrap().as_str().parse().ok()
            ).collect(),
            None => Vec::new()
        }
    }

    pub fn count_equal_chars(_input: Vec<String>) -> Vec<i32> {
        let mut rv = Vec::new();
        for i in 0.._input.len() {
            let mut current_str = _input[i].chars().collect::<Vec<_>>();
            current_str.sort();
            // println!("current str = {:?}", current_str);
            let mut current_chr: Option<char> = None;
            let mut cnt = 1;
            let mut cnt2 = 0;
            let mut cnt3 = 0;
            for j in 0..current_str.len() {
                match current_chr {
                    Some(c) => {
                        if c == current_str[j] {
                            cnt += 1
                        }
                        else {
                            // println!("char = {}, cnt = {}", current_chr.unwrap(), cnt);
                            if cnt == 2 {
                                cnt2 += 1;
                            } else if cnt == 3 {
                                cnt3 += 1;
                            }
                            cnt = 1;
                        }
                    },
                    None => (),
                }
                current_chr = Some(current_str[j]);
            }
            if cnt == 2 {
                cnt2 += 1;
            } else if cnt == 3 {
                cnt3 += 1;
            }
            if cnt2 > 0 {
                rv.push(2);
            }
            if cnt3 > 0 {
                rv.push(3);
            }
        }
        rv
    }

    pub mod fabric {
        use ndarray::{Array2};
        #[derive(PartialEq, PartialOrd, Debug, Clone, Default)]
        pub struct Definition {
            pub id: usize,
            pub x: usize,
            pub y: usize,
            pub width: usize,
            pub height: usize
        }

        #[derive(PartialEq, PartialOrd, Debug)]
        pub struct Dimensions {
            pub nrow: usize,
            pub ncol: usize
        }

        pub fn parse_definition(_definition: &String) -> Definition {
            let mut splits: Vec<_> = _definition.split(|c| c == '#' || c == ' ' || c == '@' || c == ':' || c == ',' || c == 'x').collect();
            splits.retain(|&s| s != "");
            let parsed_defs: Vec<_> = splits.into_iter().map(|x| x.parse::<usize>().unwrap()).collect();
            Definition{id: parsed_defs[0], x: parsed_defs[1], y: parsed_defs[2], width: parsed_defs[3], height: parsed_defs[4]}
        }

        pub fn get_dims(_cuts: &Vec<Definition>) -> Dimensions {
            let mut rv = Dimensions{nrow: 0, ncol: 0};
            for i in 0.._cuts.len() {
                let ncol = _cuts[i].x + _cuts[i].width;
                let nrow = _cuts[i].y + _cuts[i].height;
                if ncol > rv.ncol {
                    rv.ncol = ncol;
                }
                if nrow > rv.nrow {
                    rv.nrow = nrow;
                }
            }
            rv
        }

        pub fn add_cut(cuts: &mut Array2<i32>, def: &Definition) {
            let mut slice = cuts.slice_mut(s![def.y .. (def.y + def.height), def.x .. (def.x + def.width)]);
            slice += 1; //ArrayView2::ones((def.height, def.width));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::*;
    use regex::Regex;
    #[test]
    fn test_inputs() {
        assert_eq!(read_inputs("inputs/day-2.txt").len(), 250);
    }
    #[test]
    fn test_count_chars() {
        assert_eq!(
            count_equal_chars(vec!["abcdfgh".into(), "abcafga".into(), "abcdabcdabc".into()]),
            vec![3,2,3]
        );
    }
    #[test]
    fn test_regex() {
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        assert_eq!(parse_string::<i32>(&"12-25".into(), &re), vec![12, 25]);
        assert_eq!(parse_string::<i32>(&"just another junk".into(), &re), vec![]);
        assert_eq!(
            parse_string_vec::<i32>(&vec!["12-25-35".into(), "79-125-0".into(), "just some junk".into()], r"(\d+)-(\d+)-(\d+)")
            , vec![vec![12, 25, 35], vec![79, 125, 0]]
        )
    }
}
#[cfg(test)]
mod fabric_tests{
    use ndarray::{Array2, arr2};
    use crate::utils::fabric::*;
    #[test]
    fn test_parse_definition() {
        assert_eq!(parse_definition(&"#1201 @ 326,252: 19x18".into()), Definition{id: 1201, x: 326, y: 252, width: 19, height: 18});
    }
    #[test]
    fn test_get_dims() {
        assert_eq!(
            get_dims(&vec![
                Definition{id: 1201, x: 326, y: 252, width: 19,  height: 18},
                Definition{id: 101,  x: 367, y: 421, width: 25,  height: 9},
                Definition{id: 1221, x: 122, y: 679, width: 254, height: 67}
            ]),
            Dimensions{nrow: 746, ncol: 392}
        );
    }
    #[test]
    fn test_get_cut() {
        let mut cuts = Array2::<i32>::zeros((10, 12));
        add_cut(&mut cuts, &Definition{id: 1234, x: 2, y: 3, width: 2, height: 4});
        assert_eq!(
            cuts,
            arr2(&[
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ])
        );
        add_cut(&mut cuts, &Definition{id: 124, x: 3, y: 2, width: 4, height: 2});
        assert_eq!(
            cuts,
            arr2(&[
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0],
                [0, 0, 1, 2, 1, 1, 1, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ])
        );
    }
}
