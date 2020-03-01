use minswpr::Minswpr;

fn main() -> Result<(), String> {
    Minswpr::from("minswpr.toml").start()
}
