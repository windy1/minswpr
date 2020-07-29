use std::collections::HashMap;

pub type Model = AsRef<dyn Any>;
type ModelMap = HashMap<&'static str, Box<Model>>;

#[derive(new)]
pub struct Models {
    #[new(default)]
    models: ModelMap,
}
