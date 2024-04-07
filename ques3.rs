fn get_shortest_word(text: &str) -> Option<&str> {
    if text.is_empty() {
        return None;
    }

    let mut shortest_word: Option<&str> = None;
    let mut shortest_len = usize::MAX;

    for word in text.split_whitespace() {
        let word_len = word.len();
        if word_len < shortest_len {
            shortest_word = Some(word);
            shortest_len = word_len;
        }
    }

    shortest_word
}

fn main() {
    let text = "This is a string with words of different lengths";

    if let Some(shortest) = get_shortest_word(text) {
        println!("Shortest word: {}", shortest);
    } else {
        println!("Empty string provided");
    }
}
