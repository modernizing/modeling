pub use mermaid_render::MermaidRender;
pub use plantuml_render::PlantUmlRender;
use crate::ClassInfo;
use std::collections::HashMap;

pub mod plantuml_render;
pub mod mermaid_render;


pub fn render_method(clazz: &&ClassInfo, dep_map: &mut HashMap<String, String>, space: &str) -> Vec<String> {
    let mut methods = vec![];
    for method in &clazz.methods {
        if method.return_type.is_empty() {
            methods.push(format!("{}  {}{}()\n", space, method.access, method.name))
        } else {
            methods.push(format!(
                "{}  {} {} {}()\n",
                space, method.access, method.return_type, method.name
            ));

            if method.pure_return_type.len() > 0 {
                dep_map.insert(method.pure_return_type.clone(), clazz.name.clone());
            } else {
                dep_map.insert(method.return_type.clone(), clazz.name.clone());
            }
        }
    }
    methods
}

pub fn render_member(clazz: &&ClassInfo, dep_map: &mut HashMap<String, String>, space: &str) -> Vec<String> {
    let mut members = vec![];
    for member in &clazz.members {
        if member.data_type.is_empty() {
            members.push(format!("{}  {}{}\n", space, member.access, member.name))
        } else {
            members.push(format!(
                "{}  {} {} {}\n",
                space, member.access, member.data_type, member.name
            ));

            if member.pure_data_type.len() > 0 {
                dep_map.insert(member.pure_data_type.clone(), clazz.name.clone());
            } else {
                dep_map.insert(member.data_type.clone(), clazz.name.clone());
            }
        }
    }
    members
}


#[cfg(test)]
mod tests {
    use crate::coco_struct::{ClassInfo, MemberInfo, MethodInfo};
    use crate::ParseOption;
    use crate::render::PlantUmlRender;

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

        let member = MemberInfo::new("demo", "-", "String".to_string());
        demo.members.push(member);

        let method = MethodInfo::new("method", "-", vec![], "Demo".to_string());
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

        let member = MemberInfo::new("demo", "-", "String".to_string());
        demo.members.push(member);

        let method = MethodInfo::new("method", "-", vec![], "Demo2".to_string());
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

        let mut method = MethodInfo::new("method", "-", vec![], "[]Demo2".to_string());
        method.pure_return_type = "Demo2".to_string();
        demo.methods.push(method);

        classes.push(demo);
        classes.push(demo2);

        let str = PlantUmlRender::render(&classes, &ParseOption::default());
        assert_eq!(true, str.contains("Demo -- Demo2"));
        assert_eq!(false, str.contains("Demo -- String"));
    }
}
