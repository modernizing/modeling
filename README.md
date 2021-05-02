# Modeling

> Modeling is a tools to analysis different languages by Ctags

process:

1. generate to opt
2. call `ctags` with opt
3. analysis ctags logs
4. output resulse

## Usage

```rust
let vec = analysis_by_dir(".");
let result = PlantUmlRender::render(&vec);

println!("{}", result);
```

License
---

ctags analysis based on [https://github.com/dalance/ptags](https://github.com/dalance/ptags) with MIT, see in [src](plugins/coco_struct_analysis/src)

ctags parser rewrite from Golang's [https://github.com/ruben2020/tags2uml](https://github.com/ruben2020/tags2uml) with Apache License.

@ 2020~2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
