use std::process::Command;
use data2sound::encode;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = args[1].as_str();
    send_file(filename);
}

pub fn send_file(filename: &str) {
    let sound_file: &str = &format!("{filename}.wav");
    //
    convert_file_to_sound(filename, sound_file);

    send_sound_file(sound_file);

    fs::remove_file(sound_file).expect("Could not remove temporary file");
}

pub fn convert_file_to_sound(filename: &str, sound_file: &str) {
    encode(filename, sound_file).unwrap()
}

pub fn send_sound_file(sound_file : &str) {
    let command = format!("{}/src/fm_transmitter/fm_transmitter {}", env::current_dir().expect("").display(), sound_file);

    let output = Command::new("sh")
        .arg("-C")
        .arg(command)
        .output()
        .expect("failed to execute process");

    Command::new("ls")
        .arg("-la")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
