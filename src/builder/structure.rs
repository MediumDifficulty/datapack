#[derive(Clone)]
/// Defines a structure file in the ./structures directory in the datapack
pub struct Structure {
    pub contents: Vec<u8>,
    pub path: String,
}

impl Structure {
    pub fn new(contents: &Vec<u8>, path: &String) -> Self {
        Structure { contents: contents.to_vec(), path: path.to_string() }
    }
}