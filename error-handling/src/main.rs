use std::{arch::x86_64, fs::File, io::{self, Read}};

fn main() {

    // let username = read_username_from_file().expect("Something happened");
    // println!("{username}")

    let _ = test_the_tenerary();
    
    // match greeting_file_result {
    //     Ok(val) => println!("{val:#?}"),
    //     Err(err) => {
    //         match err.kind() {
    //             std::io::ErrorKind::NotFound => match File::create("src/hello.txt") {
    //                 Ok(val) => println!("{val:#?}"),
    //                 Err(err) => println!("{err:#?}")
    //             }
    //             other => println!("{other:#?}")
    //         }
    //     }
    // }

    // println!("{}")
}

// very confusing
fn parse_to_num (sttr: Option<&str>) -> Option<usize> {
    let num = sttr?.bytes();
    return Some(num.len());
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    let a = text.lines().next()?.chars();

    text.lines().next()?.chars().last()
}

fn test_the_tenerary () -> Result<String, io::Error> {
    let username = read_username_from_file()?;
    println!("{username}");
    Ok(username)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}