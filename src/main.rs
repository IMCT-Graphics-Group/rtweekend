use std::time::Instant;

use rtweekend::Config;

fn main() {
    let now = Instant::now();
    let config = Config::new();
    rtweekend::run(config).unwrap();
    println!("Total time: {}", now.elapsed().as_secs_f64());
}
