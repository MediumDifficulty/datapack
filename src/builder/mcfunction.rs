#[derive(Clone)]
/// Defines a .mcfunction file in the ./functions directory in the datapack
pub struct MCFunction {
    pub contents: String,
    pub path: String,
    pub ran_on_load: bool,
    pub ran_on_tick: bool,
}

impl MCFunction {
    pub fn new(contents: &String, path: &String, ran_on_load: bool, ran_on_tick: bool) -> Self {
        MCFunction { contents: contents.to_string(), path: path.to_string(), ran_on_load, ran_on_tick }
    }
}