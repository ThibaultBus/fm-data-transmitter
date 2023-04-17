use std::process::Command;
use data2sound::encode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    convert_file_to_sound(file);
    send_sound_file(file);
}

pub fn convert_file_to_sound(file: &str) {
    encode(file, format!("{file}.wav")).unwrap()
}

pub fn send_sound_file(file : &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("./fm_transmitter/fm_transmitter {file}.wav"))
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
}
