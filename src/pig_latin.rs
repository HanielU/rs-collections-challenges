pub fn main() {
    let latin = pig_latinize("first");
    println!("latin = {}", latin);
}

fn pig_latinize(input: &str) -> String {
    let mut new_str = String::new();
    let input_chars = {
        let mut temp = Vec::new();
        for ch in input.chars() {
            temp.push(ch)
        }
        temp
    };

    new_str.push_str(input);
    new_str.push('-');

    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    if VOWELS.contains(&input_chars[0]) {
        new_str.push_str("hay");
    } else {
        new_str.push(input_chars[0]);
        new_str.push_str("ay")
    }

    return new_str;
}
