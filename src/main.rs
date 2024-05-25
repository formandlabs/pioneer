use std::{thread::sleep, time::Duration};

use clap::Parser;
use ed25519_dalek::VerifyingKey;
use mainline::Bytes;
use mainline::MutableItem;
use mainline::{
    server::{DhtServer, Server},
    Dht, Id,
};
use std::time::Instant;
use tracing::{info, instrument, Level};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// info_hash to annouce a peer on
    infohash: String,
}

#[derive(Debug, Default)]
struct PioneerNode {
    inner: DhtServer,
    dht: Option<Dht>,
}

impl Server for PioneerNode {
    #[instrument]
    fn handle_request(
        &mut self,
        rpc: &mut mainline::rpc::Rpc,
        from: std::net::SocketAddr,
        transaction_id: u16,
        request: &mainline::rpc::messages::RequestSpecific,
    ) {
        info!(?request, ?from, "Request from");

        println!("Request from: {:?}", from);

        self.inner
            .handle_request(rpc, from, transaction_id, request)
    }
}

impl PioneerNode {
    fn dht(&mut self) -> &Dht {
        if self.dht.is_none() {
            self.dht = Some(
                Dht::builder()
                    .custom_server(Box::new(PioneerNode::default()))
                    .build()
                    .unwrap(),
            );
        }
        self.dht.as_ref().unwrap()
    }

    #[instrument]
    pub fn announce(&mut self, info_hash: Id) {
        let start = Instant::now();

        self.dht()
            .announce_peer(info_hash, Some(6991))
            .expect("announce_peer failed");

        println!(
            "Announced peer in {:?} seconds",
            start.elapsed().as_secs_f32()
        );
    }

    #[instrument]
    pub fn get_immutable(&mut self, info_hash: Id) {
        let start = Instant::now();

        // No need to stream responses, just print the first result, since
        // all immutable data items are guaranteed to be the same.
        let value = self
            .dht()
            .get_immutable(info_hash)
            .expect("Failed to find the immutable value for the provided info_hash");

        let string = String::from_utf8(value.to_vec())
            .expect("expected immutable data to be valid utf-8 for this demo");

        println!(
            "Got result in {:?} milliseconds\n",
            start.elapsed().as_millis()
        );

        println!("Got immutable data: {:?}", string);

        println!(
            "\nQuery exhausted in {:?} milliseconds",
            start.elapsed().as_millis(),
        );
    }

    fn lookup(&mut self, public_key: VerifyingKey) {
        let start = Instant::now();
        let mut first = false;
        let mut count = 0;

        println!("Streaming mutable items:");
        for item in self
            .dht()
            .get_mutable(public_key.as_bytes(), None, None)
            .unwrap()
        {
            count += 1;

            if !first {
                first = true;
                println!(
                    "\nGot first result in {:?} milliseconds:",
                    start.elapsed().as_millis()
                );

                match String::from_utf8(item.value().to_vec()) {
                    Ok(string) => {
                        println!("  mutable item: {:?}, seq: {:?}\n", string, item.seq());
                    }
                    Err(_) => {
                        println!(
                            "  mutable item: {:?}, seq: {:?}\n",
                            item.value(),
                            item.seq(),
                        );
                    }
                };
            }
        }

        println!(
            "\nQuery exhausted in {:?} seconds, got {:?} values.",
            start.elapsed().as_secs_f32(),
            count
        );
    }

    fn from_hex(s: String) -> VerifyingKey {
        if s.len() % 2 != 0 {
            panic!("Number of Hex characters should be even");
        }

        let mut bytes = Vec::with_capacity(s.len() / 2);

        for i in 0..s.len() / 2 {
            let byte_str = &s[i * 2..(i * 2) + 2];
            let byte = u8::from_str_radix(byte_str, 16).expect("Invalid hex character");
            bytes.push(byte);
        }

        VerifyingKey::try_from(bytes.as_slice()).expect("Invalid mutable key")
    }

    fn get_peers(&mut self, info_hash: &Id) {
        let start = Instant::now();
        let mut first = false;

        let mut count = 0;

        for peer in self.dht().get_peers(*info_hash).unwrap() {
            if !first {
                first = true;
                println!(
                    "Got first result in {:?} milliseconds:",
                    start.elapsed().as_millis()
                );

                println!("peer {:?}", peer,);
            }

            count += 1;
        }

        println!(
            "\nQuery exhausted in {:?} milliseconds, got {:?} peers.",
            start.elapsed().as_millis(),
            count
        );
    }

    fn put_immutable(&mut self, value: &Bytes) {
        let start = Instant::now();

        let info_hash = self
            .dht()
            .put_immutable(value.to_owned())
            .expect("put immutable failed");

        println!(
            "Stored immutable data as {:?} in {:?} milliseconds",
            info_hash,
            start.elapsed().as_millis()
        );
    }

    fn put(dht: &Dht, item: &MutableItem) {
        let start = Instant::now();

        dht.put_mutable(item.clone()).expect("Put mutable failed");

        println!(
            "Stored mutable data as {:?} in {:?} milliseconds",
            item.target(),
            start.elapsed().as_millis()
        );
    }
}

fn main() {
    // spin up a thread to keep the server running
    let _ = std::thread::spawn(move || {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();

        let info_hash = Id::random(); //Id::from_str(cli.infohash.as_str()).expect("invalid infohash");
        let mut pioneer = PioneerNode::default();

        println!("\nAnnouncing peer on an infohash: {} ...\n", info_hash);

        println!("\n=== COLD QUERY ===");
        pioneer.announce(info_hash);

        println!("\n=== SUBSEQUENT QUERY ===");
        pioneer.announce(info_hash);
    });

    // sleep forever
    loop {
        sleep(Duration::from_secs(10000));
    }
}
