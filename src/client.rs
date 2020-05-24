#[macro_use]
extern crate log;

use std::io::Write;
use std::io::{stdin, stdout};

use ipd::ipd_client::IpdClient;
use ipd::{NewGameRequest};

mod ipd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut player_name: String = "".to_string();

    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let mut client = IpdClient::new(channel);
    loop {
        let stdin_line = get_stdin_line(&format!("({}) >", player_name))?;
        let command: Vec<&str> = stdin_line.trim().split_whitespace().collect();
        if command.is_empty() {
            continue;
        }
        match command[0].trim() {
            "" => continue,
            "quit" => break,
            "new" => {
                if command.len() < 2 {
                    println!("you must specify a player name");
                    continue;
                }
                player_name = command[1].trim().to_owned();
                info!("requesting new game: {}", player_name);
                let request = tonic::Request::new(NewGameRequest {
                    player_name: player_name.to_string(),
                });
                info!("waiting response");
                let response = client.new_game(request).await?.into_inner();
                info!(
                    "game is = {}; opponenent = {}",
                    response.game_id, response.opponent_name
                );
            }
            _ => println!("unknown command: '{:?}'", command),
        }
    }
    Ok(())
}

fn get_stdin_line(prompt: &str) -> std::io::Result<String> {
    println!();
    print!("{} ", prompt);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(input)
}
