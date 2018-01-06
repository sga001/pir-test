extern crate chrono;
extern crate getopts;
extern crate pir_test;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate tarpc;
extern crate time;
extern crate tokio_core;
extern crate xpir;

#[macro_use]
extern crate serde_derive;

use tarpc::future::client;
use xpir::client::PirClient;
use client::ClientExt;
use tokio_core::reactor::Core;
use tarpc::util::FirstSocketAddr;
use pir_test::test_xpir::FutureClient;
use time::PreciseTime;
use rand::RngCore;
use getopts::Options;
use chrono::prelude::*;
use std::thread;

#[derive(Serialize)]
struct Results {
    date: DateTime<Utc>, // UTC time
    num: u32,            // number of entries
    d: u32,              // depth
    a: u32,              // alpha
    i: u32,              // number of iterations
    point: Vec<i64>,     //data points in microseconds
}

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
    opts.optopt("c", "threads", "number of threads", "THREADS");

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

    let iter: usize = match matches.opt_str("i") {
        Some(v) => usize::from_str_radix(&v, 10).unwrap(),
        None => 10,
    };

    let num_threads: u32 = match matches.opt_str("c") {
        Some(v) => u32::from_str_radix(&v, 10).unwrap(),
        None => 1,
    };


    let mut thread_pids = Vec::new();

    for _ in 0..num_threads {
        let addr_clone = server_addr.clone();

        let thread_pid = thread::spawn(move || {
            let mut reactor = Core::new().unwrap();
            let options = client::Options::default()
                .max_payload_size(1 << 28)
                .handle(reactor.handle());
            let conn = reactor
                .run(FutureClient::connect(
                    addr_clone.first_socket_addr(),
                    options,
                ))
                .unwrap();

            let pir_handle = PirClient::with_params(ELE_SIZE as u64, ele_num, alpha, d);
            let mut rng = rand::thread_rng();

            let mut times = Vec::new();

            for _ in 0..iter {
                let start = PreciseTime::now();

                let idx = rng.next_u64() % ele_num;
                let query = pir_handle.gen_query(idx);
                let reply = reactor.run(conn.get(query)).unwrap();
                let _element: [u8; ELE_SIZE] = pir_handle.decode_reply(&reply);

                let end = PreciseTime::now();

                let duration = start.to(end).num_microseconds().unwrap();
                times.push(duration);
            }

            let result = Results {
                date: Utc::now(),
                num: ele_num as u32,
                d: d as u32,
                a: alpha as u32,
                i: iter as u32,
                point: times,
            };

            let j = serde_json::to_string(&result).unwrap();
            println!("{}", j);
        });

        thread_pids.push(thread_pid);
    }

    for pid in thread_pids {
        let _ = pid.join();
    }
}
