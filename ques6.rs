fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
      return "".to_string();
    }
  
    let first_str = strs[0].chars();
    let mut prefix = String::new();
  
    for (i, char) in first_str.enumerate() {
      for str in strs.iter().skip(1) {
        if str.chars().nth(i).is_none() || str.chars().nth(i).unwrap() != char {
          return prefix.clone();
        }
      }
      prefix.push(char);
    }
  
    prefix
  }
  
  fn main() {
    let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let prefix = longest_common_prefix(&strs);
    println!("Longest common prefix: {}", prefix);
  }
  