fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest(x: &str, y: &str) -> String {
    if x > y {
        let a = "cat";
        return a.to_string();
    }
    let result = String::from("really long string");
    return result;
}

struct ImportantExcerpt<'a> {
    part: &'a str
}