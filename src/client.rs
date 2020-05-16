use ipd::ipd_client::IpdClient;
use ipd::NewGameRequest;

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
    Ok(())
}