fn piggify(wat: &str) -> String {
    let vowels: &str = "AEIOU";

    let first_character = wat.chars().next().unwrap();
    if vowels.contains(first_character) {
        // vowel, add -hay
        return format!("{}-hay", wat);
    } else {
        // Consonant, move first letter to end and add -ay
        return format!("{}-{}ay", wat[1..].to_string(), first_character);
    }
}

fn main() {
    println!("{}",piggify("BIsNotFIne"));
}