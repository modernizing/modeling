use std::{env, fs};
use modeling::by_dir;
use modeling::render::{PlantUmlRender, MermaidRender};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = ".".to_string();
    if args.len() > 1 {
        path = args[1].clone();
    }
    let mut design_type = "puml".to_string();
    if args.len() > 2 {
        design_type = args[2].clone();
    }

    println!("Input path: {:?}", path.clone());
    let classes = by_dir(path.as_str());

    match design_type.as_str() {
        "mermaid" => {
            let uml = MermaidRender::render(&classes);
            let _ = fs::write("modeling.mermaid", uml);
        }
        &_ => {
            let uml = PlantUmlRender::render(&classes);
            let _ = fs::write("modeling.puml", uml);
        }
    }

}
