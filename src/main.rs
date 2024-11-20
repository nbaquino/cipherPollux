use std::collections::HashMap;
use rand::Rng;

// Morse code mappings
fn morse_map() -> HashMap<char, &'static str> {
    let mut m = HashMap::new();
    m.insert('A', ".-"); m.insert('B', "-..."); m.insert('C', "-.-."); m.insert('D', "-..");
    m.insert('E', "."); m.insert('F', "..-."); m.insert('G', "--."); m.insert('H', "....");
    m.insert('I', ".."); m.insert('J', ".---"); m.insert('K', "-.-"); m.insert('L', ".-..");
    m.insert('M', "--"); m.insert('N', "-."); m.insert('O', "---"); m.insert('P', ".--.");
    m.insert('Q', "--.-"); m.insert('R', ".-."); m.insert('S', "..."); m.insert('T', "-");
    m.insert('U', "..-"); m.insert('V', "...-"); m.insert('W', ".--"); m.insert('X', "-..-");
    m.insert('Y', "-.--"); m.insert('Z', "--.."); m.insert('1', ".----"); m.insert('2', "..---");
    m.insert('3', "...--"); m.insert('4', "....-"); m.insert('5', "....."); m.insert('6', "-....");
    m.insert('7', "--..."); m.insert('8', "---.."); m.insert('9', "----."); m.insert('0', "-----");
    m.insert(',', "--..--"); m.insert('.', ".-.-.-"); m.insert('?', "..--.."); m.insert('/', "-..-.");
    m.insert('-', "-....-"); m.insert('(', "-.--."); m.insert(')', "-.--.-");
    m
}

// Reverse Morse map
fn morse_to_char_map() -> HashMap<&'static str, char> {
    morse_map().into_iter().map(|(k, v)| (v, k)).collect()
}

// Pollux cipher mappings
fn morse_to_num_map() -> HashMap<char, Vec<char>> {
    let mut m = HashMap::new();
    m.insert('.', vec!['0', '3', '7', '8', 'A', 'E', 'F', 'M', 'O', 'P', 'Q', 'X', 'Y', 'Z']);
    m.insert('-', vec!['1', '4', '5', 'B', 'C', 'G', 'J', 'N', 'R', 'T', 'W']);
    m.insert(' ', vec!['2', '6', '9', 'D', 'H', 'I', 'K', 'L', 'S', 'U', 'V']);
    m
}

// Reverse Pollux cipher mapping
fn num_to_morse_map() -> HashMap<char, char> {
    let mut m = HashMap::new();
    for (key, values) in morse_to_num_map() {
        for value in values {
            m.insert(value, key);
        }
    }
    m
}

// Convert text to Morse code
fn text_to_morse(text: &str) -> String {
    let map = morse_map();
    text.to_uppercase().chars().filter_map(|c| map.get(&c).map(|&morse| format!("{} ", morse))).collect::<String>().trim_end().to_string()
}

// Convert Morse code to text
fn morse_to_text(morse: &str) -> String {
    let map = morse_to_char_map();
    morse.split_whitespace().filter_map(|m| map.get(m).copied()).collect()
}

// Convert Morse code to numeric code
fn morse_to_num(morse: &str) -> String {
    let map = morse_to_num_map();
    let mut rng = rand::thread_rng();
    morse.chars().filter_map(|c| map.get(&c).and_then(|v| Some(v[rng.gen_range(0..v.len())]))).collect()
}

// Convert numeric code to Morse code
fn num_to_morse(nums: &str) -> String {
    let map = num_to_morse_map();
    nums.chars().filter_map(|n| map.get(&n).copied()).collect()
}

// Encode text to Pollux numeric code
fn encode_pollux(text: &str) -> String {
    let morse = text_to_morse(text);
    morse_to_num(&morse)
}

// Decode Pollux numeric code to text
fn decode_pollux(nums: &str) -> String {
    let morse = num_to_morse(nums);
    morse_to_text(&morse)
}

fn main() {
    let test_string = "CMSC191";
    let encoded = encode_pollux(test_string);
    println!("Encoded: {}", encoded);
    let decoded = decode_pollux(&encoded);
    println!("Decoded: {}", decoded);
}
