use console::Term;
use rand::seq::SliceRandom;
use std::fs;
use std::io::Write;

#[macro_use]
mod color;
use color::Color;

fn main() {
    println!(
        "{}",
        color::color_bold("[*] Starting Bee Name Gen", Color::Green)
    );

    let mut names: Vec<String> = Vec::new();

    // Get files in the 'data' directory
    cprint!(Color::Green, "[*] Loading Dir");
    let paths = fs::read_dir("./data").expect("Error Getting data files");
    let mut loaded_files = 0;

    cprint!(Color::Green, "[*] Loading Files");
    // Loop through each file and load its contents
    for raw_path in paths {
        let entry = raw_path.unwrap();
        let path = entry.path();
        cprint!(
            Color::Green,
            "\r[*] Loading: {}\x1b[1A",
            color::color(
                &format!("{} ({})", path.to_str().unwrap(), loaded_files),
                color::Color::Cyan
            )
        );
        std::io::stdout().flush().expect("Error Flushing StdOut");
        let contents =
            fs::read_to_string(&path).expect(&format!("Error Reading: {}", path.to_str().unwrap()));

        let lines = contents.split('\n').collect::<Vec<&str>>();

        for line in lines {
            let data = line.split(',').collect::<Vec<&str>>();
            let name = data.first().unwrap().to_string();
            if !names.contains(&name) && &name != "" && &name[..1] == "B" {
                names.push(name);
            }
        }
        loaded_files += 1;
    }

    names.sort();
    cprint!(Color::Green, "\n[*] Loaded {} B-names", names.len());
    cprint!(Color::Green, "[*] Done\n");
    cprint!(Color::Magenta, "R - Pick Random Name");
    cprint!(Color::Magenta, "Q - Exit");
    println!("\n");

    loop {
        let input: char = Term::stdout()
            .read_char()
            .unwrap()
            .to_lowercase()
            .next()
            .unwrap_or(' ');

        // Clear Line
        // Goes up one line, clears the line, then goes back down
        print!("\x1b[1A\x1b[K\x1b[1B");

        // 5 Star Command Parser
        match input {
            // Random Name
            'r' => {
                cprint!(
                    Color::Blue,
                    "\x1b[1A[+] Random Name: {}",
                    names
                        .choose(&mut rand::thread_rng())
                        .expect("Error Picking Name :/")
                );
            }
            // Exit
            'q' => {
                cprint!(Color::Red, "\x1b[1A[+] Exiting - Goodby :P");
                break;
            }
            // *nothing*
            '\n' => println!("\x1b[1A"),
            // No Match
            _ => {
                cprint!(Color::Red, "\x1b[1A[-] Unknown Command: {}", input);
            }
        }
        // Flush stdout... Because its buffered
        std::io::stdout().flush().expect("Error Flushing StdOut");
    }
}
