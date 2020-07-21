use std::collections::VecDeque;

fn insert_next (points: usize, circle: &mut VecDeque<usize>) -> usize {
    if circle.len() == 0 {
        (*circle).push_back(0);
    }
    if points % 23 == 0 {
        (*circle).rotate_right(7);
        let reward = points + (*circle).pop_back().unwrap();
        (*circle).rotate_left(1);
        return reward
    }
    (*circle).rotate_left(1);
    (*circle).push_back(points);
    0
}

fn high_score (players: usize, marbles: usize) -> usize {
    let mut circle: VecDeque<usize> = VecDeque::with_capacity(marbles);
    let mut scores: Vec<usize> = vec![0; players];
    let mut player_index = 0;
    for i in 1 .. marbles + 1 {
        scores[player_index] += insert_next(i, &mut circle);
        player_index += 1;
        if player_index >= players {
            player_index = 0;
        }
    }
    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert () {
        let mut circle: VecDeque<usize> = VecDeque::new();
        for i in 1 as usize .. 3 as usize {
            insert_next(i, &mut circle);
        }
        assert_eq!(circle, vec![1, 0, 2]);
        for i in 3 as usize .. 6 as usize {
            insert_next(i, &mut circle);
        }
        assert_eq!(circle, vec![1, 3, 0, 4, 2, 5]);
        for i in 6 as usize .. 26 as usize {
            insert_next(i, &mut circle);
        }
        assert_eq!(circle, vec![10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25]);
    }
    #[test]
    fn test_points () {
        let mut circle: VecDeque<usize> = VecDeque::new();
        assert_eq!(
            (1 as usize .. 26 as usize).map(|i| insert_next(i, &mut circle)).collect::<Vec<usize>>(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0]
        )
    }
    #[test]
    fn test_high_score () {
        assert_eq!(high_score(10, 1618), 8317);
        assert_eq!(high_score(13, 7999), 146373);
        assert_eq!(high_score(17, 1104), 2764);
        assert_eq!(high_score(21, 6111), 54718);
        assert_eq!(high_score(30, 5807), 37305);
    }
}

fn main () {
    let players: usize = 464;
    let marbles: usize = 7173000;
    println!("For {} players with {} marbles high score is {}", players, marbles, high_score(players, marbles));
}
