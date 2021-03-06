/// Copyright https://github.com/ruben2020/tags2uml @ruben2020
//  from file: https://github.com/ruben2020/tags2uml/blob/master/tagsparser.go
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::coco_struct::{ClassInfo, MemberInfo, MethodInfo};
use crate::ParseOption;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::Lines;

#[derive(Default)]
pub struct CtagsParser {
    pub(crate) option: ParseOption,
    class_map: HashMap<String, ClassInfo>,
}

lazy_static! {
    static ref CLASS_RE: Regex = Regex::new(
        r"(?x)
^(?P<class_name>[A-Za-z0-9_]+)
\t(?P<file_name>([^\t]+))
\t([^\t]+)\t(class|struct)"
    )
    .unwrap();
    static ref INHERITS_RE: Regex = Regex::new(r"inherits:(?P<inherits>[A-Za-z0-9_:,]+)").unwrap();
    static ref AVAILABLE_RE: Regex = Regex::new(
        r#"(?x)
^(?P<name>[A-Za-z0-9_]+)
\t(?P<data_type>[^\t]+)
\t([^\t]+.*?")
\t(?P<tag_type>[A-Za-z]+)
\tline:(?P<line_no>[0-9]+)"#
    )
    .unwrap();
    static ref RE_CLASS: Regex = Regex::new(
        r"(?x)
(class|implementation|struct):(?P<class_name>[A-Za-z0-9_\.]+)"
    )
    .unwrap();
    static ref RE_ACCESS: Regex = Regex::new(r"access:(?P<access>[A-Za-z0-9_]+)").unwrap();
    static ref RE_LANGUAGE: Regex = Regex::new(r"language:(?P<language>[A-Za-z0-9_\#]+)").unwrap();
    static ref RE_TYPE: Regex =
        Regex::new(r"/\^([ ]*)(?P<datatype>[A-Za-z0-9_.]+)([^A-Za-z0-9_]+)(.*)\$/").unwrap();
    static ref RUST_TYPE: Regex = Regex::new(
        r"/\^([ ]*)(?P<field>[A-Za-z0-9_.\s]+)\s*:(\t|\s)*(?P<datatype>[A-Za-z0-9_.<>]+)"
    )
    .unwrap();
    static ref PURE_RUST_TYPE: Regex =
        Regex::new(r"((Vec|Option|<)*)(?P<datatype>[A-Za-z0-9_]+)>*").unwrap();
    static ref GO_TYPE: Regex = Regex::new(
        r"(?x)/\^([\s]*)
([A-Za-z0-9_.]+)
(,(\s|\t)*([A-Za-z0-9_.]+))*(\s|\t)* # for `name, access, returntype string`
(?P<datatype>[A-Za-z0-9_.<>\[\]]+)"
    )
    .unwrap();
    static ref TYPE_SCRIPT_TYPE: Regex =
        Regex::new(r"/\^([ |\t]*).*:([ |\t]*)(?P<datatype>[A-Za-z0-9_.<>\[\]]+).*\$/").unwrap();
    static ref PARAMETER: Regex = Regex::new(r"/.*\((?P<parameters>(.*?))\).*/").unwrap();
    static ref RUST_RETURN_TYPE: Regex =
        Regex::new(r"\s->\s(?P<datatype>[A-Za-z0-9_.]+)\s").unwrap();
    static ref TYPE_KEYWORDS: [&'static str; 18] = [
        "private",
        "public",
        "protected",
        "static",
        "volatile",
        "synchronized",
        "final",
        "const",
        "abstract",
        "struct",
        "union",
        "enum",
        "override",
        "internal",
        "extern",
        "readonly",
        "*",
        ":",
    ];
}

impl CtagsParser {
    pub fn parse_str(all_lines: Vec<Lines>) -> CtagsParser {
        let mut parser = CtagsParser::default();
        for lines in all_lines.clone() {
            for line in lines.into_iter() {
                parser.parse_class(line);
            }
        }

        for lines in all_lines {
            for line in lines.into_iter() {
                parser.parse_method_methods(line);
            }
        }

        parser
    }

    pub fn parse(dir: PathBuf) -> CtagsParser {
        let file = File::open(format!("{}", dir.display()).as_str()).expect("cannot find file");
        let reader = BufReader::new(file);

        let mut parser = CtagsParser::default();
        for line in reader.lines().flatten() {
            parser.parse_class(line.as_str());
        }

        let file = File::open(format!("{}", dir.display()).as_str()).expect("cannot find file");
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            parser.parse_method_methods(line.as_str());
        }

        parser
    }

    pub fn parse_class(&mut self, line: &str) {
        if let Some(captures) = CLASS_RE.captures(line) {
            let class_name = &captures["class_name"];
            let file_name = &captures["file_name"];
            let mut clazz = ClassInfo::new(class_name);
            clazz.file = file_name.to_string();

            if let Some(inherits_capts) = INHERITS_RE.captures(line) {
                let inherits_str = &inherits_capts["inherits"];
                let inherits = inherits_str
                    .split(',')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                clazz.parents = inherits;
            }

            self.class_map.insert(class_name.to_string(), clazz);
        }
    }

    pub fn parse_method_methods(&mut self, line: &str) {
        if !AVAILABLE_RE.is_match(line) {
            return;
        }

        let captures = AVAILABLE_RE.captures(line).unwrap();

        let clazz;
        match self.lookup_class_from_map(line) {
            None => return,
            Some(clz) => clazz = clz,
        }

        let name = &captures["name"];
        let tag_type = &captures["tag_type"];
        let line_no: i32 = (&captures["line_no"]).parse().unwrap_or(0);

        let mut access = "".to_string();
        if let Some(capts) = RE_ACCESS.captures(line) {
            let match_access = &capts["access"];
            match match_access {
                "public" => access = "+".to_string(),
                "private" => access = "-".to_string(),
                "protected" => access = "#".to_string(),
                &_ => {}
            }
        }

        let lang_capts = RE_LANGUAGE.captures(line).unwrap();
        let language = &lang_capts["language"];
        clazz.lang = language.to_string();

        let mut data_type = "".to_string();
        let mut pure_data_type = "".to_string();
        match language {
            "Java" | "C#" | "C++" => {
                let without_keywords = CtagsParser::remove_keywords(line.to_string());
                if let Some(capts) = RE_TYPE.captures(without_keywords.as_str()) {
                    data_type = (&capts["datatype"]).to_string();
                }
            }
            "Rust" => {
                if let Some(capts) = RUST_TYPE.captures(line) {
                    data_type = (&capts["datatype"]).to_string();

                    let field_with_access = (&capts["field"]).to_string();
                    access = Self::parse_rust_access(field_with_access);

                    if let Some(ty) = PURE_RUST_TYPE.captures(data_type.as_str()) {
                        pure_data_type = (&ty["datatype"]).to_string();
                    }
                } else if let Some(capts) = RUST_RETURN_TYPE.captures(line) {
                    data_type = (&capts["datatype"]).to_string();
                    if data_type == "Self" {
                        data_type = clazz.name.to_string()
                    }
                }
            }
            "Go" => {
                if let Some(capts) = GO_TYPE.captures(line) {
                    data_type = (&capts["datatype"]).to_string();
                }
            }
            "TypeScript" => {
                if let Some(capts) = TYPE_SCRIPT_TYPE.captures(line) {
                    data_type = (&capts["datatype"]).to_string();
                }
            }
            _ => {}
        }

        if tag_type.eq("member") || tag_type.eq("field") || tag_type.eq("property") {
            let mut member = MemberInfo::new(name, access, data_type);
            member.line_no = line_no;
            if !pure_data_type.is_empty() {
                member.pure_data_type = pure_data_type;
            }
            clazz.members.push(member);
        } else if tag_type.eq("method") || tag_type.eq("function") {
            let parameters = Self::pick_parameter_list(&captures[3]);
            let mut method = MethodInfo::new(name, access, parameters, data_type);
            method.line_no = line_no;
            if !pure_data_type.is_empty() {
                method.pure_return_type = pure_data_type;
            }
            clazz.methods.push(method);
        }
    }

    fn parse_rust_access(field_with_access: String) -> String {
        let split = field_with_access
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if split.len() > 1 {
            return if split[0] == "pub" {
                "+".to_string()
            } else {
                "#".to_string()
            };
        }

        "-".to_string()
    }

    pub fn remove_keywords(mut line: String) -> String {
        for keyword in TYPE_KEYWORDS.iter() {
            line = line.replacen(keyword, "", 1)
        }

        line
    }

    fn lookup_class_from_map(&mut self, line: &str) -> Option<&mut ClassInfo> {
        let mut class_name = "".to_string();

        if let Some(captures) = RE_CLASS.captures(line) {
            class_name = captures["class_name"].to_string();
        }

        let package = class_name.clone();
        let split = class_name.split('.');
        if let Some(last) = split.last() {
            class_name = last.to_string();
        }

        if class_name.is_empty() {
            return None;
        }

        let clazz: &mut ClassInfo;
        match self.class_map.get_mut(&*class_name) {
            Some(clz) => {
                clazz = clz;
            }
            None => {
                return None;
            }
        };

        clazz.package = package;
        Some(clazz)
    }

    pub fn classes(&self) -> Vec<ClassInfo> {
        let mut classes = vec![];
        for (_str, clz) in &self.class_map {
            let mut clazz = clz.clone();
            clazz
                .methods
                .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            clazz
                .members
                .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            if self.option.merge_method_name {
                clazz
                    .methods
                    .dedup_by(|a, b| a.name.eq_ignore_ascii_case(&*b.name));
                clazz
                    .members
                    .dedup_by(|a, b| a.name.eq_ignore_ascii_case(&*b.name));
            }

            classes.push(clazz);
        }

        classes.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        return classes;
    }

    fn pick_parameter_list(signature: &str) -> Vec<String> {
        PARAMETER
            .captures(signature)
            .and_then(|cap| {
                cap.name("parameters").map(|s| s.as_str()).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.split(',').map(|s| s.to_string()).collect())
                    }
                })
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use crate::{CtagsParser, ParseOption};
    use std::path::PathBuf;

    pub fn tags_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("_fixtures")
            .join("ctags")
    }

    #[test]
    pub fn should_replace_keyword() {
        assert_eq!("", CtagsParser::remove_keywords("public".to_string()));
    }

    #[test]
    pub fn should_get_return_type() {
        let str = "MethodIdentifier	SubscriberRegistry.java	/^    MethodIdentifier(Method method) {$/;\"	method	line:239	language:Java	class:SubscriberRegistry.MethodIdentifier	access:default
MethodIdentifier	SubscriberRegistry.java	/^  private static final class MethodIdentifier {$/;\"	class	line:234	language:Java	class:SubscriberRegistry	access:private";

        let lines = vec![str.lines()];
        let parser = CtagsParser::parse_str(lines);
        let classes = parser.classes();

        assert_eq!(1, classes.len());
        let first_method = classes[0].methods[0].clone();
        assert_eq!("MethodIdentifier", first_method.return_type);
    }

    #[test]
    fn should_get_parameters() {
        let str = "MethodIdentifier	SubscriberRegistry.java	/^    MethodIdentifier(Method method) {$/;\"	method	line:239	language:Java	class:SubscriberRegistry.MethodIdentifier	access:default
MethodIdentifier	SubscriberRegistry.java	/^  private static final class MethodIdentifier {$/;\"	class	line:234	language:Java	class:SubscriberRegistry	access:private";

        let parameters = CtagsParser::pick_parameter_list(str);
        assert_eq!(vec![String::from("Method method")], parameters);
    }

    #[test]
    pub fn should_build_rust_datatype() {
        let str =
            "MethodInfo	src/coco_struct.rs	/^pub struct MethodInfo {$/;\"	struct	line:21	language:Rust
name	src/coco_struct.rs	/^    pub name: String,$/;\"	field	line:22	language:Rust	struct:MethodInfo";

        let lines = vec![str.lines()];
        let parser = CtagsParser::parse_str(lines);
        let classes = parser.classes();

        assert_eq!(1, classes.len());
        assert_eq!("String", classes[0].members[0].data_type);
        assert_eq!("+", classes[0].members[0].access);
    }

    #[test]
    pub fn should_build_rust_method_datatype() {
        let str = r#"GraphvizRender	graphviz_render.rs	/^pub struct GraphvizRender;$/;"	struct	line:11	language:Rust
render	graphviz_render.rs	/^    pub fn render(classes: &Vec<ClassInfo>, parse_option: &ParseOption) -> String {$/;"	method	line:35	language:Rust	implementation:GraphvizRender"#;

        let lines = vec![str.lines()];
        let parser = CtagsParser::parse_str(lines);
        let classes = parser.classes();

        assert_eq!(1, classes.len());
        assert_eq!("render", classes[0].methods[0].name);
        // assert_eq!("+", classes[0].methods[0].access);
    }

    #[test]
    pub fn should_parse_java_file() {
        let dir = tags_dir().join("java_tags");
        let parser = CtagsParser::parse(dir);
        let classes = parser.classes();
        assert_eq!(1, classes.len());
        assert_eq!(9, classes[0].methods.len());

        assert_eq!("Java", classes[0].lang);
        assert_eq!("TypeName.java", classes[0].file);

        let first_method = classes[0].methods[0].clone();
        assert_eq!("description", first_method.name);
        assert_eq!("+", first_method.access)
    }

    #[test]
    pub fn should_parse_rust_file() {
        let dir = tags_dir().join("coco_tags");
        let parser = CtagsParser::parse(dir);
        let classes = parser.classes();

        assert_eq!(1, classes.len());
        assert_eq!("Rust", classes[0].lang);
        assert_eq!("coco_swagger/src/lib.rs", classes[0].file);
        let methods = classes[0].methods.clone();
        assert_eq!(5, methods.len());
        assert_eq!("default", methods[0].name);
        assert_eq!("execute", methods[1].name);
    }

    #[test]
    pub fn should_parse_rust_field_file() {
        let dir = tags_dir().join("coco_class_tags");
        let parser = CtagsParser::parse(dir);
        let classes = parser.classes();

        assert_eq!(3, classes.len());
        assert_eq!("ClassInfo", classes[0].name);
        assert_eq!(7, classes[0].members.len());
        assert_eq!("file", classes[0].members[0].name);

        let six_member = &classes[0].members[6];
        assert_eq!("parents", six_member.name);
        assert_eq!("String", six_member.pure_data_type);
        assert_eq!(43, six_member.line_no)
    }

    #[test]
    pub fn should_parse_golang_file() {
        let dir = tags_dir().join("go_tags");
        let parser = CtagsParser::parse(dir);
        let classes = parser.classes();

        assert_eq!(3, classes.len());
        assert_eq!("id", classes[0].members[0].name);
        // for /^	name, access, returntype string$/;"
        assert_eq!("access", classes[2].members[0].name);
        assert_eq!("string", classes[2].members[0].data_type);
        assert_eq!("name", classes[2].members[1].name);
        assert_eq!("string", classes[2].members[1].data_type);
        assert_eq!("string", classes[2].members[2].data_type);
    }

    #[test]
    pub fn should_parse_cpp_file() {
        let dir = tags_dir().join("cpp_tags");
        let parser = CtagsParser::parse(dir);
        let classes = parser.classes();
        assert_eq!(5, classes.len());

        let string_field = classes[2].clone();
        assert_eq!("C", string_field.lang);
        assert_eq!("IntFieldOrm", string_field.name);
        assert_eq!(1, string_field.parents.len());
        assert_eq!("IFieldOrm", string_field.parents[0]);
        assert_eq!(1, string_field.members.len());
        assert_eq!("m_value", string_field.members[0].name);

        assert_eq!(3, string_field.methods.len());
        assert_eq!("IntFieldOrm", string_field.methods[0].name);
        assert_eq!("migrate", string_field.methods[1].name);
        assert_eq!("save", string_field.methods[2].name);
    }

    #[test]
    pub fn should_parse_ts_file() {
        let dir = tags_dir().join("ts_tags");
        let parser = CtagsParser::parse(dir);
        let classes = parser.classes();

        assert_eq!(3, classes.len());
        assert_eq!("TypeScript", classes[0].lang);

        assert_eq!(1, classes[0].members.len());
        assert_eq!("name", classes[0].members[0].name);
        assert_eq!("string", classes[0].members[0].data_type);

        let methods = classes[0].methods.clone();
        assert_eq!(2, methods.len());
        assert_eq!("constructor", methods[0].name);
        assert_eq!("move", methods[1].name)
    }

    #[test]
    pub fn should_merge_duplicate() {
        let str = "\
MethodIdentifier	SubscriberRegistry.java	/^    MethodIdentifier(Method method) {$/;\"	method	line:239	language:Java	class:SubscriberRegistry.MethodIdentifier	access:default
MethodIdentifier	SubscriberRegistry.java	/^    MethodIdentifier(Method method) {$/;\"	method	line:238	language:Java	class:SubscriberRegistry.MethodIdentifier	access:default
MethodIdentifier	SubscriberRegistry.java	/^  private static final class MethodIdentifier {$/;\"	class	line:234	language:Java	class:SubscriberRegistry	access:private";

        let lines = vec![str.lines()];
        let parser = CtagsParser::parse_str(lines.clone());
        let classes = parser.classes();

        assert_eq!(2, classes[0].methods.len());

        let mut option = ParseOption::default();
        option.merge_method_name = true;

        let mut parser = CtagsParser::parse_str(lines);
        parser.option = option;
        let classes = parser.classes();

        assert_eq!(1, classes[0].methods.len());
    }
}
