use rtweekend::Config;

fn main() {
    let config = Config::new();
    rtweekend::run(config).unwrap();
}
