pub fn add_loop() -> usize {
    let mut count = 0;
    for _ in 0..1000_000_000 {
        count += 1;
    }
    count
}
