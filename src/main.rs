extern crate rand;

use std::collections::{ HashMap };

pub mod dom;

fn main() {
    let mut link_attributes = HashMap::new();
    link_attributes.insert("href".to_owned(), "https://example.com".to_owned());
    link_attributes.insert("target".to_owned(), "_blank".to_owned());

    println!("{}",
            dom::element("a".to_string(),
                    link_attributes,
                    vec![
                        dom::text("hello world".to_owned())
                    ],
            )
    )
}
