use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1u32..sum / 3u32)
        .into_par_iter()
        .filter_map(|a| {
            let b_plus_c = sum - a;
            let is_whole_number: bool = (b_plus_c.pow(2) - a.pow(2)) % (2 * b_plus_c) == 0;
            let b = (b_plus_c.pow(2) - a.pow(2)) / (2 * b_plus_c);

            if b > a && is_whole_number {
                Some([a, b, b_plus_c - b])
            } else {
                None
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}
