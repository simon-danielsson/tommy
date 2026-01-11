use tommy::*;

#[test]
fn read_the_test_file() {
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

    #[derive(Debug)]
    #[allow(unused)]
    struct Icons {
        entry: char,
        exit: char,
        controls: char,
    }
    from_table_struct!(Icons {
        entry: char,
        exit: char,
        controls: char,
    });

    let parsed_user = ParseConfig::from_file("test.toml".to_string()).unwrap();
    let parsed_fabk = ParseConfig::from_file("fallback.toml".to_string()).unwrap();

    let cursor_conf: Cursor = parsed_user
        .table("cursor")
        .or_else(|| parsed_fabk.table("cursor"))
        .unwrap();

    // let window_conf: Window = parsed.table("window").unwrap();
    // let icons_conf: Icons = parsed.table("icons").unwrap();

    // assert_eq!(cursor_conf.blink, true);
    println!("{:#?}", cursor_conf);
    // println!("{:#?}", window_conf);
    // println!("{:#?}", icons_conf);
}
