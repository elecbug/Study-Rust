use libp2p::gossipsub;

mod func;
mod log;

#[tokio::main]
async fn main() {
    let log_file = "./log.txt";
    let topic = "gossip-test";
    
    func::init_setting();
    log::exist_and_create_log(log_file);
    
    let mut swarm = func::create_gossip_swarm();
    
    swarm.behaviour_mut().gossipsub
        .subscribe(&gossipsub::IdentTopic::new(topic))
        .expect("Swarm is not subscribe to topic");

    swarm.listen_on(
        "/ip4/0.0.0.0/udp/0/quic-v1"
            .parse()
            .expect("Address is not parsing")
    ).expect("Swarm is not listening");
    swarm.listen_on(
        "/ip4/0.0.0.0/tcp/0"
        .parse()
        .expect("Address is not parsing")
    ).expect("Swarm is not listening");

    func::select_loop(&mut swarm, topic).await;
}
