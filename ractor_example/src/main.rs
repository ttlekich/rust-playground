use ractor::{cast, Actor, ActorProcessingErr, ActorRef};

pub struct PingPong;

#[derive(Debug, Clone)]
pub enum Message {
    Ping,
    Pong,
}

impl Message {
    fn next(&self) -> Self {
        match self {
            Self::Ping => Self::Pong,
            Self::Pong => Self::Ping,
        }
    }

    fn print(&self) {
        match self {
            Self::Ping => println!("Ping"),
            Self::Pong => println!("Pong"),
        }
    }
}

#[async_trait::async_trait]
impl Actor for PingPong {
    type Msg = Message;
    type State = u8;
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        cast!(myself, Message::Ping)?;
        Ok(0u8)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if *state < 20u8 {
            message.print();
            cast!(myself, message.next())?;
            *state += 1;
        } else {
            println!();
            myself.stop(None);
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let (_actor, handle) = Actor::spawn(None, PingPong, ())
        .await
        .expect("Failed to spawn ping-pong actor");

    handle
        .await
        .expect("Ping-pong actor failed to exit properly");
}
