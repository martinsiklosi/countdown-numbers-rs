use clap::Parser;
use countdown::find_combination;

#[derive(Parser, Debug)]
struct Args {
	#[arg(short, long, required = true, num_args = 1..)]
	numbers: Vec<u128>,

	#[arg(short, long, required = true)]
	target: u128,
}

fn main() {
	let args = Args::parse();

	let result = find_combination(&args.numbers, &args.target);
	println!("{} == {}", result.state, result.value);
}
