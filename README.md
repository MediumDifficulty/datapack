# A lightweight way to create Minecraft datapacks in rust

## Covered datapack files that can be created easily
- advancements
- functions
- item_modifiers
- loot_tables
- predicates
- recipes
- structures
- tags/blocks
- tags/entity_types
- tags/fluids
- tags/functions
- tags/game_events
- tags/items
- dimension
- dimension_type
- worldgen/biome
- worldgen/configured_carver
- worldgen/configured_feature
- worldgen/configured_structure_feature
- worldgen/configured_surface_builder
- worldgen/noise_settings
- worldgen/placed_feature
- worldgen/processor_list
- worldgen/template_pool

## Create a simple hello world datapack

```rust
use datapack::builder::DataPackBuilder;
use datapack::component::{Component, MCFunction};
use datapack::namespace::Namespace;

use std::fs::File;

let file = File::create("example.zip").unwrap();

DataPackBuilder::new()
    .add_namespace(
        Namespace::new("example")
            .add_component(Component::Function(MCFunction::new("say hello world", "hello", true, false)))
    ).build(&file)
```