mod func;
mod log;

#[tokio::main]
async fn main() {
    let log_file = "./log.txt";
    let topic = "gossip-test";
    
    func::init_setting();
    log::exist_and_create_log(log_file);
    
    let mut swarm = func::create_gossip_swarm();

    func::subscribe_topic(&mut swarm, topic);

    func::listen_swarm(&mut swarm, "udp/0/quic-v1");
    func::listen_swarm(&mut swarm, "tcp/0");

    func::select_loop(&mut swarm, topic).await;
}
