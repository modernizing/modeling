use crate::coco_struct::ClassInfo;
use std::collections::HashMap;
use crate::render::plantuml_render::{PlantUmlRender};

/// Render classes info to string
pub struct MermaidRender;

impl MermaidRender {
    pub fn render(classes: &Vec<ClassInfo>) -> String {
        let mut rendered: Vec<String> = vec![];
        let mut deps: Vec<String> = vec![];

        let mut class_map: HashMap<String, bool> = HashMap::default();
        for clazz in classes {
            class_map.insert(clazz.name.clone(), true);
        }

        for clazz in classes {
            let mut dep_map: HashMap<String, String> = HashMap::default();

            let members = PlantUmlRender::render_member(&clazz, &mut dep_map);
            let methods = PlantUmlRender::render_method(&clazz, &mut dep_map);

            let content = format!("{}{}", members.join(""), methods.join(""));
            let mut class_field = clazz.name.clone();
            if clazz.parents.len() > 0 {
                class_field = format!("{} extends {}", clazz.name, clazz.parents.join(","));
            }

            rendered.push(format!("class {} {{\n{}}}", class_field, content));

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
            "\n{}\n{}",
            rendered.join("\n\n"),
            deps.join("")
        )
    }
}