use crate::coco_struct::ClassInfo;
use crate::render::{process_name, render_member, render_method};
use crate::ParseOption;
use std::collections::HashMap;

/// Render classes info to string
pub struct MermaidRender;

impl MermaidRender {
    pub fn render(classes: &[ClassInfo], parse_option: &ParseOption) -> String {
        let space = "    ";
        let mut rendered: Vec<String> = vec![];
        let mut deps: Vec<String> = vec![];

        let mut class_map: HashMap<String, bool> = HashMap::default();
        for clazz in classes {
            class_map.insert(process_name(&parse_option, &clazz.name), true);
        }

        for clazz in classes {
            let mut dep_map: HashMap<String, String> = HashMap::default();

            let members = render_member(&clazz, &mut dep_map, space, parse_option, &mut class_map);
            let mut methods = vec![];
            if !parse_option.field_only {
                methods = render_method(&clazz, &mut dep_map, space, parse_option);
            }

            let content = format!("{}{}", members.join(""), methods.join(""));
            let class_name = process_name(&parse_option, &clazz.name);
            if clazz.parents.len() > 0 && !parse_option.without_parent {
                rendered.push(format!(
                    "{}{} <|-- {}",
                    space,
                    clazz.parents.join(","),
                    class_name
                ));
            }

            rendered.push(format!(
                "{}class {} {{\n{}{}}}",
                space, class_name, content, space
            ));

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

        format!("{}\n{}", rendered.join("\n\n"), deps.join(""))
    }
}
