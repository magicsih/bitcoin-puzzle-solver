# Bitcoin Puzzle Solver

Bitcoin Puzzle Solver is a Rust application that attempts to solve the Bitcoin puzzle by generating and checking a range of private keys. It utilizes multiple threads to enhance performance and uses Telegram for real-time notifications.

## Motivation
- [Original transaction](https://www.blockchain.com/explorer/transactions/btc/08389f34c98c606322740c0be6a7125d9860bb8d5cb182c02f98461e5fa6cd15)
- [Bitcoin puzzle transaction ~32 BTC prize to who solves it](https://bitcointalk.org/index.php?topic=1306983)
- [Bitcoin challenge discusion](https://bitcointalk.org/index.php?topic=5166284)
- [== Bitcoin challenge transaction: ~1000 BTC total bounty to solvers! ==UPDATED==](https://bitcointalk.org/index.php?topic=5218972)

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Building

1. Clone the repository:

```bash
git clone https://github.com/magicsih/bitcoin-puzzle-solver
cd bitcoin-puzzle-solver
```

2. Build the project:

```bash
cargo build --release
```

## Running

You can then run the application as follows:

```bash
cargo run
```

```bash
Puzzle solver main process started.
concurrency:5
search space:2^66
36,893,488,147,419,103,232~73,786,976,294,838,206,464
target:13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so
ThreadId(6) starts searching from 65,913,684,784,163,022,770
ThreadId(3) starts searching from 42,087,244,010,988,039,528
ThreadId(2) starts searching from 54,703,510,351,966,310,198
ThreadId(5) starts searching from 52,564,880,187,736,458,273
ThreadId(4) starts searching from 72,820,389,139,615,756,636
```

```bash
Alternatively, you can directly run the built binary:

```bash
./target/release/bitcoin-puzzle-solver
```

## Configuration

You can configure the number of threads used by the application by passing it as a command line argument. If not specified, the application will use half of the available CPU cores.

Example:

```bash
cargo run --release -- 4
```

This command will run the application with 4 threads.

You will need to configure the following constants in the program:

- `POWER`: This is the exponent to which 2 is raised to generate the start and end of the range of private keys. The range of private keys will be from 2^POWER to 2^(POWER+1)-1.

  For example, if POWER is 65, the range of private keys will be from 2^65 to 2^66-1.

- `TARGET`: This is the target Bitcoin address for which you want to find the private key.

  For example, if TARGET is "13zb1hQbWVsc2S7ZTZnP2G4undNNpdh5so", the program will try to find the private key for this address.

You can modify these constants in the source code before you build the program.

### Telegram Message Setup (Optional)

Before running the application, set your environment variables:

```bash
export TELEGRAM_TOKEN=your_telegram_token
export TELEGRAM_CHAT_ID=your_telegram_chat_id
```

## Disclaimer

This application is for educational purposes only. It is not recommended to use this application for real Bitcoin transactions. Use at your own risk.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

Contributions are welcome. Please submit a Pull Request or open an Issue on GitHub.
