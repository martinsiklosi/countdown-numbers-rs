use itertools::iproduct;
use std::{collections::HashSet, fmt, rc::Rc};

#[derive(Clone)]
pub enum State {
	Add { o1: Rc<State>, o2: Rc<State> },
	Multiply { o1: Rc<State>, o2: Rc<State> },
	Subtract { o1: Rc<State>, o2: Rc<State> },
	Divide { o1: Rc<State>, o2: Rc<State> },
	Base { value: u128 },
}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Add { o1, o2 } => write!(f, "({}+{})", o1, o2),
			Self::Multiply { o1, o2 } => write!(f, "{}*{}", o1, o2),
			Self::Subtract { o1, o2 } => write!(f, "({}-{})", o1, o2),
			Self::Divide { o1, o2 } => write!(f, "{}/({})", o1, o2),
			Self::Base { value } => write!(f, "{}", value),
		}
	}
}

#[derive(Clone)]
pub struct Expression {
	pub state: Rc<State>,
	pub value: u128,
	contains: usize,
}

fn add(e1: &Expression, e2: &Expression) -> Option<Expression> {
	Some(Expression {
		state: Rc::new(State::Add {
			o1: Rc::clone(&e1.state),
			o2: Rc::clone(&e2.state),
		}),
		value: e1.value + e2.value,
		contains: e1.contains | e2.contains,
	})
}

fn multiply(e1: &Expression, e2: &Expression) -> Option<Expression> {
	if e1.value == 0 || e1.value == 1 || e2.value == 0 || e2.value == 1 {
		return None;
	}
	Some(Expression {
		state: Rc::new(State::Multiply {
			o1: Rc::clone(&e1.state),
			o2: Rc::clone(&e2.state),
		}),
		value: e1.value * e2.value,
		contains: e1.contains | e2.contains,
	})
}

fn subtract(e1: &Expression, e2: &Expression) -> Option<Expression> {
	if e2.value > e1.value {
		return None;
	}
	Some(Expression {
		state: Rc::new(State::Subtract {
			o1: Rc::clone(&e1.state),
			o2: Rc::clone(&e2.state),
		}),
		value: e1.value - e2.value,
		contains: e1.contains | e2.contains,
	})
}

fn divide(e1: &Expression, e2: &Expression) -> Option<Expression> {
	if e2.value == 0 || e2.value == 1 || e1.value % e2.value != 0 {
		return None;
	}
	Some(Expression {
		state: Rc::new(State::Divide {
			o1: Rc::clone(&e1.state),
			o2: Rc::clone(&e2.state),
		}),
		value: e1.value / e2.value,
		contains: e1.contains | e2.contains,
	})
}

fn shares_dependencies(e1: &Expression, e2: &Expression) -> bool {
	e1.contains & e2.contains != 0
}

fn generate_hash(e: &Expression, n_numbers: &usize) -> usize {
	((e.value as usize) << n_numbers) + e.contains
}

fn generate_hashes(v: &[Expression], n_numbers: &usize) -> HashSet<usize> {
	v.iter().map(|e| generate_hash(e, n_numbers)).collect()
}

fn combinations(e1: &Expression, e2: &Expression) -> Vec<Expression> {
	[add, multiply, subtract, divide]
		.iter()
		.filter_map(|&operation| operation(e1, e2))
		.collect()
}

fn permutations(
	v1: &Vec<Expression>,
	v2: &Vec<Expression>,
	hashes: &HashSet<usize>,
	n_numbers: &usize,
) -> (Vec<Expression>, HashSet<usize>) {
	let mut result = Vec::new();
	let mut hashes = hashes.to_owned();
	for (e1, e2) in iproduct!(v1, v2) {
		if shares_dependencies(e1, e2) {
			continue;
		}
		for combination in combinations(e1, e2) {
			let combination_hash = generate_hash(&combination, n_numbers);
			if hashes.insert(combination_hash) {
				result.push(combination);
			}
		}
	}
	(result, hashes)
}

fn generate_base(numbers: &[u128]) -> Vec<Expression> {
	numbers
		.iter()
		.enumerate()
		.map(|(i, n)| Expression {
			state: Rc::new(State::Base { value: *n }),
			value: *n,
			contains: 2_u32.pow(i as u32) as usize,
		})
		.collect()
}

fn generate_useful(base: &[Expression]) -> Vec<Expression> {
	let mut hashes = generate_hashes(base, &base.len());
	let mut groups = vec![Vec::new(); base.len()];
	groups[0].extend(base.to_owned());
	for i in 0..base.len() {
		for j in 0..i {
			let (perms, new_hashes) =
				permutations(&groups[j], &groups[i - j - 1], &hashes, &base.len());
			hashes = new_hashes;
			groups[i].extend(perms);
		}
	}
	groups.into_iter().flatten().collect()
}

pub fn find_combination(numbers: &[u128], target: &u128) -> Expression {
	let base = generate_base(numbers);
	let useful = generate_useful(&base);
	useful
		.into_iter()
		.min_by_key(|e| target.abs_diff(e.value))
		.unwrap()
}
