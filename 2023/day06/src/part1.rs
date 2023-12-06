use crate::common::get_options;

pub fn run(times: &[u64], distances: &[u64]) -> u64 {
    times
        .iter()
        .enumerate()
        .map(|(n, t)| get_options(*t, distances[n]))
        .product()
}
