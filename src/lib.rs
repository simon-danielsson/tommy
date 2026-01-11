use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Char(char),
    Integer(i32),
    Float(f64),
    Boolean(bool),
}

impl Value {
    pub(crate) fn as_string(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub(crate) fn as_i32(&self) -> Option<i32> {
        if let Value::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub(crate) fn as_f64(&self) -> Option<f64> {
        if let Value::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub(crate) fn as_bool(&self) -> Option<bool> {
        if let Value::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub(crate) fn as_char(&self) -> Option<char> {
        if let Value::Char(c) = self {
            Some(*c)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Table {
    name: String,
    fields: Vec<(String, Value)>, // field name, field contents
}

#[allow(unused)]
impl Table {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.fields
            .iter()
            .find_map(|(k, v)| if k == key { Some(v) } else { None })
    }

    pub fn get_as<T>(&self) -> T
where
        T: for<'a> From<&'a Table>,
    {
        T::from(self)
    }
}

pub struct ParseConfig {
    table_l: Vec<Table>,
    file_path: String,
}

impl ParseConfig {
    /// Takes a directory path of type String and parses the file immediately
    pub fn from_file(file_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut parser = Self {
            table_l: Vec::new(),
            file_path,
        };

        parser.derive_tables()?;

        Ok(parser)
    }
    /// Retrieve table from list of parsed tables
    pub fn table<T>(&self, name: &str) -> Option<T>
where
        T: FromTable,
    {
        self.table_l
            .iter()
            .find(|t| t.name == name)
            .and_then(T::from_table)
    }

    fn derive_tables(&mut self) -> std::io::Result<()> {
        let file = File::open(self.file_path.clone())?;
        let reader = BufReader::new(file);

        let mut table_l: Vec<Table> = Vec::new();
        let mut table_c: Option<Table> = None;

        for line_result in reader.lines() {
            let mut line = line_result?;
            line = line.trim().to_string();

            // skip comments and empty lines
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // table declaration
            if line.starts_with('[') && line.ends_with(']') {
                // push prev table
                if let Some(table) = table_c.take() {
                    table_l.push(table);
                }

                // start a new table
                let table_name = line.trim_matches(&['[', ']'][..]).to_string();
                table_c = Some(Table {
                    name: table_name,
                    fields: Vec::new(),
                });

                continue;
            }

            // assign key-value pairs
            if let Some(eq) = line.find('=') {
                let key = line[..eq].trim();
                let value = line[eq + 1..].trim();

                if let Some(table) = table_c.as_mut() {
                    let parsed_value = value
                        .parse::<bool>()
                        .ok()
                        .map(Value::Boolean)
                        .or_else(|| {
                            value.parse::<i32>()
                                .ok()
                                .map(Value::Integer)
                        })
                        .or_else(|| {
                            value.parse::<f64>().ok().map(Value::Float)
                        })
                        .or_else(|| {
                            if value.starts_with('\'')
                            && value.ends_with('\'') && value.len()
                            >= 3
                            {
                                Some(Value::Char(
                                    value[1..value.len() - 1]
                                        .chars()
                                        .next()
                                        .unwrap(),
                                ))
                            } else if value.len() == 1 {
                                Some(Value::Char(
                                    value.chars()
                                        .next()
                                        .unwrap(),
                                ))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| {
                            Value::String(value.to_string())
                        });

                    table.fields.push((key.to_string(), parsed_value));
                }
            }
        }

        // push last table
        if let Some(table) = table_c {
            table_l.push(table);
        }

        // add table list to self
        self.table_l = table_l;

        Ok(())
    }
}

#[macro_export]
macro_rules! from_table_struct {
    ($struct_name:ident { $($field:ident: $type:ty),* $(,)? }) => {
        impl FromTable for $struct_name {
        fn from_table(table: &Table) -> Option<Self> {
        Some($struct_name {
        $(
        $field: table.get(stringify!($field))
        .and_then(<$type>::from_value)?,
        )*
        })
        }
        }
    };
}

pub trait FromValue: Sized {
    fn from_value(v: &Value) -> Option<Self>;
}

impl FromValue for String {
    fn from_value(v: &Value) -> Option<Self> {
        v.as_string().map(|s| s.to_string())
    }
}
impl FromValue for i32 {
    fn from_value(v: &Value) -> Option<Self> {
        v.as_i32()
    }
}
impl FromValue for f64 {
    fn from_value(v: &Value) -> Option<Self> {
        v.as_f64()
    }
}
impl FromValue for bool {
    fn from_value(v: &Value) -> Option<Self> {
        v.as_bool()
    }
}
impl FromValue for char {
    fn from_value(v: &Value) -> Option<Self> {
        v.as_char()
    }
}

pub trait FromTable: Sized {
    fn from_table(table: &Table) -> Option<Self>;
}
