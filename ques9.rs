fn reverse_string(text: &str) -> String {
    text.chars().rev().collect::<String>()
  }
  
  fn main() {
    let text = "Hello, world!";
    let reversed = reverse_string(text);
    println!("Original: {}", text);
    println!("Reversed: {}", reversed);
  }
  