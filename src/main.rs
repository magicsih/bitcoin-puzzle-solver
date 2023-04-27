extern crate bitcoin;
extern crate bitcoin_hashes;
extern crate rand;
extern crate reqwest;
extern crate secp256k1;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use rand::Rng;
use secp256k1::Secp256k1;
use std::env;
use std::thread;
use std::time::Instant;

const POWER: u32 = 65;
const TARGET: &str = "13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so";

const CONSOLE_PRINT_THRESHOLD: u128 = 10_000_000;
const MESSAGER_PRINTOUT_THRESHOLD: u128 = 1_000_000_000;

fn send_message_to_telegram(message: &str) {
    let telegram_setup = env::var("TELEGRAM_TOKEN").is_ok() && env::var("TELEGRAM_CHAT_ID").is_ok();
    if telegram_setup {
        let telegram_token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN is not set");
        let telegram_chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID is not set");

        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}",
            telegram_token, telegram_chat_id, message
        );
        let _ = client.get(&url).send();
    } else {
        //print out message to console
        println!("{}", message);
    }
}

fn random_lookfor(begin: u128, end: u128) {
    println!(
        "[{:?}] B-Puzzle Random Solver Started. {} to {}",
        thread::current().id(),
        begin,
        end
    );
    let start = Instant::now();
    let secp = Secp256k1::new();

    let mut counter: u128 = 0;
    let mut rng = rand::thread_rng();

    loop {
        let value = rng.gen_range(begin..end);
        let private_key = format!("{:0>64x}", value);
        let private_key_bytes = hex::decode(private_key.clone()).unwrap();
        let private_key = PrivateKey {
            compressed: true,
            network: Network::Bitcoin,
            key: secp256k1::SecretKey::from_slice(&private_key_bytes).unwrap(),
        };
        let public_key = private_key.public_key(&secp);
        let address = Address::p2pkh(&public_key, Network::Bitcoin).to_string();

        // print address and p2pkh(private_key)
        // println!("address:{} key:{}", address, private_key.to_wif());

        if address == TARGET {
            send_message_to_telegram(&format!(
                "[Lucky] PrivateKey:{} Address:{}",
                private_key, address
            ));
            println!(
                "[Lucky] Value: {} Private Key: {} Address: {}",
                value, private_key, address
            );
            break;
        }

        counter += 1;

        if counter % CONSOLE_PRINT_THRESHOLD == 0 {
            let throughput = counter as f64 / start.elapsed().as_secs_f64();
            println!(
                "[{:?}] {}({:.2} addrs/s)",
                thread::current().id(),
                counter,
                throughput
            );
            if counter % MESSAGER_PRINTOUT_THRESHOLD == 0 {
                send_message_to_telegram(&format!(
                    "[{:?}] {}({:.2} addrs/s)",
                    thread::current().id(),
                    counter,
                    throughput
                ));
            }
        }
    }

    println!("B-Puzzle Solver Finished.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_threads: u32 = if args.len() < 2 {
        num_cpus::get() as u32 / 2
    } else {
        args[1].parse().expect("Failed to parse number of threads")
    };

    let begin = 2u128.pow(POWER);
    let end: u128 = 2u128.pow(POWER + 1) - 1;

    println!(
        "B-Puzzle Solver Main Process Started. THREADS:{} BEGIN: {} END: {}",
        num_threads, begin, end
    );
    send_message_to_telegram(&format!(
        "B-Puzzle Solver Main Process Started. THREADS:{} BEGIN: {} END: {}",
        num_threads, begin, end
    ));

    let mut handles = vec![];

    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            random_lookfor(begin, end);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
