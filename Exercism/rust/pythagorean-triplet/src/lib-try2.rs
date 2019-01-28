use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut answers = HashSet::new();

    for a in 1..=(sum - 3) / 3 {
        for b in a + 1..=(sum - a - 1) / 2 {
            let c = sum - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                answers.insert([a, b, c]);
            }
        }
    }
    answers
}
