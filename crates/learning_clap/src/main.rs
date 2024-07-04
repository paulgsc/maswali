use clap::Parser;
use learning_clap::config::Config;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	/// Name of the person to greet
	#[arg(short, long)]
	name: String,

	/// Number of times to greet
	#[arg(short, long, default_value_t)]
	count: u8,
}

fn main() {
	let args = Args::parse();

	// example of config in real world application
	let config = Config::parse();

	for _ in 0..args.count {
		println!("Hello {}!", args.name);
	}
}
