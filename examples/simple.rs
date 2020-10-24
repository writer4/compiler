const SOURCE_CODE: &'static str = r###"
# Hello World
## Hello again

Peter **Tom**

Alpha Beta
Gamma

Red __Blue
Green__
"###;

fn main() {
    let result = compiler::compile(SOURCE_CODE).unwrap();
    println!("{}", result);
}
