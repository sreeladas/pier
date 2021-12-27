use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    #[serde(skip)]
    pub alias: String,
    pub query: String,
    pub sources: Option<Vec<String>>,
    pub description: Option<String>,
    pub references: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

impl Script {
    pub fn display_query(&self, display_full: bool, width: usize) -> &str {
        match display_full {
            true => &self.query,
            false => {
                match &self.query.lines().nth(0) {
                    Some(line) => {
                        match line.chars().count() {
                            c if c <= width => line,
                            c if c > width => &line[0..width],
                            _ => "",
                        }
                    }
                    None => &self.query,
                }
            }
        }
    }
}
