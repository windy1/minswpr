use minswpr::Minswpr;

fn main() -> Result<(), String> {
    Minswpr::from_config("minswpr.toml")?.start()
}
