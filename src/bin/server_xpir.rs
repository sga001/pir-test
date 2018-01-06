extern crate getopts;
extern crate pir_test;
extern crate rand;
extern crate tarpc;
extern crate tokio_core;

use tarpc::future::server;
use tarpc::util::FirstSocketAddr;
use pir_test::test_xpir::FutureServiceExt;
use pir_test::test_xpir::PirTestServer;
use rand::RngCore;
use tokio_core::reactor;
use getopts::Options;

fn print_usage(program: &str, opts: &Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

    // parameters
    opts.optflag("", "help", "print this help menu");
    opts.optopt("h", "host", "server's address", "IP:PORT");
    opts.optopt("n", "num", "number of elements", "NUM");
    opts.optopt("d", "dimensions", "number of dimensions", "DIM");
    opts.optopt("a", "alpha", "aggegation parameter", "ALPHA");
    opts.optopt("i", "iterations", "number of iterations", "ITER");

    const ELE_SIZE: usize = 288;

    // Parse parameters
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            print_usage(&program, &opts);
            panic!(e.to_string())
        }
    };

    if matches.opt_present("help") {
        print_usage(&program, &opts);
        return;
    }

    let server_addr: String = match matches.opt_str("h") {
        Some(v) => v,
        None => "localhost:12345".to_string(),
    };

    let ele_num: u64 = match matches.opt_str("n") {
        Some(v) => u64::from_str_radix(&v, 10).unwrap(),
        None => 8192,
    };

    let alpha: u64 = match matches.opt_str("a") {
        Some(v) => u64::from_str_radix(&v, 10).unwrap(),
        None => 18,
    };

    let d: u64 = match matches.opt_str("d") {
        Some(v) => u64::from_str_radix(&v, 10).unwrap(),
        None => 2,
    };

    let mut collection: Vec<[u8; ELE_SIZE]> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..ele_num {
        let mut x: [u8; ELE_SIZE] = [0; ELE_SIZE];
        rng.fill_bytes(&mut x);
        collection.push(x);
    }

    let server = PirTestServer::new(&collection, alpha, d);

    let mut reactor = reactor::Core::new().unwrap();
    let (_, conn) = server
        .listen(
            server_addr.first_socket_addr(),
            &reactor.handle(),
            server::Options::default().max_payload_size(1 << 28),
        )
        .unwrap();

    println!("Ready");

    reactor.run(conn).unwrap();
}
