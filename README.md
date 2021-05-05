# Modeling

[![crates.io](https://meritbadge.herokuapp.com/modeling)](https://crates.io/crates/druid)
[![docs.rs](https://docs.rs/modeling/badge.svg)](https://docs.rs/modeling/)
[![license](https://img.shields.io/crates/l/modeling)](https://github.com/inherd/modeling/blob/master/LICENSE)

> Modeling is a tools to analysis different languages by Ctags

process:

1. generate to opt
2. call `ctags` with opt
3. analysis ctags logs
4. output resulse

language support:

 - [x] Java
 - [x] Cpp
 - [x] TypeScript
 - [x] Golang
 - [x] Rust (basic)

## Usage

### use binary

```
cargo install modeling
modeling .
```

### Library

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

Process finished with exit code 0
```

License
---

ctags analysis based on [https://github.com/dalance/ptags](https://github.com/dalance/ptags) with MIT, see in [src](plugins/coco_struct_analysis/src)

ctags parser rewrite from Golang's [https://github.com/ruben2020/tags2uml](https://github.com/ruben2020/tags2uml) with Apache License.

@ 2020~2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
