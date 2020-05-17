use ipd::ipd_server::{Ipd, IpdServer};
use ipd::{Action, ActionRequest, ActionResult, NewGameRequest, NewGameResponse};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status, Streaming};
mod ipd;

#[derive(Default)]
pub struct IpdData {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Ipd for IpdData {
    type PlayGameStream = mpsc::Receiver<Result<ActionResult, Status>>;

    async fn new_game(
        &self,
        _request: Request<NewGameRequest>,
    ) -> Result<Response<NewGameResponse>, Status> {
        Ok(Response::new(NewGameResponse {
            game_id: "foot".to_string(),
            opponent_name: "clam".to_string(),
        }))
    }

    async fn play_game(
        &self,
        request: Request<Streaming<ActionRequest>>,
    ) -> Result<Response<Self::PlayGameStream>, Status> {
        let mut streamer = request.into_inner();
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            while let Some(req) = streamer.message().await.unwrap() {
                tx.send(Ok(ActionResult {
                    game_id: req.game_id,
                    sequence: req.sequence,
                    opponent_action: Action::Cooperate as i32,
                    score: 1,
                }))
                .await;
            }
        });
        Ok(Response::new(rx))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let ipd = IpdData::default();
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(IpdServer::new(ipd))
        .serve(addr)
        .await?;
    Ok(())
}
