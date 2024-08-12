use countdown_numbers_rs::find_combination;
use std::process;
use text_io::read;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid input")]
struct InputError;

type UserInput = (Vec<u128>, u128);

fn take_user_input() -> Result<UserInput, InputError> {
	print!("numbers: ");
	let numbers_input: String = read!("{}\n");
	let numbers = numbers_input
		.split_whitespace()
		.map(|s| s.parse::<u128>())
		.collect::<Result<Vec<u128>, _>>()
		.map_err(|_| InputError)?;

	print!("target: ");
	let target_input: String = read!();
	let target = target_input.parse::<u128>().map_err(|_| InputError)?;

	Ok((numbers, target))
}

fn main() {
	let (numbers, target) = take_user_input().unwrap_or_else(|error| {
		println!("{}", error);
		process::exit(1);
	});

	let result = find_combination(&numbers, &target);
	println!("{} == {}", result.state, result.value);
	println!("{} off", target.abs_diff(result.value));
}
