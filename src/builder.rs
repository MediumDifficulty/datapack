use std::{fs::File, io::Write};

use zip::{ZipWriter, write::FileOptions};

use crate::{component::Component, namespace::Namespace};


pub struct DataPackBuilder {
    namespaces: Vec<Namespace>,
    pack_format: usize,
    description: String,
}

impl DataPackBuilder {
    /// Creates a new instance of DataPackBuilder
    pub fn new() -> Self {
        DataPackBuilder { namespaces: Vec::new(), pack_format: 10, description: String::new() }
    }

    /// Sets the "pack_format" in the pack.mcmeta
    pub fn set_pack_format(&mut self, pack_format: usize) -> &mut Self {
        self.pack_format = pack_format;
        self
    }

    /// Sets the "description" in the pack.mcmeta
    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = String::from(description);
        self
    }

    pub fn add_namespace(&mut self, namespace: &Namespace) -> &mut Self {
        self.namespaces.push(namespace.to_owned());
        self
    }

    pub fn build(&self, file: &File) {
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);

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


        for namespace in &self.namespaces {
            for component in &namespace.components {
                component.write_to_file(&mut writer, &options, &namespace.name)
            }
        }

        self.create_vanilla_function_tags(&mut writer, &options);

        writer.finish().unwrap();
    }

    fn create_vanilla_function_tags(&self, writer: &mut ZipWriter<&File>, options: &FileOptions) {
        let mut load_file = String::from("{\n\t\"values\": [\n");
        let mut tick_file = String::from("{\n\t\"values\": [\n");

        for namespace in &self.namespaces {
            for component in &namespace.components {
                match component {
                    Component::Function(properties) => {
                        if properties.ran_on_load {
                            load_file.push_str(format!("\t\t\"{}:{}\",\n", namespace.name, properties.path).as_str())
                        }
            
                        if properties.ran_on_tick {
                            tick_file.push_str(format!("\t\t\"{}:{}\",\n", namespace.name, properties.path).as_str())
                        }
                    },
                    _ => continue
                }
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

