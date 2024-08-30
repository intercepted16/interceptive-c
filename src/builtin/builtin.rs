// src/builtin/builtin.rs

pub fn print(args: Vec<String>) {
    for arg in args {
        println!("{}", arg);
    }
}

pub(crate) fn functions() -> Vec<(&'static str, Box<dyn Fn(Vec<String>)>)> {
    vec![
        ("print", Box::new(print)),
    ]
}