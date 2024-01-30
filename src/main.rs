extern crate vi;
use std::env;
use std::process::Command;
use vi::telex;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("Usage: ./tv <input>");
        return;
    }

    let inputs = args.join(" ");

    let words = inputs.split(' ');

    let mut result = String::new();
    for word in words {
        telex::transform_buffer(word.chars(), &mut result);
        result.push(' ');
    }

    let result_trimmed = result.trim();

    println!("{}", result_trimmed);


    let _ = Command::new("termux-clipboard-set")
        .arg(result_trimmed)
        .output();

    // Gán kết quả vào clipboard sử dụng termux-clipboard-set
    /* let clipboard_command = Command::new("termux-clipboard-set")
        .arg(result_trimmed)
        .output(); */
    /* match clipboard_command {
        Ok(_) => println!(),
        Err(e) => eprintln!("Error copying to clipboard: {}", e),
    } */
}
