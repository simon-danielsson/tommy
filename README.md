<h1 align="center">
    Tommy
</h1>
  
<p align="center">
  <em>A light-weight toml parser for <br>
configuration files in rust projects.</em>
</p>
  
<p align="center">
    <img src="https://img.shields.io/crates/v/tommy?style=flat-square&color=blueviolet&link=https%3A%2F%2Fcrates.io%2Fcrates%2Ftommy" alt="Crates.io version" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <img src="https://img.shields.io/badge/Rust-stable-orange?style=flat-square" alt="Rust" />
  <img src="https://img.shields.io/github/last-commit/simon-danielsson/tommy/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#Info">Info</a> •
  <a href="#usage">Usage</a> •
  <a href="#license">License</a>
</p>  
  

---
<div id="usage"></div>

## 📦 Information
  
For my rust programs I was using various serde/toml crates for parsing (what ultimately only were) simple configuration files, and at some point I decided that it was unnecessary.  
  
Tommy is dumb, blunt and clunky. It's built for parsing simple configuration files containing tables of integers, strings, chars, floats and booleans - it can't do anything more and it doesn't need to do anything more.  
  
---
<div id="usage"></div>

## ✨ Usage
    
``` rust
use tommy::*;

macro_rules! config_table {
    ($nme:ident { $($fld:ident : $typ:ty),* $(,)? }) => {
        #[derive(Debug)]
        #[allow(unused)]
        struct $nme {
        $($fld: $typ),*
        }
        from_table_struct!($nme {
        $($fld: $typ),*
        });
    };
}

config_table!(Cursor {
    blink: bool,
    blink_duration: i32,
});

config_table!(Window {
    title: String,
    width: f64,
    height: f64,
});

config_table!(Icons {
    entry: char,
    exit: char,
    controls: char,
});

struct Config {
    cursor: Cursor,
    window: Window,
    icons: Icons,
}

impl Config {
    fn new(cursor: Cursor, window: Window, icons: Icons) -> Self {
        Self {
            cursor,
            window,
            icons,
        }
    }
}

fn main() {
    let parsed_user = ParseConfig::from_file("test.toml".to_string()).unwrap();
    let parsed_fabk = ParseConfig::from_file("fallback.toml".to_string()).unwrap();

    /// # or instead of using macro:
    /// let cursor_conf: Cursor = parsed_user
    ///     .table("cursor")
    ///     .or_else(|| parsed_fabk.table("cursor"))
    ///     .unwrap();
    macro_rules! load_conf {
        ($var:ident : $ty:ty) => {
            let $var: $ty = parsed_user
            .table(stringify!($ty).to_lowercase().as_str())
            .or_else(|| {
            println!(
            "WARNING: fallback was used for table: {}",
            stringify!($ty)
            );
            parsed_fabk.table(stringify!($ty).to_lowercase().as_str())
            })
            .unwrap();
        };
    }

    load_conf!(cursor_conf: Cursor);
    load_conf!(window_conf: Window);
    load_conf!(icons_conf: Icons);

    let config: Config = Config::new(cursor_conf, window_conf, icons_conf);

    println!("{:#?}", config.cursor);
    println!("{:#?}", config.window);
    println!("{:#?}", config.icons);
}
```
  
---
<div id="license"></div>

## 📜 License
This project is licensed under the [MIT License](https://github.com/simon-danielsson/tommy/blob/main/LICENSE).  
