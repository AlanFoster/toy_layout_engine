use std::collections::HashMap;
use std::fmt;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct Node {
    children: Vec<Node>,

    node_type: NodeType,
}

impl Node {
    fn _indent(&self, formatter: &mut fmt::Formatter, indentation: u32) -> fmt::Result {
        let c = formatter.fill();
        for _ in 0..indentation {
            write!(formatter, "{}", c);
        }
        Ok(())
    }

    fn display(&self, formatter: &mut fmt::Formatter, indentation: u32) -> fmt::Result {
        match self.node_type {
            NodeType::Text(ref text) => {
                self._indent(formatter, indentation)?;
                write!(formatter, "{}", text.trim())
            },
            NodeType::Element(ref element_data) => {
                let attributes = match element_data.attributes.is_empty() {
                    true => "".to_string(),
                    false =>
                        element_data.attributes
                            .iter()
                            .map(|(key, value)| format!("{}=\"{}\"", key, value))
                            .collect::<Vec<String>>()
                            .join(" "),
                };

                self._indent(formatter, indentation)?;
                write!(formatter, "<{}", element_data.tag_name)?;
                if !attributes.is_empty() {
                    write!(formatter, " {}", attributes)?;
                }
                write!(formatter, ">\n")?;
                for child in self.children.iter() {
                    self._indent(formatter, indentation)?;
                    child.display(formatter, indentation + 2)?;
                    write!(formatter, "\n")?;
                }

                self._indent(formatter, indentation)?;
                write!(formatter, "</{}>", element_data.tag_name)
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.display(f, 0);
    }
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug, PartialEq)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn element(name: String, attributes: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes,
        }),
    }
}
