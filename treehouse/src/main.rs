use std::io::stdin;

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
    let allowed_visitors_list = ["hmd", "uzi", "rgb"];
    let mut allow_visitor_in = false;
    for visitor in allowed_visitors_list {
        if visitor == name {
            allow_visitor_in = true;
        }
    }
    if allow_visitor_in {
        println!("Welcome to my treehouse {:?}", name);
    } else {
        println!("Get out you are not on the list {:?}", name);
    }
}
