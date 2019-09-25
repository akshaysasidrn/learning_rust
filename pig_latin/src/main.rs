// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is
// added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay”
// added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

use std::io;

fn main() {
    println!("Enter the sentence:");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("failed to read line");

    let pig_latin = convert_to_pig_latin(input);

    println!("{}", pig_latin);
}

fn convert_to_pig_latin(sentence: String) -> String {
    let mut res = String::new();

    for word in sentence.split_whitespace() {
        if let Some(c) = word.chars().next() {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                res.push_str(&format!("{}-hay ", word));
            } else {
                res.push_str(&format!("{}-{}ay ", &word[1..], &c));
            }
        } else {
            println!("Please provide a sentence");
        }
    }

    res
}
