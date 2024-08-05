use itertools::iproduct;
use std::collections::HashSet;
use text_io::read;

#[derive(Clone)]
struct Expression {
    text: String,
    value: i128,
    contains: usize,
}

fn add(e1: &Expression, e2: &Expression) -> Option<Expression> {
    Some(Expression {
        text: format!("({}+{})", e1.text, e2.text),
        value: e1.value + e2.value,
        contains: e1.contains + e2.contains,
    })
}

fn multiply(e1: &Expression, e2: &Expression) -> Option<Expression> {
    if e1.value == 1 || e2.value == 1 {
        None
    } else {
        Some(Expression {
            text: format!("{}*{}", e1.text, e2.text),
            value: e1.value * e2.value,
            contains: e1.contains + e2.contains,
        })
    }
}

fn subtract(e1: &Expression, e2: &Expression) -> Option<Expression> {
    if e2.value >= e1.value {
        None
    } else {
        Some(Expression {
            text: format!("({}-{})", e1.text, e2.text),
            value: e1.value - e2.value,
            contains: e1.contains + e2.contains,
        })
    }
}

fn divide(e1: &Expression, e2: &Expression) -> Option<Expression> {
    if e2.value == 1 || e1.value % e2.value != 0 {
        None
    } else {
        Some(Expression {
            text: format!("{}/({})", e1.text, e2.text),
            value: e1.value / e2.value,
            contains: e1.contains + e2.contains,
        })
    }
}

fn shares_dependencies(e1: &Expression, e2: &Expression) -> bool {
    e1.contains & e2.contains != 0
}

fn generate_hash(e: &Expression, n_numbers: &usize) -> usize {
    ((e.value as usize) << n_numbers) + e.contains
}

fn generate_hashes(v: &Vec<Expression>, n_numbers: &usize) -> HashSet<usize> {
    v.iter().map(|e| generate_hash(&e, &n_numbers)).collect()
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
    let mut hashes = hashes.clone();
    for (e1, e2) in iproduct!(v1, v2) {
        if shares_dependencies(e1, e2) {
            continue;
        }
        for combination in combinations(e1, e2) {
            let combination_hash = generate_hash(&combination, n_numbers);
            if hashes.contains(&combination_hash) {
                continue;
            }
            result.push(combination);
            hashes.insert(combination_hash);
        }
    }
    (result, hashes)
}

fn generate_base_expressions(numbers: &Vec<i128>) -> Vec<Expression> {
    numbers
        .iter()
        .enumerate()
        .map(|(i, n)| Expression {
            text: format!("{}", n),
            value: *n,
            contains: 2_u32.pow(i as u32) as usize,
        })
        .collect()
}

fn generate_useful_expressions(base_expressions: &Vec<Expression>) -> Vec<Expression> {
    let mut hashes = generate_hashes(&base_expressions, &base_expressions.len());
    let mut expression_groups = vec![Vec::new(); base_expressions.len()];
    expression_groups[0].extend(base_expressions.clone());
    for i in 0..base_expressions.len() {
        for j in 0..i {
            let (perms, new_hashes) = permutations(
                &expression_groups[j as usize],
                &expression_groups[(i - j - 1) as usize],
                &hashes,
                &base_expressions.len(),
            );
            hashes = new_hashes;
            expression_groups[i as usize].extend(perms);
        }
    }
    expression_groups.into_iter().flatten().collect()
}

fn find_combination(numbers: &Vec<i128>, target: &i128) -> Expression {
    let base_expressions = generate_base_expressions(&numbers);
    let useful_expressions = generate_useful_expressions(&base_expressions);
    useful_expressions
        .into_iter()
        .min_by_key(|e| (e.value - target).abs())
        .unwrap()
}

fn main() {
    print!("numbers: ");
    let numbers_input: String = read!("{}\n");
    let numbers: Vec<i128> = numbers_input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string().parse().unwrap())
        .collect();

    print!("target: ");
    let target: i128 = read!();

    let result = find_combination(&numbers, &target);
    println!("{} == {}", result.text, result.value);
}
