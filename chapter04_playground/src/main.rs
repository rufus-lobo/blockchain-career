fn main() {
    println!("Enter a sentence for word processing:");

    let mut sentence: String = String::new();
    std::io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");
    let sentence = sentence.trim();

    let firstword = first_word(&sentence);
    println!("The first word is: {firstword}");
    let wordcount = word_count(&sentence);
    println!("The number of words are: {wordcount}");
    let longestword: &str = longest_word(&sentence);
    println!("The longest word is: {longestword}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn word_count(s: &str) -> u8 {
    let mut count = 0;
    let bytes = s.as_bytes();

    for (_, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count += 1;
        }
    }

    count + 1
}

fn longest_word(s: &str) -> &str {
    let mut longest_word: &str = "";
    let mut longest_word_length: usize = 0;
    let mut prev_word_start_index: usize = 0;

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if (i - prev_word_start_index) >= longest_word_length {
                longest_word_length = i - prev_word_start_index;
                longest_word = &s[prev_word_start_index..i];
            }
            prev_word_start_index = i + 1;
        }
    }

    longest_word
}
