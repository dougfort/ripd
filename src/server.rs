#[macro_use]
extern crate log;

use ipd::ipd_server::{Ipd, IpdServer};
use ipd::{ActionRequest, ActionResult, NewGameRequest, NewGameResponse};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status, Streaming};
mod ipd;

#[derive(Default)]
pub struct IpdData {}

impl IpdData {
    fn new() -> Self {
        IpdData {}
    }
}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Ipd for IpdData {
    type PlayGameStream = mpsc::Receiver<Result<ActionResult, Status>>;

    async fn new_game(
        &self,
        request: Request<NewGameRequest>,
    ) -> Result<Response<NewGameResponse>, Status> {
        info!("new_game: {}", request.get_ref().player_name);
        Ok(Response::new(NewGameResponse {
            game_id: "foot".to_string(),
            opponent_name: "clam".to_string(),
        }))
    }

    async fn play(
        &self,
        request: Request<ActionRequest>,
    ) -> Result<Response<ActionResult>, Status> {
        debug!("play: {:?}", request);
        Err(Status::unimplemented("play"))
    }

    async fn play_game(
        &self,
        request: Request<Streaming<ActionRequest>>,
    ) -> Result<Response<Self::PlayGameStream>, Status> {
        debug!("stream_game: {:?}", request);
        Err(Status::unimplemented("play"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = "[::1]:50051".parse().unwrap();
    let ipd = IpdData::new();
    info!("Server listening on {}", addr);
    Server::builder()
        .add_service(IpdServer::new(ipd))
        .serve(addr)
        .await?;
    Ok(())
}
