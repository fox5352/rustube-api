use std::{io::{stdin, stdout, Write}, process::exit};
// local
use rustube_api::video_downloader::Video;

#[tokio::main]
async fn main() {
    let mut input = String::new();

    // ---------------------- gets input ---------------------- 
    print!("enter url: ");
    // Flush the output stream to ensure the prompt is printed before waiting for input
    if let Err(e) = stdout().flush() {
        eprintln!("Failed to flush output stream: {}", e);
        return;
    }
    // Get user input
    if let Err(e) = stdin().read_line(&mut input) {
        eprintln!("Failed to read user input: {}", e);
        return;
    }

    // ---------------------- checks input ---------------------- 
    if input.len() == 0 {
        eprintln!("input is empty");
        exit(1);
    }
    let commands: Vec<&str> = input.trim().split(" ").collect();    

    // ---------------------- creates video struct ---------------------- 
    let video = match Video::new(commands[0].to_string()).await {
      Ok(data) => data,
      Err(e) => {
          eprintln!("error: {}", e);
          exit(1);
      }
    };

    // ---------------------- parses input & executes command ----------------------
    if commands.len() > 1 {
      // TODO: switch for regex later
        if commands[1] == "-a" {
          video.get_audio_stream().await.unwrap();
        }
    }else {
      video.get_best_quality().await.unwrap();
    }
}