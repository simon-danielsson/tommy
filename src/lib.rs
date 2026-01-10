use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
}

struct Table {
    name: String,
    fields: Vec<(String, Value)>, // field name, field contents
}

pub struct ParseConfig {
    table_l: Vec<Table>,
    file_path: String,
}

impl ParseConfig {
    // "Combined" constructor + parse
    pub fn from_file(file_path: String) -> Self {
        let mut parser = Self {
            table_l: Vec::new(),
            file_path,
        };

        if let Err(e) = parser.derive_tables() {
            panic!("error whilst deriving tables: {}", e);
        }

        for table in &parser.table_l {
            println!("Table: {}", table.name);
            for (k, v) in &table.fields {
                println!("  {} = {:?}", k, v);
            }
        }

        parser
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

            if let Some(eq_idx) = line.find('=') {
                let key = line[..eq_idx].trim();
                let value = line[eq_idx + 1..].trim();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_the_test_file() {
        let _config = ParseConfig::from_file("src/test.toml".to_string());
    }
}
