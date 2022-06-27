#[cfg(test)]
mod tests {
    use std::fs::File;

    use datapack::{DataPackBuilder, component::{Component, MCFunction}, namespace::Namespace};

    #[test]
    fn build() {
        let file = File::create("test.zip").unwrap();

        DataPackBuilder::new()
            .add_namespace(
                Namespace::new("test")
                    .add_component(Component::Function(MCFunction::new("say hi", &String::from("hello"), true, false)))
            ).add_namespace(
                Namespace::new("test2")
                    .add_component(Component::Function(MCFunction::new("say hi2", "custom/hello", true, false)))
            ).build(&file)
    }
}