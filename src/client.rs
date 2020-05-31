#[macro_use]
extern crate log;

use ipd::ipd_client::IpdClient;
use ipd::{Action, ActionRequest, NewGameRequest};

mod ipd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let player_name: String = "noname".to_string();
    let addr = "http://[::1]:50051";

    info!("connecting to {}", addr);
    let channel = tonic::transport::Channel::from_static(addr)
        .connect()
        .await?;
    let mut client = IpdClient::new(channel);
    info!("requesting new game: {}", player_name);
    let game_request = tonic::Request::new(NewGameRequest {
        player_name: player_name.to_string(),
    });
    info!("waiting response");
    let game_response = client.new_game(game_request).await?.into_inner();
    info!(
        "game is = {}; opponenent = {}",
        game_response.game_id, game_response.opponent_name
    );

    let mut n: i32 = 0;
    loop {
        let action_request = tonic::Request::new(ActionRequest {
            game_id: game_response.game_id,
            action: 1,
        });
        let action_result = client.play(action_request).await?.into_inner();
        info!(
            "game: {}, action: {}, payoff: {}",
            action_result.game_id,
            Action::from(action_result.action),
            action_result.payoff
        );
        n += 1;
        if n == 10 {
            break;
        }
    }

    Ok(())
}
