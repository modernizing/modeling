use std::collections::HashMap;

use crate::coco_struct::ClassInfo;
use crate::render::{process_name, render_member, render_method};
use crate::ParseOption;

/// Render classes info to string
pub struct PlantUmlRender;

impl PlantUmlRender {
    pub fn render(classes: &Vec<ClassInfo>, parse_option: &ParseOption) -> String {
        let mut rendered: Vec<String> = vec![];
        let mut deps: Vec<String> = vec![];

        let mut class_map: HashMap<String, bool> = HashMap::default();
        for clazz in classes {
            class_map.insert(process_name(&parse_option, &clazz.name), true);
        }

        for clazz in classes {
            let mut dep_map: HashMap<String, String> = HashMap::default();

            let members = render_member(&clazz, &mut dep_map, "", parse_option, &mut class_map);
            let mut methods = vec![];
            if !parse_option.field_only {
                methods = render_method(&clazz, &mut dep_map, "", parse_option);
            }

            let content = format!("{}{}", members.join(""), methods.join(""));
            let clazz_name = process_name(&parse_option, &clazz.name);
            if clazz.parents.len() > 0 && !parse_option.without_parent {
                for parent in &clazz.parents {
                    rendered.push(format!("{} <|-- {}", parent, clazz.name));
                }
            }

            rendered.push(format!("class {} {{\n{}}}", clazz_name, content));

            for (callee, current_clz) in dep_map {
                if callee == current_clz {
                    continue;
                }

                if class_map.get(&callee).is_none() {
                    continue;
                }

                deps.push(format!("{} -- {}\n", current_clz, callee));
            }
        }

        format!(
            "@startuml\n\n{}\n{}\n@enduml",
            rendered.join("\n\n"),
            deps.join("")
        )
    }
}
