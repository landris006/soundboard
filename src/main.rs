use rodio::{Decoder, OutputStream, Sink};
use std::fs::{self, File};
use std::io::{stdin, BufReader, ErrorKind};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("This program takes exactly one argument!");
        return;
    }

    if args[1] == "ls" {
        let files = match fs::read_dir("sounds") {
            Ok(files) => files
                .map(|file| {
                    file.unwrap()
                        .path()
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                })
                .collect::<Vec<String>>(),
            Err(_) => {
                println!("No sound files found, make sure to put a folder named 'sounds' next to this executable,\ninside the folder put .mp3 files! (e.g. 'example_sound.mp3')");
                return;
            }
        };

        if files.len() == 0 {
            println!("No sound files found, make sure to put a folder named 'sounds' next to this executable,\ninside the folder put .mp3 files! (e.g. 'example_sound.mp3')");
            return;
        }

        for (index, file) in files.iter().enumerate() {
            println!("{}. {}", index + 1, file);
        }

        println!("");
        println!("Enter the index of the sound to play it: ");
        fn take_input(files: &Vec<String>) {
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .ok()
                .expect("Could not read user input...");

            if input.trim() == String::from("q") || input.trim() == String::from("") {
                return;
            }

            match input.trim().parse::<i32>() {
                Ok(number) => {
                    if number < 1 || number > files.len() as i32 {
                        println!(
                        "Supplied number is out of range, please enter a number between {} and {}!",
                        1,
                        files.len());
                        take_input(files);
                    }

                    play_audio(&files[(number - 1) as usize]);
                }
                Err(_) => {
                    println!("Please supply a valid number!");
                    take_input(files);
                }
            };
        }

        take_input(&files);
        return;
    }

    play_audio(&args[1]);
}

fn play_audio(name: &String) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(match File::open(format!("sounds/{}.mp3", name)) {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => println!("Could not find the file specified, supply the 'ls' argument, to list avaliable sounds."),
                _ => {println!("Could not open file...")}
            };
            process::exit(0)
        }
    });
    let source = Decoder::new(file).unwrap();

    sink.append(source);

    sink.sleep_until_end();
    process::exit(0);
}
