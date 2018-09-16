extern crate rand;

pub mod dom;

fn main() {
    let mut link_attributes = dom::AttrMap::new();
    link_attributes.insert("href".to_owned(), "https://example.com".to_owned());
    link_attributes.insert("target".to_owned(), "_blank".to_owned());

    println!("{}",
             dom::element("div".to_string(),
                          dom::AttrMap::new(),
                          vec![
                              dom::text("Hello world, this is a useful link:".to_owned()),
                              dom::element("a".to_string(),
                                           link_attributes,
                                           vec![
                                               dom::text("useful link".to_owned())
                                           ],
                              ),
                          ],
             )
    )
}
