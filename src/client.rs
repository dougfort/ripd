use futures::stream::iter;
use ipd::ipd_client::IpdClient;
use ipd::{Action, NewGameRequest, ActionRequest};

mod ipd;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
    let mut client = IpdClient::new(channel);
    let request = tonic::Request::new(
        NewGameRequest {
            player_name: String::from("clampet")
        },
    );
    let response = client.new_game(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    

    let stream_request = tonic::Request::new(iter(vec![
        ActionRequest {
            game_id: response.game_id.to_owned(),
            sequence: 1,
            action: Action::Defect as i32,
        },
        ActionRequest {
            game_id: response.game_id.to_owned(),
            sequence:2,
            action: Action::Defect as i32,
        },
        ActionRequest {
            game_id: response.game_id.to_owned(),
            sequence: 3,
            action: Action::Defect as i32,
        },
    ]));
    let mut response = client.play_game(stream_request).await?.into_inner();;
    while let Some(res) = response.message().await? {
        println!("NOTE = {:?}", res);
    }
    Ok(())
}