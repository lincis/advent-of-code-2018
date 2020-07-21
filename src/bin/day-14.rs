fn next_index (length: usize, current: usize, increment: usize) -> usize {
    let index = (current + (increment % length)) % length;
    assert!(index < length);
    index
}

struct DwarfCooks {
    pos1: usize,
    pos2: usize,
    scoreboard: Vec<usize>,
}

impl Default for DwarfCooks {
    fn default() -> Self {
        DwarfCooks {
            pos1: 0,
            pos2: 1,
            scoreboard: vec![3, 7],
        }
    }
}

impl DwarfCooks {
    fn combine(&mut self) {
        let new_score = self.scoreboard[self.pos1] + self.scoreboard[self.pos2];
        if new_score > 9 {
            self.scoreboard.push(1);
        }
        self.scoreboard.push(new_score % 10);
        // println!("{} => {:?}", new_score, self.scoreboard);
        self.pos1 = next_index(self.scoreboard.len(), self.pos1, self.scoreboard[self.pos1] + 1);
        self.pos2 = next_index(self.scoreboard.len(), self.pos2, self.scoreboard[self.pos2] + 1);
    }
}

fn get_scores (preceding: usize) -> String {
    let mut cooks = DwarfCooks::default();
    while cooks.scoreboard.len() < preceding + 10 {
        cooks.combine();
    }
    cooks.scoreboard[preceding .. preceding + 10].iter().map(|s| s.to_string()).collect()
}

fn first_occurence (pattern: &str, steps: usize) -> usize {
    let mut cooks = DwarfCooks::default();
    loop {
        cooks.scoreboard.reserve((steps as f64 * 1.6) as usize);
        for _ in 0 .. steps {
            cooks.combine();
        }
        let result = cooks.scoreboard.iter().map(|s| s.to_string()).collect::<String>().find(pattern);
        match result {
            Some(receipes) => return receipes,
            None => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_index () {
        assert_eq!(next_index(7, 4, 5), 2);
        assert_eq!(next_index(7, 4, 9), 6);
        assert_eq!(next_index(7, 4, 1), 5);
        assert_eq!(next_index(7, 4, 3), 0);
        assert_eq!(next_index(7, 4, 10), 0);
    }
    #[test]
    fn test_combine () {
        let mut cooks = DwarfCooks::default();
        cooks.combine();
        assert_eq!(cooks.scoreboard, vec![3, 7, 1, 0]);
        cooks.combine();
        assert_eq!(cooks.scoreboard, vec![3, 7, 1, 0, 1, 0]);
        cooks.combine();
        assert_eq!(cooks.scoreboard, vec![3, 7, 1, 0, 1, 0, 1]);
        cooks.combine();
        assert_eq!(cooks.scoreboard, vec![3, 7, 1, 0, 1, 0, 1, 2]);
    }
    #[test]
    fn test_iters () {
        assert_eq!(get_scores(9), "5158916779");
        assert_eq!(get_scores(5), "0124515891");
        assert_eq!(get_scores(18), "9251071085");
        assert_eq!(get_scores(2018), "5941429882");
    }
    #[test]
    fn test_occurence () {
        assert_eq!(first_occurence("51589", 10000), 9);
        assert_eq!(first_occurence("01245", 10000), 5);
        assert_eq!(first_occurence("92510", 10000), 18);
        assert_eq!(first_occurence("59414", 10000), 2018);
    }
}

fn main () {
    println!("Seq after 793031 = {}", get_scores(793031));
    println!("793031 is found after {} reciepes", first_occurence("793031", 10000000));
}
