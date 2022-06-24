fn print_info(status : &String, tried : &[char], failed_count : &u32) {
    println!("\n{}", status);
    println!("Tried: {}", tried.iter().collect::<String>());
    println!("Failed: {}", failed_count);
}
fn read_input() -> char{
    println!("Please enter a character: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().chars().next().unwrap()
}
fn check(input : char, status : &mut String, tried : &mut Vec<char>, failed_count : &mut u32, answer: &str){
    if tried.contains(&input){
        println!("You already tried this character");
    }
    else if answer.contains(&input.to_string()){
        tried.push(input);
        status.clear();
        for i in answer.chars(){
            if tried.contains(&i){
                status.push(i);
            }
            else{
                status.push('_');
            }
        }
        println!("Correct!");
    }
    else {
        tried.push(input);
        *failed_count += 1;
        println!("Wrong!");
    }
}
fn main() {
    let answer = "hangman".to_string();
    let mut status = "_______".to_string();
    let mut tried: Vec<char> = Vec::new();
    let mut failed_count: u32 = 0;
    loop {
        print_info(&status, &tried, &failed_count);
        check(read_input(), &mut status, &mut tried, &mut failed_count, &answer);
        if failed_count == 6 {
            println!("You lose!");
            println!("The answer is '{}'", answer);
            break;
        }
        if status==answer {
            println!("You win!");
            break;
        }
    }
}
