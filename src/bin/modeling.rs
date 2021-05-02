use std::{env, fs};
use modeling::by_dir;
use modeling::render::PlantUmlRender;

fn main() {
    let args: Vec<String> = env::args().collect();

    let classes = by_dir(".");
    let uml = PlantUmlRender::render(&classes);

    let _ = fs::write("modeling.puml", uml);
}
