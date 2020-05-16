use tonic::{transport::Server, Request, Response, Status, Streaming};
use std::pin::Pin;
use futures::Stream;
use ipd::ipd_server::{Ipd, IpdServer};
use ipd::{NewGameRequest, NewGameResponse, ActionRequest, ActionResult};
mod ipd; 

#[derive(Default)]
pub struct IpdData {}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<ActionResult, Status>> + Send + Sync>>;

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Ipd for IpdData {

    type PlayGameStream = ResponseStream;

    async fn new_game(&self,_request:Request<NewGameRequest>)->Result<Response<NewGameResponse>,Status>{
        Ok(Response::new(NewGameResponse{
            game_id: "foot".to_string(),
            opponent_name: "clam".to_string(),
        }))
    }

    async fn play_game(
        &self,
        _request: Request<Streaming<ActionRequest>>,
    ) -> Result<Response<ResponseStream>, Status> {
        Err(Status::unimplemented("not implemented"))
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