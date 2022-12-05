

pub fn task06() {
    let start = std::time::Instant::now();
    let text = include_str!("../tasks/task06.txt");

    

    let elapsed = start.elapsed();
    println!("Elapsed: {:.3}ms", elapsed.as_secs_f64() * 1000.0);
}