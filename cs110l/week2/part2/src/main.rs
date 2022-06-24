// use std::io::{self, BufRead};
fn prompt_user_input(s: &str) -> String {
    println!("{}", s);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn read_shopping_list() -> Vec<String> {
    let mut shopping_list = Vec::new();
    loop {
        let input = prompt_user_input("remember to buy: ");
        if input.to_lowercase() == "done" {
            break;
        }
        else {
            shopping_list.push(input);
        }
    }
    shopping_list
}
fn print_shopping_list(shopping_list: &Vec<String>) {
    println!("you need to buy:");
    for item in shopping_list.iter() {
        println!("* {}", item);
    }
}
fn main() {
    print_shopping_list(&read_shopping_list());
}
