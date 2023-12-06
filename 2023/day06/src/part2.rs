use crate::common::get_options;

pub fn run(times: &[u64], distances: &[u64]) -> u64 {
    let time = times
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = distances
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    get_options(time, distance)
}

#[test]
fn test_join_inputs() {
    let time: [i32; 3] = [7, 15, 30];
    let joined = time
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    assert_eq!(joined, 71530);
}
