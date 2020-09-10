fn main() {
    let mut name=String::new();
    println!("Enter your name:");
    std::io::stdin().read_line(&mut name).ok().expect("couldn't read input");
    println!("Your name is : {}",name);
}
