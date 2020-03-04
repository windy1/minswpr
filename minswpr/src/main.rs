use minswpr::config;
use minswpr::Minswpr;
use minswpr::MsResult;

fn main() -> MsResult {
    let config = config::resolve()?;
    println!("using config: `{}`", config.display());
    Minswpr::from(config).start()
}
