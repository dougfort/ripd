#[macro_use]
extern crate log;

use ipd::ipd_server::{Ipd, IpdServer};
use ipd::{ActionRequest, ActionResult, NewGameRequest, NewGameResponse};
use payoff::compute_payoff;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};
mod ipd;
mod payoff;

#[derive(Default)]
pub struct GameData {
    score: u32,
    history: Vec<(ipd::Action, ipd::Action)>,
}

#[derive(Default)]
pub struct IpdData {
    game_id: AtomicU32,
    game_map: Mutex<HashMap<u32, GameData>>,
}

impl IpdData {
    fn new() -> Self {
        IpdData {
            game_id: AtomicU32::new(1),
            game_map: Mutex::new(HashMap::new()),
        }
    }
}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl Ipd for IpdData {
    type PlayGameStream = ReceiverStream<Result<ActionResult, Status>>;

    async fn new_game(
        &self,
        request: Request<NewGameRequest>,
    ) -> Result<Response<NewGameResponse>, Status> {
        let request = request.into_inner();
        let game_id = self.game_id.fetch_add(1, Ordering::Relaxed);

        info!(
            "new_game: player: {}; game_id: {}",
            request.player_name, game_id
        );

        let game_data = GameData {
            score: 0,
            history: Vec::new(),
        };
        {
            let mut game_map = self.game_map.lock().unwrap();
            game_map.insert(game_id, game_data);
        }
        Ok(Response::new(NewGameResponse {
            game_id,
            opponent_name: "Calculon".to_string(),
        }))
    }

    async fn play(
        &self,
        request: Request<ActionRequest>,
    ) -> Result<Response<ActionResult>, Status> {
        let request = request.into_inner();
        let game_id = request.game_id;
        let opponent_action = ipd::Action::from(request.action);
        let mut game_map = self.game_map.lock().unwrap();
        let game_data = match game_map.get_mut(&game_id) {
            Some(gd) => gd,
            None => return Err(Status::unavailable(format!("no such game: {}", game_id))),
        };
        let our_action = ipd::Action::Defect;
        let (our_payoff, opponent_payoff) = compute_payoff(our_action, opponent_action);
        game_data.score += our_payoff;
        info!(
            "play: game_id: {}; action: ({}, {}), payoff ({}, {}), score: {}",
            game_id, our_action, opponent_action, our_payoff, opponent_payoff, game_data.score,
        );
        game_data.history.push((opponent_action, our_action));
        Ok(Response::new(ActionResult {
            game_id,
            action: i32::from(our_action),
            payoff: opponent_payoff,
        }))
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
