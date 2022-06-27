use std::{fs::File, io::Write};

use flate2::{write::GzEncoder, Compression};
use nbt::{CompoundTag, encode::write_compound_tag};
use zip::{ZipWriter, write::FileOptions};

#[derive(Clone)]
pub struct Nbt {
    pub content: Vec<u8>,
    pub path: String
}

impl Nbt {
    pub fn new(content: CompoundTag, path: String) -> Self {
        let mut buf = Vec::new();
        write_compound_tag(&mut buf, &content).unwrap();

        let mut gzipped = GzEncoder::new(Vec::new(), Compression::default());
        gzipped.write_all(&buf).unwrap();

        Self { content: gzipped.finish().unwrap(), path }
    }
}

#[derive(Clone)]
pub struct MCFunction {
    pub content: Vec<u8>,
    pub path: String,
    pub ran_on_load: bool,
    pub ran_on_tick: bool
}

impl MCFunction {
    pub fn new(content: String, path: String, ran_on_load: bool, ran_on_tick: bool) -> Self {
        Self { content: content.into_bytes(), path, ran_on_load, ran_on_tick }
    }
}

#[derive(Clone)]
pub struct Json {
    pub content: Vec<u8>,
    pub path: String
}

impl Json {
    pub fn new(content: String, path: String) -> Self {
        Self { content: content.into_bytes(), path }
    }
}

#[derive(Clone)]
pub enum Component {
    Advancement(Json),
    Function(MCFunction),
    ItemModifier(Json),
    LootTable(Json),
    Predicate(Json),
    Recipe(Json),
    Structure(Nbt),

    TagBlock(Json),
    TagEntityType(Json),
    TagFluid(Json),
    TagFunction(Json),
    TagGameEvent(Json),
    TagItem(Json),
    
    Dimension(Json),
    DimensionType(Json),

    WorldGenBiome(Json),
    WorldGenConfiguredCarver(Json),
    WorldGenConfiguredFeature(Json),
    WorldGenConfiguredStructureFeature(Json),
    WorldGenConfiguredSurfaceBuilder(Json),
    WorldGenNoiseSettings(Json),
    WorldGenPlacedFeature(Json),
    WorldGenProcessorList(Json),
    WorldGenTemplatePool(Json),
}

impl Component {
    pub fn path(&self) -> &str {
        match self {
            Component::Advancement(_) =>                        "advancements",
            Component::Function(_) =>                           "functions",
            Component::ItemModifier(_) =>                       "item_modifiers",
            Component::LootTable(_) =>                          "loot_tables",
            Component::Predicate(_) =>                          "predicates",
            Component::Recipe(_) =>                             "recipes",
            Component::Structure(_) =>                          "structures",
            Component::TagBlock(_) =>                           "tags/blocks",
            Component::TagEntityType(_) =>                      "tags/entity_types",
            Component::TagFluid(_) =>                           "tags/fluids",
            Component::TagFunction(_) =>                        "tags/functions",
            Component::TagGameEvent(_) =>                       "tags/game_events",
            Component::TagItem(_) =>                            "tags/items",
            Component::Dimension(_) =>                          "dimension",
            Component::DimensionType(_) =>                      "dimension_type",
            Component::WorldGenBiome(_) =>                      "worldgen/biome",
            Component::WorldGenConfiguredCarver(_) =>           "worldgen/configured_carver",
            Component::WorldGenConfiguredFeature(_) =>          "worldgen/configured_feature",
            Component::WorldGenConfiguredStructureFeature(_) => "worldgen/configured_structure_feature",
            Component::WorldGenConfiguredSurfaceBuilder(_) =>   "worldgen/configured_surface_builder",
            Component::WorldGenNoiseSettings(_) =>              "worldgen/noise_settings",
            Component::WorldGenPlacedFeature(_) =>              "worldgen/placed_feature",
            Component::WorldGenProcessorList(_) =>              "worldgen/processor_list",
            Component::WorldGenTemplatePool(_) =>               "worldgen/template_pool",
        }
    }

    pub fn write_to_file(&self, writer: &mut ZipWriter<&File>, options: &FileOptions, namespace: &String) {
        match self {
            Component::Advancement(it) =>                        Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::Function(it) =>                     Self::write_file(writer, options, &format!("data/{}/{}/{}.mcfunction", namespace, self.path(), it.path), it.content.as_slice()),
            Component::ItemModifier(it) =>                       Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::LootTable(it) =>                          Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::Predicate(it) =>                          Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::Recipe(it) =>                             Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::Structure(it) =>                           Self::write_file(writer, options, &format!("data/{}/{}/{}.nbt", namespace, self.path(), it.path), it.content.as_slice()),
            Component::TagBlock(it) =>                           Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::TagEntityType(it) =>                      Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::TagFluid(it) =>                           Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::TagFunction(it) =>                        Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::TagGameEvent(it) =>                       Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::TagItem(it) =>                            Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::Dimension(it) =>                          Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::DimensionType(it) =>                      Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::WorldGenBiome(it) =>                      Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::WorldGenConfiguredCarver(it) =>           Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::WorldGenConfiguredFeature(it) =>          Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::WorldGenConfiguredStructureFeature(it) => Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::WorldGenConfiguredSurfaceBuilder(it) =>   Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::WorldGenNoiseSettings(it) =>              Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::WorldGenPlacedFeature(it) =>              Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::WorldGenProcessorList(it) =>              Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
            Component::WorldGenTemplatePool(it) =>               Self::write_file(writer, options, &format!("data/{}/{}/{}.json", namespace, self.path(), it.path), it.content.as_slice()),
        }
    }

    fn write_file(writer: &mut ZipWriter<&File>, options: &FileOptions, path: &String, contents: &[u8]) {
        writer.start_file(path, *options).unwrap();
        writer.write_all(contents).unwrap();
    }
}