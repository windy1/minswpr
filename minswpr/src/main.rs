use clap::Clap;
use minswpr::config;
use minswpr::math::{Dimen, Point};
use minswpr::point;
use minswpr::Minswpr;
use minswpr::MsResult;
use std::path::PathBuf;

fn main() -> MsResult {
    let opts = Opts::parse();
    let config = opts
        .config
        .map(PathBuf::from)
        .unwrap_or_else(|| config::resolve().unwrap());

    println!("using config: `{}`", config.display());

    let mut config = config::read_config(config)?;

    let Dimen { x: cw, y: ch } = config.board.dimen;
    config.board.dimen = point!(
        opts.width.unwrap_or_else(|| cw),
        opts.height.unwrap_or_else(|| ch)
    );

    let num_mines = config.board.num_mines;
    config.board.num_mines = opts.num_mines.unwrap_or_else(|| num_mines);

    Minswpr::new(config)?.start()
}

/// A clone of Microsoft's classic Minesweeper, because why not
#[derive(Clap)]
struct Opts {
    #[clap(long = "config")]
    config: Option<String>,
    #[clap(short = "w", long = "width")]
    width: Option<usize>,
    #[clap(short = "h", long = "height")]
    height: Option<usize>,
    #[clap(short = "m", long = "num-mines")]
    num_mines: Option<usize>,
}
