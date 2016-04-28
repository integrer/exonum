#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate mio;
extern crate sodiumoxide;

mod message;
mod connection;
mod network;
mod events;
mod signature;
mod state;
mod node;

use node::{Node, Configuration};
use network::{NetworkConfiguration};
use events::{EventsConfiguration};
use signature::{gen_keypair};

fn main() {
    env_logger::init().unwrap();

    let addresses : Vec<::std::net::SocketAddr> = vec![
        "127.0.0.1:7000".parse().unwrap(),
        "127.0.0.1:7001".parse().unwrap(),
        "127.0.0.1:7002".parse().unwrap(),
        "127.0.0.1:7003".parse().unwrap(),
    ];

    let mut nodes = Vec::new();
    for address in &addresses {
        let (public_key, secret_key) = gen_keypair();
        nodes.push(Node::with_config(Configuration {
            public_key: public_key,
            secret_key: secret_key,
            network: NetworkConfiguration {
                listen_address: address.clone(),
                max_incoming_connections: 8,
                max_outgoing_connections: 8,
            },
            events: EventsConfiguration::new(),
            peer_discovery: addresses.clone()
        }))
    }

    ::std::thread::sleep_ms(100);

    let mut threads = Vec::new();
    for mut node in nodes {
        threads.push(::std::thread::spawn(move || {
            node.run()
        }))
    }

    for thread in threads {
        thread.join();
    }
}