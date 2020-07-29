use crate::config::Config;
use crate::MsResult;
use reba::math::{Dimen, Point};
use std::collections::HashMap;

/// A `&'static str` array of all difficulties
pub const ALL: &[&'static str] = &[&BEGINNER, &INTERMEDIATE, &EXPERT];

const BEGINNER: &'static str = "beginner";
const INTERMEDIATE: &'static str = "intermediate";
const EXPERT: &'static str = "expert";

lazy_static! {
    static ref CONFIGS: HashMap<&'static str, DifficultyConfig> = hashmap! {
        BEGINNER => DifficultyConfig::new(point!(9, 9), 10),
        INTERMEDIATE => DifficultyConfig::new(point!(16, 16), 40),
        EXPERT => DifficultyConfig::new(point!(30, 16), 99),
    };
}

#[derive(new)]
struct DifficultyConfig {
    dimen: Dimen<usize>,
    num_mines: usize,
}

/// Applies the settings of the specified `difficulty` to the specified `config`
/// or returns and `Err(String)` if the the difficulty was not found
pub fn apply_to_config(config: &mut Config, difficulty: &str) -> MsResult {
    let d = CONFIGS
        .get(difficulty)
        .ok_or_else(|| format!("unknown difficulty: `{}`", difficulty))?;
    config.board.dimen = d.dimen;
    config.board.num_mines = d.num_mines;
    Ok(())
}
