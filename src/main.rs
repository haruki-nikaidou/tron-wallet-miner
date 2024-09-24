use std::sync::Arc;
use clap::Parser;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod address;
mod monitor;
mod hasher;
mod miner;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    password: String,

    #[arg(short, long)]
    worker: usize,

    #[arg(short, long)]
    require: String
}

fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}


fn main() {
    setup_tracing();
    let args = Args::parse();
    if args.password.len() < 32 {
        eprintln!("Password must be at least 16 characters long");
        std::process::exit(1);
    }
    let require = bs58::decode(args.require).into_vec().expect("Invalid require");
    let monitor = Arc::new(monitor::Monitor::new());
    for i in 0..args.worker {
        let monitor = monitor.clone();
        let require = require.clone();
        let password = args.password.clone();
        std::thread::spawn(move || {
            miner::create_miner(&password, i, &require, monitor);
        });
    }
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        monitor.print_reset();
        if monitor.is_found() {
            break;
        }
    }
}
