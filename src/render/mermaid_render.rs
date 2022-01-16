use crate::coco_struct::ClassInfo;
use std::collections::HashMap;
use crate::ParseOption;
use crate::render::{render_member, render_method};

/// Render classes info to string
pub struct MermaidRender;

impl MermaidRender {
    pub fn render(classes: &Vec<ClassInfo>, parse_option: &ParseOption) -> String {
        let space = "    ";
        let mut rendered: Vec<String> = vec![];
        let mut deps: Vec<String> = vec![];

        let mut class_map: HashMap<String, bool> = HashMap::default();
        for clazz in classes {
            class_map.insert(clazz.name.clone(), true);
        }

        for clazz in classes {
            let mut dep_map: HashMap<String, String> = HashMap::default();

            let members = render_member(&clazz, &mut dep_map, space);
            let mut methods = vec![];
            if !parse_option.field_only {
                methods = render_method(&clazz, &mut dep_map, space);
            }

            let content = format!("{}{}", members.join(""), methods.join(""));
            let class_field = clazz.name.clone();
            if clazz.parents.len() > 0 && !parse_option.without_parent {
                rendered.push(format!("{}{} <|-- {}", space, clazz.parents.join(","), clazz.name));
            }

            rendered.push(format!("{}class {} {{\n{}{}}}", space, class_field, content, space));

            for (callee, current_clz) in dep_map {
                if callee == current_clz {
                    continue;
                }

                if class_map.get(&callee).is_none() {
                    continue;
                }

                deps.push(format!("{}{} -- {}\n", space, current_clz, callee));
            }
        }

        format!(
            "{}\n{}",
            rendered.join("\n\n"),
            deps.join("")
        )
    }
}
