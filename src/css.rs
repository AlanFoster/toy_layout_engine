use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Stylesheet {
    rules: Vec<Rule>,
}

#[derive(Debug, PartialEq)]
struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, PartialEq)]
struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

#[derive(Debug, PartialEq)]
struct Declaration {
    name: String,
    value: Value
}

#[derive(Debug, PartialEq)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, PartialEq)]
enum Unit {
    Px,
}

#[derive(Debug, PartialEq)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn parse_css(input: String) -> Stylesheet {
    let rules = Parser { pos: 0, input: input }.parse_rules();

    Stylesheet {
        rules
    }
}

struct Parser {
    pos: usize,
    input: String,
}

fn is_valid_identifier_char(c: char) -> bool {
    match c {
        'a'...'z' | 'A'...'Z' | '0'...'9' => true,
        _ => false,
    }
}

impl Parser {
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();

        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }
            rules.push(self.parse_rule());
        }

        rules
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        loop {
            selectors.push(self.parse_simple_selector());
            self.consume_whitespace();
            match self.next_char() {
                ',' => { self.consume_char(); self.consume_whitespace(); },
                '{' => break,
                e @ _ => panic!("Unexpected char {}", e),
            }
        }

        selectors
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();
        assert_eq!(self.consume_char(), '{');

        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }

            declarations.push(self.parse_declaration());
        }

        declarations
    }

    fn parse_simple_selector(&mut self) -> Selector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };

        loop {
            self.consume_whitespace();

            if self.eof() {
                break;
            }

            match self.next_char() {
                '#' => {
                    self.consume_char();
                    let tag_name = self.parse_identifier();
                    selector.id = Some(tag_name);
                },
                '.' => {
                    self.consume_char();
                    let class_name = self.parse_identifier();
                    selector.class.push(class_name);
                },
                c if is_valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }

        Selector::Simple(selector)
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.parse_declaration_name();
        self.consume_whitespace();
        assert_eq!(self.consume_char(), ':');
        self.consume_whitespace();
        let value = self.parse_value();
        assert_eq!(self.consume_char(), ';');

        Declaration {
            name: name,
            value: value,
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'...'9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => self.parse_keyword(),
        }
    }

    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        Value::ColorValue(
            Color {
                r: self.parse_hex_pair(),
                g: self.parse_hex_pair(),
                b: self.parse_hex_pair(),
                a: 255,
            }
        )
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let mut hex_pair = self.consume_char().to_string();
        hex_pair.push(self.consume_char());
        u8::from_str_radix(hex_pair.as_ref(), 0x10).unwrap()
    }

    fn parse_length(&mut self) -> Value {
        let float = self.parse_float();
        let unit = self.parse_unit();

        Value::Length(float, unit)
    }

    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| match c {
            '0' ... '9' | '.' => true,
            _ => false,
        });

        f32::from_str(s.as_ref()).unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
        match self.parse_declaration_name().to_ascii_lowercase().as_ref() {
            "px" => Unit::Px,
            e @ _ => panic!("unexpected {}", e)
        }
    }

    fn parse_keyword(&mut self) -> Value {
        let value = self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '-' => true,
            _ => false,
        });

        Value::Keyword(value)
    }

    fn parse_declaration_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9' | '-' => true,
            _ => false,
        })
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(is_valid_identifier_char)
    }

    fn next_char(&mut self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn eof(&mut self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut indices = self.input[self.pos..].char_indices();
        let (_, current) = indices.next().unwrap();
        let (next_pos, _) = indices.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        current
    }

    fn consume_while<F>(&mut self, f: F) -> String
        where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && f(self.next_char()) {
            result.push(self.consume_char())
        }

        result
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(char::is_whitespace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_css() {
        let input = r#"
            h1 { padding: 10px; }
        "#.to_string();

        assert_eq!(
            parse_css(input),
            Stylesheet {
                rules: vec![
                    Rule {
                        selectors: vec![
                            Selector::Simple(
                                SimpleSelector {
                                    tag_name: Some("h1".to_string()),
                                    id: None,
                                    class: Vec::new(),
                                }
                            ),
                        ],
                        declarations: vec![
                            Declaration {
                                name: "padding".to_string(),
                                value: Value::Length(10.0, Unit::Px),
                            }
                        ]
                    }
                ]
            }
        )
    }

    #[test]
    fn parse_simple_css_with_color() {
        let input = r#"
            h1 { color: #aabbcc; }
        "#.to_string();

        assert_eq!(
            parse_css(input),
            Stylesheet {
                rules: vec![
                    Rule {
                        selectors: vec![
                            Selector::Simple(
                                SimpleSelector {
                                    tag_name: Some("h1".to_string()),
                                    id: None,
                                    class: Vec::new(),
                                }
                            ),
                        ],
                        declarations: vec![
                            Declaration {
                                name: "color".to_string(),
                                value: Value::ColorValue(
                                    Color {
                                        r: 0xAA,
                                        g: 0xBB,
                                        b: 0xCC,
                                        a: 255,
                                    }
                                )
                            }
                        ]
                    }
                ]
            }
        )
    }

    #[test]
    fn parsing_complex_css() {
        let input = r#"
            h1, div.bar, #foo { padding: 10px; color: inherit; }
        "#.to_string();

        assert_eq!(
            parse_css(input),
            Stylesheet {
                rules: vec![
                    Rule {
                        selectors: vec![
                            Selector::Simple(
                                SimpleSelector {
                                    tag_name: Some("h1".to_string()),
                                    id: None,
                                    class: Vec::new(),
                                }
                            ),
                            Selector::Simple(
                                SimpleSelector {
                                    tag_name: Some("div".to_string()),
                                    id: None,
                                    class: vec!["bar".to_string()],
                                }
                            ),
                            Selector::Simple(
                                SimpleSelector {
                                    tag_name: None,
                                    id: Some("foo".to_string()),
                                    class: Vec::new(),
                                }
                            ),
                        ],
                        declarations: vec![
                            Declaration {
                                name: "padding".to_string(),
                                value: Value::Length(10.0, Unit::Px)
                            },
                            Declaration {
                                name: "color".to_string(),
                                value: Value::Keyword("inherit".to_string()),
                            }
                        ]
                    }
                ]
            }
        )
    }
}
