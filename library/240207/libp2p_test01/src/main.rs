use futures::prelude::*;
use libp2p::{core::transport::ListenerId, noise, ping::{self, Config}, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Multiaddr, Swarm};
use std::{error::Error, time::Duration};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    setting();

    let max: i32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut range: Vec<Multiaddr> = vec![];
    let mut i = 1;

    loop {
        if i > max {
            println!("");
            break;
        }

        print!("{}, ", i);
        range.push(format!("/ip4/127.0.0.{}/tcp/0", i).parse().unwrap());

        i += 1;
    }

    let mut swarms: Vec<Swarm<ping::Behaviour>> = vec![];

    for v in &range {
        let mut swarm = match create_swarm() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Do not create swarm");
                return Result::Err(e);
            },
        };

        let _ = listen_swarm(&mut swarm, v);

        swarms.push(swarm);
    }

    
    for mut swarm in swarms {
        let range = range.clone();

        swarm = dial_swarm(swarm, &range).await;

        tokio::spawn(async move {
            loop {
                match swarm.select_next_some().await {
                    SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
                    SwarmEvent::Behaviour(event) => println!("{event:?}"),
                    _ => {},
                }
            }
        });
    }

    loop {}
}

async fn dial_swarm(mut swarm: Swarm<ping::Behaviour>, range: &Vec<Multiaddr>) -> Swarm<ping::Behaviour> {
    for m in range {
        match swarm.dial(m.clone()) {
            Ok(_) => {
                println!("Dial {:?}", *m);
                continue;
            },
            Err(e) => {
                eprintln!("Do not dial {:?}", e);
                continue;
            },
        };
    }

    swarm
}

// Start listening from swarm
fn listen_swarm(swarm: &mut Swarm<ping::Behaviour>, addr: &Multiaddr) -> Result<ListenerId, Box<dyn Error>> {
    Result::Ok(swarm.listen_on(addr.clone())?)
}

// Create the Resulted swarm and return
fn create_swarm() -> Result<Swarm<ping::Behaviour>, Box<dyn Error>> {
    Result::Ok(libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| {
            ping::Behaviour::new(Config::default().with_interval(Duration::from_secs(3)))
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build())
}

// Set default tracing subscriber setting
fn setting() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();
}