use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greatings: String,
}
impl Visitor {
    fn new(name: &str, greetings: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greatings: greetings.to_string(),
        }
    }
    fn great_visitor(&self) {
        println!("{}", self.greatings);
    }
}

fn whats_your_name() -> String {
    let mut your_name = String::new();
    println!("Hello. What is your name?");
    stdin()
        .read_line(&mut your_name)
        .expect("unable to read name");
    your_name.trim().to_lowercase()
}

fn main() {
    let name = whats_your_name();
    let visitors_list = [
        Visitor::new("hmd", "Hello Leader"),
        Visitor::new("uzi", "HEllo Uzi boi"),
        Visitor::new("rgb", "Hello Rayyan G Boy"),
    ];
    let known_visitor = visitors_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
        Some(visitor) => visitor.great_visitor(),
        None => println!("Baar nikal"),
    }
}
