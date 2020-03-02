use minswpr::config;
use minswpr::Minswpr;

fn main() -> Result<(), String> {
    let config = config::resolve()?;
    println!("using config: `{}`", config.display());
    Minswpr::from(config).start()
}
