use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum / 3)
        .flat_map(|a| {
            (a + 1..=(sum - a - 1) / 2)
                .map(move |b| [a, b, sum - a - b])
                .filter(|r| r[0].pow(2) + r[1].pow(2) == r[2].pow(2))
        })
        .collect::<HashSet<[u32; 3]>>()
}
