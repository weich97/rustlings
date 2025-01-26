// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    // "blue" is &str, so we call string_slice.
    string_slice("blue");

    // "red".to_string() produces a String, so we call string.
    string("red".to_string());

    // String::from("hi") produces a String, so we call string.
    string(String::from("hi"));

    // "rust is fun!".to_owned() produces a String, so we call string.
    string("rust is fun!".to_owned());

    // "nice weather".into() produces a String, so we call string.
    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    // "  hello there ".trim() produces a &str, so we call string_slice.
    string_slice("  hello there ".trim());

    // "Happy Monday!".replace("Mon", "Tues") produces a String, so we call string.
    string("Happy Monday!".replace("Mon", "Tues"));

    // "mY sHiFt KeY iS sTiCkY".to_lowercase() produces a String, so we call string.
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
