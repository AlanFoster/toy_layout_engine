use dom;

pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser { pos: 0, input: source }.parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        dom::element("html".to_string(), dom::AttrMap::new(), nodes)
    }
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        cur_char
    }

    fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char())
        }

        result
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(char::is_whitespace)
    }

    fn consume_identifier(&mut self) -> String {
        self.consume_while(|c| match c {
            'a' ... 'z' | 'A' ... 'Z' | '0' ... '9' => true,
            _ => false,
        })
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_identifier()
    }

    fn parse_attr_name(&mut self) -> String {
        self.consume_identifier()
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        assert_eq!(self.consume_char(), '<');
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert_eq!(self.consume_char(), '>');

        let children = self.parse_nodes();

        assert_eq!(self.consume_char(), '<');
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.parse_tag_name(), tag_name);
        assert_eq!(self.consume_char(), '>');

        dom::element(tag_name, attrs, children)
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_attr_name();
        assert_eq!(self.consume_char(), '=');
        let value = self.parse_attr_value();

        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let start_quote = self.consume_char();
        assert!(start_quote == '"' || start_quote == '\'');
        let value = self.consume_while(|c| c != start_quote);
        assert_eq!(self.consume_char(), start_quote);

        value
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = dom::AttrMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break
            }

            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }

        attributes
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }

            nodes.push(self.parse_node());
        }

        nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_css() {
        let input = r#"
            <div>
              Hello world, this is a useful link:
              <a href="https://example.com" target="_blank">
                  useful link
              </a>
            </div>
        "#.to_string();

        let mut link_attributes = dom::AttrMap::new();
        link_attributes.insert("href".to_owned(), "https://example.com".to_owned());
        link_attributes.insert("target".to_owned(), "_blank".to_owned());

        assert_eq!(
            parse(input),
            dom::element("div".to_string(),
                         dom::AttrMap::new(),
                         vec![
                             dom::text("Hello world, this is a useful link:\n              ".to_owned()),
                             dom::element("a".to_string(),
                                          link_attributes,
                                          vec![
                                              dom::text("useful link\n              ".to_owned())
                                          ],
                             )
                         ],
            )
        )
    }
}
