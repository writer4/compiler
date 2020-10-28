#[allow(dead_code)]
const SOURCE_CODE: &'static str = r###"
# Hello World
## Hello again

Peter **Tom**

Alpha Beta
Gamma

Red __Blue
Green__
"###;

#[cfg(feature = "html-backend")]
fn main() {
    let result = writer4_compiler::compile_html(SOURCE_CODE).unwrap();
    println!("{}", result);
}

#[cfg(not(feature = "html-backend"))]
fn main() {}
