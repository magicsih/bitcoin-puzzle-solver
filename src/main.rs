extern crate bitcoin;
extern crate bitcoin_hashes;
extern crate rand;
extern crate reqwest;
extern crate secp256k1;

mod on_found;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use on_found::OnFound;
use rand::Rng;
use std::env;
use std::thread;
use std::time::Instant;
use thousands::Separable;

const POWER: u32 = 66;
const TARGET: &str = "13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so";

const CONSOLE_PRINT_THRESHOLD: u128 = 500_000_000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_threads: u32 = if args.len() < 2 {
        num_cpus::get() as u32 / 2
    } else {
        args[1].parse().expect("Failed to parse number of threads")
    };

    let begin = 2u128.pow(POWER - 1);
    let end: u128 = 2u128.pow(POWER);

    println!(
        "Puzzle solver main process started.\nconcurrency:{}\nsearch space:2^{}\n{}~{}\ntarget:{}",
        num_threads,
        POWER,
        begin.separate_with_commas(),
        end.separate_with_commas(),
        TARGET
    );

    let mut handles = vec![];

    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            random_lookfor(rng.gen_range(begin..end), Some(&on_found::on_found));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn random_lookfor(begin: u128, on_found: Option<&OnFound>) {
    println!(
        "{:?} starts searching from {}",
        thread::current().id(),
        begin.separate_with_commas()
    );
    let start = Instant::now();
    let secp = bitcoin::secp256k1::Secp256k1::new();

    let mut counter: u128 = 0;

    loop {
        let value: u128 = begin + counter;
        let private_key_hex = format!("{:0>64x}", value);
        let private_key_bytes = hex::decode(private_key_hex.clone()).unwrap();
        let private_key: PrivateKey = PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: bitcoin::secp256k1::SecretKey::from_slice(&private_key_bytes).unwrap(),
        };
        let public_key = private_key.public_key(&secp);
        let address = Address::p2pkh(&public_key, Network::Bitcoin).to_string();

        if address == TARGET {
            println!(
                "[{:?}] value: {} private key: {} address: {}",
                thread::current().id(),
                value,
                private_key,
                address
            );
           
            if let Some(on_found_callback) = on_found {
                on_found_callback(value, private_key, address);
            }
            break;
        }

        counter += 1;

        let throughput = counter as f64 / start.elapsed().as_secs_f64();

        if counter % CONSOLE_PRINT_THRESHOLD == 0 {
            println!(
                "[{:?}] {} ({:.2} addrs/s)",
                thread::current().id(),
                counter.separate_with_commas(),
                throughput
            );
        }
    }

    println!("[{:?}] btc puzzle solver finished.", thread::current().id());
}