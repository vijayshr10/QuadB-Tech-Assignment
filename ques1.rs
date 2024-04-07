use std::io;

fn is_palindrome(text: &str) -> bool {
    // Convert the string to lowercase for case-insensitive comparison
    let text_lower = text.to_lowercase();

    // Remove whitespace characters (replace with an empty string)
    let text_without_whitespace = text_lower.replace(|c: char| c.is_whitespace(), "");

    // Check if the reversed string is equal to the original string
    text_without_whitespace.chars().eq(text_without_whitespace.chars().rev())
}


fn main() {
    let mut input = String::new();
    println!("Enter a string: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Remove trailing newline character
    input.pop();

    if is_palindrome(&input) {
        println!("{} is a palindrome!", input);
    } else {
        println!("{} is not a palindrome.", input);
    }
}
