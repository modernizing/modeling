# Modeling

[![crates.io](https://meritbadge.herokuapp.com/modeling)](https://crates.io/crates/modeling)
[![docs.rs](https://docs.rs/modeling/badge.svg)](https://docs.rs/modeling/)
[![license](https://img.shields.io/crates/l/modeling)](https://github.com/inherd/modeling/blob/master/LICENSE)

> Modeling is a tools to analysis different languages by Ctags

process:

1. generate to opt
2. call `ctags` with opt
3. analysis ctags logs
4. output result

language support:

 - [x] Java
 - [x] C#
 - [x] Cpp
 - [x] TypeScript
 - [x] Golang
 - [x] Rust (basic)
 - ... others by ctags

## Usage

```bash
Modeling 0.5.0

USAGE:
    modeling [FLAGS] [OPTIONS]

FLAGS:
    -b, --by-modules            multiple modules
    -f, --field-only            only load field in methods
    -h, --help                  Prints help information
    -m, --merge                 merge for same method name
    -r, --remove-impl-suffix    if class's field start with `IRepository` will become `Repository`
    -V, --version               Prints version information
    -w, --without-parent        without class inheritance

OPTIONS:
    -g, --grep <grep>                  by grep regex rules: for example: [default: ]
    -i, --input <input>                input dir [default: .]
    -o, --output-type <output-type>    support: puml, mermaid, graphviz with json [default: puml]
    -p, --packages <packages>...       filter by packages, like: `com.phodal.modeling`
    -s, --suffixes <suffixes>...       filter by suffixes, like: `java` for .java file
```

### sample: puml to Image

convert to image: `plantuml modeling.puml modeling.svg -tsvg`

### sample: Grep with MVC

```
modeling --input=/youpath/ --field-only --without-parent --grep ".*Service|.*Controller|.*Repository"
```

### sample: with Graphviz and Visualization

with `--output-type=graphviz`

```bash
modeling --input=/youpath  --field-only -o graphviz --without-impl-suffix
```

## Library

```
cargo install modeling
modeling .
```

#### Library

```rust
use modeling::{by_dir};
use modeling::render::PlantUmlRender;

let classes = by_dir("src/");
let puml = PlantUmlRender::render(&classes);
```

output sample:

```puml
@startuml

class Animal {
  + string name
  + string constructor()
  +move()
}

class Horse extends Animal {
  +move()
}

class Snake extends Animal {
  +move()
}

@enduml
```


License
---

ctags analysis based on [https://github.com/dalance/ptags](https://github.com/dalance/ptags) with MIT, see in [src](plugins/coco_struct_analysis/src)

ctags parser rewrite from Golang's [https://github.com/ruben2020/tags2uml](https://github.com/ruben2020/tags2uml) with Apache License.

@ 2020~2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
