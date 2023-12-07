pub fn get_options(time: u64, distance: u64) -> u64 {
    let mut count = 0;
    for i in 0..time {
        if (time - i) * i > distance {
            count += 1;
        }
    }

    count
}

#[test]
fn test_get_options() {
    assert_eq!(get_options(7, 9), [2, 3, 4, 5].len() as u64);
    assert_eq!(
        get_options(15, 40),
        (4..=11).collect::<Vec<_>>().len() as u64
    );
    assert_eq!(
        get_options(30, 200),
        (11..=19).collect::<Vec<_>>().len() as u64
    );
}
