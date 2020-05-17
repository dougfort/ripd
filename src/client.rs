use std::io::Write;
use std::io::{stdin, stdout};

use futures::stream::iter;
use ipd::ipd_client::IpdClient;
use ipd::{Action, ActionRequest, NewGameRequest};

mod ipd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut player_name = "player";
    let mut game_id: String = "".to_string();
    let mut opponent_name: String = "".to_string();

    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let mut client = IpdClient::new(channel);
    loop {
        let stdin_line = get_stdin_line(">")?;
        let command: Vec<&str> = stdin_line.trim().split_whitespace().collect();
        if command.is_empty() {
            continue;
        }
        match command[0].trim() {
            "" => continue,
            "quit" => break,
            "new" => {
                let request = tonic::Request::new(NewGameRequest {
                    player_name: player_name.to_string(),
                });
                let response = client.new_game(request).await?.into_inner();
                game_id = response.game_id;
                opponent_name = response.opponent_name;
                println!("game is = {}; opponenent = {}", game_id, opponent_name);
            }
            "play" => {
                let stream_request = tonic::Request::new(iter(vec![
                    ActionRequest {
                        game_id: game_id.to_string(),
                        sequence: 1,
                        action: Action::Defect as i32,
                    },
                    ActionRequest {
                        game_id: game_id.to_string(),
                        sequence: 2,
                        action: Action::Defect as i32,
                    },
                    ActionRequest {
                        game_id: game_id.to_string(),
                        sequence: 3,
                        action: Action::Defect as i32,
                    },
                ]));
                let mut response = client.play_game(stream_request).await?.into_inner();
                while let Some(res) = response.message().await? {
                    println!("NOTE = {:?}", res);
                }
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
