extern crate rand;

pub mod dom;
pub mod html;

fn main() {
    let source = r#"
        <div>
          Hello world, this is a useful link:
          <a href="https://example.com" target="_blank">
              useful link
          </a>
        </div>
    "#.to_string();

    println!("{}", html::parse(source))
}
