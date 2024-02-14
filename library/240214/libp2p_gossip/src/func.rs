use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, time::Duration};
use futures::StreamExt;
use libp2p::{gossipsub, mdns, noise, swarm::{NetworkBehaviour, SwarmEvent}, tcp, yamux, Swarm};
use tokio::{io, io::AsyncBufReadExt, select};
use tracing_subscriber::EnvFilter;

use crate::log;

#[derive(NetworkBehaviour)]
pub struct GossipBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub fn init_setting() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .expect("Can not initial setting");
}

pub fn listen_swarm(swarm: &mut Swarm<GossipBehaviour>, protocol: &str) {
    swarm.listen_on(
        format!("/ip4/0.0.0.0/{protocol}")
            .parse()
            .expect("Address is not parsing")
    )
    .expect("Swarm is not listening");
}

pub fn subscribe_topic(swarm: &mut Swarm<GossipBehaviour>, topic: &str) {
    swarm.behaviour_mut().gossipsub
        .subscribe(&gossipsub::IdentTopic::new(topic))
        .expect("Swarm is not subscribe to topic");
}

pub fn create_gossip_swarm() -> Swarm<GossipBehaviour> {
    libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )
        .expect("Can not create TCP")
        .with_quic()
        .with_behaviour(|key| {
            // To content-address message, we can take the hash of message and use it as an ID.
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            // Set a custom gossipsub configuration
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
                .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
                .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
                .duplicate_cache_time(Duration::from_secs(5))
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?; // Temporary hack because `build` does not return a proper `std::error::Error`.

            // build a gossipsub network behaviour
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;

            Ok(GossipBehaviour { gossipsub, mdns } )
        })
        .expect("Can not define behaviours")
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build()
}

pub async fn select_loop(swarm: &mut Swarm<GossipBehaviour>,  topic: &str) {
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    let topic = gossipsub::IdentTopic::new(topic);
    let log_file = "./log.txt";

    loop {
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                match swarm.behaviour_mut().gossipsub.publish(topic.clone(), line.as_bytes()) {
                    Ok(id) => {
                        log::append_log(log_file, None, &format!("Publish ID: {id} Data: {line}"));
                    },
                    Err(e) => println!("Publish error: {e:?}"),
                }
            }
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(GossipBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        log::append_log(log_file, None, &format!("mDNS discovered a new peer: {peer_id}"));

                        println!("mDNS discovered a new peer: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(GossipBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        log::append_log(log_file, None, &format!("mDNS discovered peer has expired: {peer_id}"));

                        println!("mDNS discover peer has expired: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(GossipBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => {
                    log::append_log(log_file, Some(peer_id), &format!("ID: {id} Data: {}", &String::from_utf8_lossy(&message.data)));

                    println!(
                        "Got message: '{}' with id: {id} from peer: {peer_id}",
                        String::from_utf8_lossy(&message.data),
                    )
                },
                SwarmEvent::NewListenAddr { address, .. } => {
                    log::append_log(log_file, None, &format!("Local node is listening on {address}"));

                    println!("Local node is listening on {address}");
                }
                _ => {}
            }
        }
    }
}
