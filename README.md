<h1 align="center">
    Tommy
</h1>
  
<p align="center">
  <em>A light-weight toml parser for easy parsing <br>
of configuration files in Rust projects</em>
</p>
  
<p align="center">
    <img src="https://img.shields.io/crates/v/tommy?style=flat-square&color=blueviolet&link=https%3A%2F%2Fcrates.io%2Fcrates%2Ftommy
" alt="Crates.io version" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-stable-orange?style=flat-square" alt="Rust" />
  <img src="https://img.shields.io/github/last-commit/simon-danielsson/tommy/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#examples">Features</a> •
  <a href="#license">License</a>
</p>  
  

---
<div id="examples"></div>

## ✨ Examples
    
``` rust
#[derive(Debug)]
#[allow(unused)]
struct SomeTable {
string: String,
number: i32,
float: f64,
boolean: bool,
}

from_table_struct!(SomeTable {
string: String,
number: i32,
float: f64,
boolean: bool,
});

let parsed = ParseConfig::from_file("path/to/file.toml".to_string());
let first_table: SomeTable = parsed.table("first_table").unwrap();
```
  
---
<div id="license"></div>

## 📜 License
This project is licensed under the [MIT License](https://github.com/simon-danielsson/tommy/blob/main/LICENSE).  
