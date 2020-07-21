use aoc::utils::*;
use std::collections::HashMap;

fn do_grow(pots: &mut Vec<bool>, grow: &HashMap<Vec<bool>, bool>) -> i32 {
    let mut add_new = 0;
    while pots[0..5].iter().filter(|&b| b == &false).count() < 5 {
        add_new += 1;
        pots.insert(0, false);
    }
    while pots[pots.len() - 5..]
        .iter()
        .filter(|&b| b == &false)
        .count()
        < 5
    {
        pots.push(false);
    }
    let ref_pots = (*pots).clone();
    for i in 2..pots.len() - 3 {
        let slice = &ref_pots[i - 2..i + 3];
        // println!("{:?}", slice);
        if grow.contains_key(slice) {
            pots[i] = grow[slice];
            // println!("{:?} => {}")
        } else {
            pots[i] = false;
        }
        // println!("{:?} => {}", print_pots(&ref_pots[i - 2..i + 3].to_vec()), pots[i]);
    }
    add_new
}

fn parse_pots(definition: &String) -> Vec<bool> {
    definition[15 ..].chars().map(|c| c == '#').collect()
}

fn parse_grow(definitions: &Vec<String>) -> HashMap<Vec<bool>, bool> {
    let mut grow: HashMap<Vec<bool>, bool> = HashMap::new();
    for i in 1..definitions.len() {
        grow.insert(
            definitions[i][..5].chars().map(|c| c == '#').collect(),
            definitions[i][9..10] == "#".to_string(),
        );
    }
    grow
}

fn sum_pot_pos(pots: &Vec<bool>, shift: i32) -> i32 {
    let mut sum = 0;
    for i in 0..pots.len() {
        if pots[i] {
            sum += i as i32 + shift;
        }
    }
    sum
}

fn print_pots(pots: &Vec<bool>) -> String {
    pots.iter().map(|&p| if p { '#' } else { '.' }).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    fn sample_def() -> Vec<String> {
        vec![
            "initial state: #..#.#..##......###...###".into(),
            "...## => #".into(),
            "..#.. => #".into(),
            ".#... => #".into(),
            ".#.#. => #".into(),
            ".#.## => #".into(),
            ".##.. => #".into(),
            ".#### => #".into(),
            "#.#.# => #".into(),
            "#.### => #".into(),
            "##.#. => #".into(),
            "##.## => #".into(),
            "###.. => #".into(),
            "###.# => #".into(),
            "####. => #".into(),
        ]
    }
    #[test]
    fn test_parse_pots() {
        assert_eq!(
            parse_pots(&"initial state: .##..###.#".to_string()),
            vec![false, true, true, false, false, true, true, true, false, true]
        );
    }
    // #[test]
    // fn test_parse_grow() {
    //     assert_eq!(
    //         parse_pots(&".##..###.#".to_string()),
    //         vec![false, true, true, false, false, true, true, true, false, true]
    //     );
    // }
    #[test]
    fn test_sample_flow() {
        let raw_input = sample_def();
        // let mut pots: Vec<bool> = raw_input[0][15..].chars().map(|c| c == '#').collect();
        let mut pots = parse_pots(&raw_input[0]);
        let grow = parse_grow(&raw_input);
        let mut shift = 0;
        for i in 0..20 {
            println!("{}: {:?}", i, print_pots(&pots));
            shift -= do_grow(&mut pots, &grow);
            // println!("{}: {:?}", i, print_pots(&pots));
        }
        println!("{:?}", print_pots(&pots));
        println!("shift = {:?}", shift);
        assert_eq!(sum_pot_pos(&pots, shift), 325);
    }
}

fn main() {
    let raw_input = read_inputs("inputs/day-12.txt");
    // let mut pots: Vec<bool> = raw_input[0][15..].chars().map(|c| c == '#').collect();
    let mut pots = parse_pots(&raw_input[0]);
    let grow = parse_grow(&raw_input);
    let mut shift = 0;
    let mut last = 0;
    for i in 0 .. 10000 {
        // println!("{}: {:?}", i, print_pots(&pots));
        shift -= do_grow(&mut pots, &grow);
        // println!("{}: {:?}", i, print_pots(&pots));
        if i % 1000 == 0 {
            let current = sum_pot_pos(&pots, shift);
            println!("{}: sum = {}, diff = {}", i, current, current - last);
            last = current;
        }
    }
    // println!("{:?}", print_pots(&pots));
    // println!("shift = {:?}", shift);
    let sum = sum_pot_pos(&pots, shift) as i64;
    println!("sum = {}", sum);
    println!("sum 50bn = {}", sum + (50000000000i64 - 10000i64) * 52000i64 / 1000i64);
}
