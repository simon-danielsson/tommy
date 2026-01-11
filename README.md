<h1 align="center">
    Tommy
</h1>
  
<p align="center">
  <em>A light-weight toml parser for <br>
configuration files in Rust projects</em>
</p>
  
<p align="center">
    <img src="https://img.shields.io/crates/v/tommy?style=flat-square&color=blueviolet&link=https%3A%2F%2Fcrates.io%2Fcrates%2Ftommy" alt="Crates.io version" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-stable-orange?style=flat-square" alt="Rust" />
  <img src="https://img.shields.io/github/last-commit/simon-danielsson/tommy/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#usage">Usage</a> •
  <a href="#license">License</a>
</p>  
  

---
<div id="usage"></div>

## ✨ Usage
    
``` rust
use tommy::*;

#[derive(Debug)]
#[allow(unused)]
struct Cursor {
    blink: bool,
    blink_duration: i32,
}
from_table_struct!(Cursor {
    blink: bool,
    blink_duration: i32,
});

#[derive(Debug)]
#[allow(unused)]
struct Window {
    title: String,
    width: f64,
    height: f64,
}
from_table_struct!(Window {
    title: String,
    width: f64,
    height: f64,
});

fn main() {
    let parsed_user = ParseConfig::from_file("test.toml".to_string()).unwrap();
    let parsed_fabk = ParseConfig::from_file("fallback.toml".to_string()).unwrap();

    let cursor_conf: Cursor = parsed_user
        .table("cursor")
        .or_else(|| parsed_fabk.table("cursor"))
        .unwrap();
    let window_conf: Window = parsed_user
        .table("cursor")
        .or_else(|| parsed_fabk.table("window"))
        .unwrap();

    println!("{:#?}", cursor_conf);
    println!("{:#?}", window_conf);
}

```
  
---
<div id="license"></div>

## 📜 License
This project is licensed under the [MIT License](https://github.com/simon-danielsson/tommy/blob/main/LICENSE).  
