use minswp::Minswp;

fn main() -> Result<(), String> {
    Minswp::new().expect("could not initialize app").start()
}
