use std::{env, io::{stdin, stdout, Write}, process::exit};
// local
use rustube_api::video_downloader::Video;

#[tokio::main]
async fn main() {
  let mut args = env::args();
  args.next();

  // ---------------------- args as input ---------------------- 
  if args.len() > 0 {
    let url = match args.next() {
      Some(data) => data,
      None => {
        eprintln!("No URL provided");
        exit(1);
      }
    };
      
    let video = match Video::new(url).await {
      Ok(data) => data,
      Err(e) => {
        eprintln!("error: {}", e);
        exit(1);
      }
    };

    if args.len() > 0 {
      let flag = match args.next() {
          Some(data) => data,
          None => {
            eprintln!("No command provided");
            exit(1);
          }
      };
      if flag == "-a" {
          video.get_audio_stream().await.unwrap();
      }else {
        println!("flag not recognized: {}", flag);
        video.get_best_quality().await.unwrap();
      }
    }else {
      video.get_best_quality().await.unwrap();
    }

  }else {
    // ---------------------- gets input ---------------------- 
    let mut input = String::new();
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
      if commands[1] == "-a" {
        video.get_audio_stream().await.unwrap();
      }
    }else {
      video.get_best_quality().await.unwrap();
    }
  }
}