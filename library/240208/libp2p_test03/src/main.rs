use futures::prelude::*;
use libp2p::{noise, ping::{self, Config}, swarm::SwarmEvent, tcp, yamux, Multiaddr, Swarm};
use std::{error::Error, time::Duration};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    if let Err(e) = default_setting() {
        eprintln!("{:?}", e);
    }
    
    let (addr_num, max_peer) = env_args_parse();
    let range: Vec<Multiaddr> = create_addr_range(max_peer);
    let swarm = create_swarm();
    let swarm = listen_swarm(swarm, create_ip(addr_num));

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let swarm = dial_swarm(swarm, &range).await;

    tokio::spawn(async move {
        behavior_roof(swarm).await;
    });

    loop {}
}

async fn behavior_roof(mut swarm: Swarm<ping::Behaviour>) {
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            _ => {},
        }
    }
}

fn create_addr_range(max_peer: i32) -> Vec<Multiaddr> {
    
    let mut range: Vec<Multiaddr> = vec![];
    let mut i = 1;

    loop {
        if i > max_peer {
            println!("");
            break;
        }

        print!("{}, ", i);
        range.push(create_ip(i));

        i += 1;
    }

    range
}

fn create_ip(addr_num: i32) -> Multiaddr {
    format!("/ip4/127.0.0.{}/tcp/65432", &addr_num).parse().unwrap()
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

fn listen_swarm(mut swarm: Swarm<ping::Behaviour>, addr: Multiaddr) -> Swarm<ping::Behaviour> {
    let _ = swarm.listen_on(addr.clone());
    
    swarm
}

fn create_swarm() -> Swarm<ping::Behaviour> {
    let tcp = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        );

    let behavior = match tcp {
        Ok(o) => {
            o.with_behaviour(|_| {
                ping::Behaviour::new(Config::default().with_interval(Duration::from_secs(3)))
            })
        },
        Err(e) => panic!("{:?}", e),
    };
            
    let build = match  behavior {
        Ok(o) => {
            o.with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
                .build()
        },
        Err(e) => panic!("{:?}", e),
    };

    build
}

fn default_setting() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
}

fn env_args_parse() -> (i32, i32) {
    if std::env::args().len() != 3 {
        panic!("env args' count is only 2");
    }

    (std::env::args().nth(1).unwrap().parse().unwrap(),
        std::env::args().nth(2).unwrap().parse().unwrap())
}