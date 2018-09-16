use std::collections::HashMap;
use std::fmt;

pub type AttrMap = HashMap<String, String>;

pub struct Node {
    children: Vec<Node>,

    node_type: NodeType,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.node_type {
            NodeType::Text(ref text) => write!(f, "{}", text),
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

                let children =
                    self.children
                        .iter()
                        .map(|ref child| child.to_string())
                        .collect::<Vec<String>>()
                        .join("\n");

                return write!(
                    f,
                    "<{} {}>{}</{}>",
                    element_data.tag_name,
                    attributes,
                    children,
                    element_data.tag_name,
                );
            }
        }
    }
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}

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
