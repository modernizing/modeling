use std::{env, fs};
use modeling::by_dir;
use modeling::render::PlantUmlRender;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = ".".to_string();
    if args.len() > 1 {
        path = args[1].clone();
    }

    println!("Input path: {:?}", path.clone());
    let classes = by_dir(path.as_str());
    let uml = PlantUmlRender::render(&classes);

    let _ = fs::write("modeling.puml", uml);
}
