use crate::{ClassInfo, ParseOption};
pub use mermaid_render::MermaidRender;
pub use plantuml_render::PlantUmlRender;
use std::collections::HashMap;

pub mod graphviz_render;
pub mod mermaid_render;
pub mod plantuml_render;

pub fn process_name(parse_option: &&ParseOption, name: &str) -> String {
    if !parse_option.without_suffix.is_empty() {
        if let Some(text) = name.strip_suffix(&parse_option.without_suffix) {
            return text.to_string();
        }
    }

    name.to_string()
}

pub fn render_method(
    clazz: &ClassInfo,
    dep_map: &mut HashMap<String, String>,
    space: &str,
    parse_option: &ParseOption,
) -> Vec<String> {
    let mut methods = vec![];
    let clazz_name = process_name(&parse_option, &clazz.name);
    for method in &clazz.methods {
        let method_name = process_name(&parse_option, &method.name);
        if method.return_type.is_empty() {
            methods.push(format!("{}  {}{}()\n", space, method.access, method_name))
        } else {
            methods.push(format!(
                "{}  {} {} {}()\n",
                space, method.access, method.return_type, method_name
            ));

            if method.pure_return_type.len() > 0 {
                dep_map.insert(method.pure_return_type.clone(), clazz_name.clone());
            } else {
                dep_map.insert(method.return_type.clone(), clazz_name.clone());
            }
        }
    }
    methods
}

pub fn render_member(
    clazz: &ClassInfo,
    dep_map: &mut HashMap<String, String>,
    space: &str,
    parse_option: &ParseOption,
    class_map: &mut HashMap<String, bool>,
) -> Vec<String> {
    let clazz_name = process_name(&parse_option, &clazz.name);
    let mut members = vec![];
    for member in &clazz.members {
        let member_name = process_name(&parse_option, &member.name);
        if member.data_type.is_empty() {
            members.push(format!("{}  {}{}\n", space, member.access, member_name))
        } else {
            let id = "Id";
            let mut data_type: &str = &member.data_type;
            if parse_option.without_impl_suffix {
                // ex. `IRepository` will check is R uppercase
                if data_type.len() > id.len() && data_type.starts_with("I") {
                    let char = data_type.chars().nth(1).unwrap();
                    if char.to_uppercase().to_string() == char.to_string() {
                        data_type = &data_type[1..data_type.len()];
                    }
                }
            }

            if parse_option.inline_id_suffix {
                let ids = "Ids";
                // - int UserId to be `User UserId`
                if member_name.ends_with(id) && member_name.len() > id.len() {
                    let member_name = &member_name[0..(member_name.len() - id.len())];
                    if class_map.get(member_name).is_some() {
                        data_type = member_name;
                    }
                }

                if member_name.ends_with(ids) && member_name.len() > ids.len() {
                    let member_name = &member_name[0..(member_name.len() - ids.len())];
                    if class_map.get(member_name).is_some() {
                        data_type = member_name;
                    }
                }
            }

            members.push(format!(
                "{}  {} {} {}\n",
                space, member.access, data_type, member_name
            ));

            if member.pure_data_type.len() > 0 {
                dep_map.insert(member.pure_data_type.clone(), clazz_name.clone());
            } else {
                dep_map.insert(data_type.to_string(), clazz_name.clone());
            }
        }
    }
    members
}

#[cfg(test)]
mod tests {
    use crate::coco_struct::{ClassInfo, MemberInfo, MethodInfo};
    use crate::render::PlantUmlRender;
    use crate::ParseOption;

    #[test]
    fn should_render_empty() {
        let classes = vec![];
        let str = PlantUmlRender::render(&classes, &ParseOption::default());
        assert_eq!("@startuml\n\n\n\n@enduml", str);
    }

    #[test]
    fn should_render_single_empty_class() {
        let mut classes = vec![];
        let demo = ClassInfo::new("Demo");
        classes.push(demo);

        let str = PlantUmlRender::render(&classes, &ParseOption::default());
        assert_eq!("@startuml\n\nclass Demo {\n}\n\n@enduml", str);
    }

    #[test]
    fn should_render_member_method() {
        let mut classes = vec![];
        let mut demo = ClassInfo::new("Demo");

        let member = MemberInfo::new("demo", "-".to_string(), "String".to_string());
        demo.members.push(member);

        let method = MethodInfo::new("method", "-".to_string(), vec![], "Demo".to_string());
        demo.methods.push(method);

        classes.push(demo);

        let str = PlantUmlRender::render(&classes, &ParseOption::default());
        assert_eq!(
            "@startuml\n\nclass Demo {\n  - String demo\n  - Demo method()\n}\n\n@enduml",
            str
        );
    }

    #[test]
    fn should_render_deps() {
        let mut classes = vec![];
        let mut demo = ClassInfo::new("Demo");
        let demo2 = ClassInfo::new("Demo2");

        let member = MemberInfo::new("demo", "-".to_string(), "String".to_string());
        demo.members.push(member);

        let method = MethodInfo::new("method", "-".to_string(), vec![], "Demo2".to_string());
        demo.methods.push(method);

        classes.push(demo);
        classes.push(demo2);

        let str = PlantUmlRender::render(&classes, &ParseOption::default());
        assert_eq!(true, str.contains("Demo -- Demo2"));
        assert_eq!(false, str.contains("Demo -- String"));
    }

    #[test]
    fn should_render_parents() {
        let mut classes = vec![];
        let mut demo = ClassInfo::new("Demo");
        let demo2 = ClassInfo::new("Demo2");

        demo.parents.push(demo2.name.clone());

        classes.push(demo);
        classes.push(demo2);

        let str = PlantUmlRender::render(&classes, &ParseOption::default());
        println!("{}", str);
        assert!(str.contains("Demo2 <|-- Demo"));
    }

    #[test]
    fn should_render_array() {
        let mut classes = vec![];
        let mut demo = ClassInfo::new("Demo");
        let demo2 = ClassInfo::new("Demo2");

        let mut method = MethodInfo::new("method", "-".to_string(), vec![], "[]Demo2".to_string());
        method.pure_return_type = "Demo2".to_string();
        demo.methods.push(method);

        classes.push(demo);
        classes.push(demo2);

        let str = PlantUmlRender::render(&classes, &ParseOption::default());
        assert_eq!(true, str.contains("Demo -- Demo2"));
        assert_eq!(false, str.contains("Demo -- String"));
    }

    #[test]
    fn should_remove_suffix_text() {
        let mut classes = vec![];
        let mut demo = ClassInfo::new("DemoDto");
        let demo2 = ClassInfo::new("Demo2Dto");

        let mut method = MethodInfo::new("method", "-".to_string(), vec![], "[]Demo2".to_string());
        method.pure_return_type = "Demo2".to_string();
        demo.methods.push(method);

        classes.push(demo);
        classes.push(demo2);

        let mut parse_option = ParseOption::default();
        parse_option.without_suffix = "Dto".to_string();

        let str = PlantUmlRender::render(&classes, &parse_option);
        assert_eq!(true, str.contains("Demo -- Demo2"));
        assert_eq!(false, str.contains("Demo -- String"));
    }

    #[test]
    fn should_char() {
        let str = "IRepo";
        let char = str.chars().nth(1).unwrap();
        assert_eq!('R', char);
    }
}
