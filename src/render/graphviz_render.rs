use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};

use crate::coco_struct::ClassInfo;
use crate::ParseOption;
use crate::render::{render_member, render_method};

/// Render classes info to string
pub struct GraphvizRender;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DData {
    nodes: Vec<DNode>,
    links: Vec<DLink>
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DNode {
    id: String,
    package: String,
    group: usize
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DLink {
    source: String,
    target: String,
    value: usize,
    pub package: String,
}

impl GraphvizRender {
    pub fn render(classes: &Vec<ClassInfo>, parse_option: &ParseOption) -> String {
        let mut sub_graphs_map: HashMap<String, Vec<String>> = HashMap::default();
        let mut deps: Vec<String> = vec![];
        let mut data = DData::default();

        let mut class_map: HashMap<String, bool> = HashMap::default();
        for clazz in classes {
            class_map.insert(clazz.name.clone(), true);
        }

        let mut class_catalog: HashMap<&str, usize> = HashMap::new();
        class_catalog.insert("Repository", 1);
        class_catalog.insert("Controller", 2);
        class_catalog.insert("Ctrl", 2);
        class_catalog.insert("Service", 3);
        class_catalog.insert("ServiceImpl", 3);

        let mut map_names: HashMap<usize, &str> = HashMap::new();
        map_names.insert(1, "Repository");
        map_names.insert(2, "Controller");
        map_names.insert(3, "Service");

        for clazz in classes {
            let mut dep_map: HashMap<String, String> = HashMap::default();

            let mut has_catalog = false;
            let class_name = &clazz.name;
            for (key, value) in &class_catalog {
                if class_name.ends_with(key) {
                    has_catalog = true;
                    let graph = sub_graphs_map.entry(map_names.get(value).unwrap().to_string()).or_insert(vec![]);
                    graph.push(class_name.to_string());

                    data.nodes.push(DNode {
                        id: class_name.to_string(),
                        package: clazz.package.to_string(),
                        group: *value
                    })
                }
            }

            if !has_catalog {
                data.nodes.push(DNode {
                    id: class_name.to_string(),
                    package: clazz.package.to_string(),
                    group: 4
                })
            }

            let _ = render_member(&clazz, &mut dep_map, "", parse_option, &mut class_map);
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

                data.links.push(DLink {
                    source: current_clz,
                    target: callee,
                    package: clazz.package.clone(),
                    value: 1
                })
            }
        }

        let mut sub_graphs = vec![];
        for (key, items) in sub_graphs_map {
            sub_graphs.push(format!("\n  subgraph cluster_{}{{\n    {}\n    }}", key, items.join("\n    ")));
        }

        let _ = fs::write("output.json", serde_json::to_string(&data).unwrap());

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


#[cfg(test)]
mod tests {
    use crate::{ClassInfo, ParseOption};
    use crate::render::graphviz_render::GraphvizRender;

    #[test]
    fn should_render_graphviz() {
        let info = ClassInfo::new("WorldServiceImpl");
        let clzs = vec![info];
        let string = GraphvizRender::render(&clzs, &ParseOption::default());
        assert_eq!("digraph G {\n  compound=true;\n  ranksep=1\n  node[shape=record]\n\n  subgraph cluster_Service{\n    WorldServiceImpl\n    }\n\n}", string);
    }
}