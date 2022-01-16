use std::collections::HashMap;

use crate::coco_struct::ClassInfo;
use crate::ParseOption;
use crate::render::{render_member, render_method};

/// Render classes info to string
pub struct GraphvizRender;

impl GraphvizRender {
    pub fn render(classes: &Vec<ClassInfo>, parse_option: &ParseOption) -> String {
        let mut sub_graphs_map: HashMap<String, Vec<String>> = HashMap::default();
        let mut deps: Vec<String> = vec![];

        let mut class_map: HashMap<String, bool> = HashMap::default();
        for clazz in classes {
            class_map.insert(clazz.name.clone(), true);
        }

        for clazz in classes {
            let mut dep_map: HashMap<String, String> = HashMap::default();

            // todo: add parameter for mvc
            if clazz.name.ends_with("Repository") {
                let graph = sub_graphs_map.entry("Repository".to_string()).or_insert(vec![]);
                graph.push(clazz.name.to_string());
            }

            if clazz.name.ends_with("Controller") {
                let graph = sub_graphs_map.entry("Controller".to_string()).or_insert(vec![]);
                graph.push(clazz.name.to_string());
            }

            if clazz.name.ends_with("Service") {
                let graph = sub_graphs_map.entry("Service".to_string()).or_insert(vec![]);
                graph.push(clazz.name.to_string());
            }

            let _ = render_member(&clazz, &mut dep_map, "", parse_option);
            if !parse_option.field_only {
                let _ = render_method(&clazz, &mut dep_map, "");
            }

            for (callee, current_clz) in dep_map {
                if callee == current_clz {
                    continue;
                }

                if class_map.get(&callee).is_none() {
                    continue;
                }

                deps.push(format!("{} -> {}\n", current_clz, callee));
            }
        }

        let mut sub_graphs = vec![];
        for (key, items) in sub_graphs_map {
            sub_graphs.push(format!("\n  subgraph cluster_{}{{\n    {}\n    }}", key, items.join("\n    ")));
        }

        format!(
            "digraph G {{
  compound=true;
  ranksep=1
  node[shape=record]
{}\n{}\n}}",
            sub_graphs.join("\n"),
            deps.join("")
        )
    }
}
