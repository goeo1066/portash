use std::{
    fmt::{self},
    path::Path,
};

use crate::{
    configs::{
        PortashConfig, PortashConfigCommand, PortashConfigGeneral,
        file_config::LineParseResult::{Blank, Comment, Property, Section, Text},
    },
    parsers::parse_prompt,
};

pub fn load_from_file(file_path: &Path) -> Result<PortashConfig, String> {
    if let Ok(data) = std::fs::read_to_string(file_path) {
        let mut config_general: Option<PortashConfigGeneral> = None;
        let mut config_command_list: Vec<PortashConfigCommand> = vec![];

        let result = SectionPropertyResult::parse_from_lines(&data);
        for spr in &result {
            let parsed = spr;
            let section: &str = &parsed.section;

            match section {
                "general" => {
                    let config: PortashConfigGeneral = parsed.into();
                    config_general = Some(config);
                }
                "command" => {
                    let config: PortashConfigCommand = parsed.into();
                    config_command_list.push(config);
                }
                _ => {
                    eprint!("Unknown Header - {}", spr);
                }
            }
        }

        if let Some(config_general) = config_general {
            Ok(PortashConfig {
                config_general: config_general,
                config_command: config_command_list,
            })
        } else {
            Err("".into())
        }
    } else {
        let path: String = file_path.to_str().map_or_default(|p| p.into());
        Err(format!("Failed to read file from {}", path))
    }
}

fn convert_to_config_general(property: &SectionPropertyResult) -> PortashConfigGeneral {
    let mut result = PortashConfigGeneral::default();

    for property in &property.properties {
        let key: &str = &property.key;
        match key {
            "prompt_header" => {
                result.prompt_header = property.value.clone();
            }
            "chained_header" => {
                result.chained_header = property.value.clone();
            }
            "msg_on_not_allowed_command" => {
                result.msg_on_not_allowed_command = property.value.clone();
            }
            "msg_on_login" => {
                result.msg_on_login = property.value.clone();
            }
            _ => {}
        }
    }

    result
}
fn convert_to_config_command(property: &SectionPropertyResult) -> PortashConfigCommand {
    let mut result = PortashConfigCommand {
        command: String::default(),
        execute: vec![],
    };

    for property in &property.properties {
        let key: &str = &property.key;
        match key {
            "command" => {
                result.command = property.value.clone();
            }
            "execute" => {
                result.execute = parse_prompt(&property.value).clone();
            }
            _ => {}
        }
    }

    result
}

impl fmt::Display for SectionPropertyResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let values: Vec<String> = self
            .properties
            .iter()
            .map(|it| format!("({} -> {})", it.key, it.value))
            .collect();

        f.write_fmt(format_args!(
            "[Section] {} - [Properties] {}",
            self.section,
            values.join(", ")
        ))
    }
}

impl Into<PortashConfigGeneral> for &SectionPropertyResult {
    fn into(self) -> PortashConfigGeneral {
        convert_to_config_general(self)
    }
}

impl Into<PortashConfigGeneral> for SectionPropertyResult {
    fn into(self) -> PortashConfigGeneral {
        convert_to_config_general(&self)
    }
}

// ---

impl Into<PortashConfigCommand> for &SectionPropertyResult {
    fn into(self) -> PortashConfigCommand {
        convert_to_config_command(self)
    }
}

impl Into<PortashConfigCommand> for SectionPropertyResult {
    fn into(self) -> PortashConfigCommand {
        convert_to_config_command(&self)
    }
}

#[derive(Clone)]
pub struct SectionPropertyResult {
    section: String,
    properties: Vec<KeyValue>,
}

impl SectionPropertyResult {
    fn parse_from_lines(lines: &String) -> Vec<Self> {
        let mut result: Vec<Self> = vec![];
        let mut buffer: Option<Self> = None;

        for e in lines.split("\n") {
            let line = e;
            let line_parse_result = LineParseResult::parse_from_line(&line);
            match line_parse_result {
                Section(name) => {
                    if let Some(spr) = buffer {
                        result.push(spr);
                    }
                    buffer = Some(Self::new_with_section(&name));
                    continue;
                }
                Property(key, value) => {
                    if let Some(spr) = &mut buffer {
                        spr.add_property(&key, &value);
                    }
                    continue;
                }
                Text(line) => {
                    println!("Unsupported now, Text('{}')", line);
                }
                Blank => {
                    continue;
                }
                Comment => {}
            }
        }

        if let Some(spr) = buffer {
            result.push(spr);
        }
        return result;
    }

    fn new_with_section(section: &String) -> Self {
        Self {
            section: section.clone(),
            properties: vec![],
        }
    }

    pub fn add_property(&mut self, key: &String, value: &String) {
        let key_value = KeyValue {
            key: key.clone(),
            value: value.clone(),
        };
        self.properties.push(key_value);
    }
}

#[derive(Clone)]
pub struct KeyValue {
    key: String,
    value: String,
}

pub enum LineParseResult {
    Comment,
    Blank,
    Text(String),
    Section(String),
    Property(String, String),
}

impl LineParseResult {
    fn parse_from_line(line: &str) -> Self {
        let str = line;

        if str.starts_with("[") && str.ends_with("]") {
            // maybe a section
            return Section(str[1..str.len() - 1].into());
        }

        if str.starts_with("#") {
            return Comment;
        }

        if str.contains("=") {
            // maybe a property
            if let Some(index_of_eq) = first_index_of_char(str, '=') {
                let key = &str[0usize..index_of_eq];
                let value = &str[index_of_eq + 1..];
                return Property(key.into(), value.into());
            }
        }

        if str.trim().is_empty() {
            return Blank;
        } else {
            return Text(str.into());
        }
    }
}

fn first_index_of_char(str: &str, c: char) -> Option<usize> {
    for (i, e) in str.chars().into_iter().enumerate() {
        if e == c {
            return Some(i);
        }
    }
    return None;
}
