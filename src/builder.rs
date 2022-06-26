pub mod mcfunction;
pub mod structure;

use std::{fs::File, io::Write};

use zip::{ZipWriter, write::FileOptions};

use self::{mcfunction::MCFunction, structure::Structure};

/// # Examples
/// 
/// ```
/// use datapack::builder::DataPackBuilder;
/// 
/// let mut pack = DataPackBuilder::new();
/// 
/// pack.set_pack_format(10)
///     .set_name(String::from("my datapack"))
///     .set_description(String::from("my informative description"));
/// ```
pub struct DataPackBuilder {
    mc_functions: Vec<MCFunction>,
    structures: Vec<Structure>,
    pack_format: usize,
    description: String,
    name: String,
}

impl DataPackBuilder {
    /// Creates a new instance of DataPackBuilder
    pub fn new() -> Self {
        DataPackBuilder { mc_functions: Vec::new(), structures: Vec::new(), pack_format: 10, description: String::new(), name: String::new() }
    }

    /// Adds a .mcfunction file to the datapack
    /// 
    /// # Examples
    /// 
    /// ```
    /// use datapack::builder::DataPackBuilder;
    /// use datapack::builder::mcfunction::MCFunction;
    /// 
    /// let mut pack = DataPackBuilder::new();
    /// 
    /// let mc_function = MCFunction::new(&String::from("say hi"), &String::from("hello"), true, false);
    /// 
    /// pack.add_mc_function(mc_function);
    /// ```
    pub fn add_mc_function(&mut self, function: MCFunction) -> &mut Self {
        self.mc_functions.push(function);
        self
    }

    /// Adds a structure file to the datapack.
    /// To use this you must create a new Structure, then give it a gzipped byte Vec containing the structure in the nbt file format
    pub fn add_structure(&mut self, structure: Structure) -> &mut Self {
        self.structures.push(structure);
        self
    }

    /// Sets the "pack_format" in the pack.mcmeta
    pub fn set_pack_format(&mut self, pack_format: usize) -> &mut Self {
        self.pack_format = pack_format;
        self
    }

    /// Sets the "description" in the pack.mcmeta
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = description;
        self
    }

    /// Sets the namespace name in the datapack
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Builds the datapack
    /// 
    /// # Examples
    /// 
    /// ```
    /// use datapack::builder::DataPackBuilder;
    /// use std::fs::File;
    /// 
    /// let file = File::create("my_datapack.zip").unwrap();
    /// 
    /// DataPackBuilder::new()
    ///     .set_pack_format(10)
    ///     .set_name(String::from("my_datapack"))
    ///     .set_description(String::from("my informative description"))
    ///     .build(&file);
    /// ```
    pub fn build(&self, file: &File) {
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Zstd);

        let mut writer = ZipWriter::new(file);


        // Create  the pack.mcmeta file
        writer.start_file("pack.mcmeta", options).unwrap();
        writer.write_all(format!(
            "{{
    \"pack\": {{
        \"pack_format\": {},
        \"description\": \"{}\"
    }}
}}",        self.pack_format, self.description)
                .as_bytes()).unwrap();


        self.add_mc_function_files(&mut writer, &options);
        self.create_vanilla_function_tags(&mut writer, &options);
        self.add_structure_files(&mut writer, &options);

        writer.finish().unwrap();
    }

    fn add_structure_files(&self, writer: &mut ZipWriter<&File>, options: &FileOptions) {
        for structure in &self.structures {
            writer.start_file(format!("data/{}/structures/{}.nbt", self.name, structure.path).as_str(), *options).unwrap();
            writer.write_all(structure.contents.as_slice()).unwrap();
        }
    }

    fn add_mc_function_files(&self, writer: &mut ZipWriter<&File>, options: &FileOptions) {
        for function in &self.mc_functions {
            writer.start_file(format!("data/{}/functions/{}.mcfunction", self.name, function.path).as_str(), *options).unwrap();
            writer.write_all(function.contents.as_bytes()).unwrap();
        }
    }
    
    fn create_vanilla_function_tags(&self, writer: &mut ZipWriter<&File>, options: &FileOptions) {
        let mut load_file = String::from("{\n\t\"values\": [\n");
        let mut tick_file = String::from("{\n\t\"values\": [\n");

        for function in &self.mc_functions {
            if function.ran_on_load {
                load_file.push_str(format!("\t\t\"{}:{}\",\n", self.name, function.path).as_str())
            }

            if function.ran_on_tick {
                tick_file.push_str(format!("\t\t\"{}:{}\",\n", self.name, function.path).as_str())
            }
        }

        if load_file.chars().rev().nth(1) != Some('[') {
            load_file.pop();
            load_file.pop();
        }

        load_file.push_str("\n\t]\n}");

        if tick_file.chars().rev().nth(1) != Some('[') {
            tick_file.pop();
            tick_file.pop();
        }

        tick_file.push_str("\n\t]\n}");

        writer.start_file("data/minecraft/tags/functions/load.json", *options).unwrap();
        writer.write_all(load_file.as_bytes()).unwrap();

        writer.start_file("data/minecraft/tags/functions/tick.json", *options).unwrap();
        writer.write_all(tick_file.as_bytes()).unwrap();
    }
}

