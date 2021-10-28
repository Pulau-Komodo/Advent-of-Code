fn main() {
	year_2020::print_answers(18, &[get_answer_1, get_answer_2]);
}

#[derive(Debug, Clone)]
enum Token {
	Number(u64),
	Addition,
	Multiplication,
	OpeningParenthesis,
	ClosingParenthesis,
}

fn tokenize_expression(expression: &str) -> Vec<Token> {
	expression
		.chars()
		.rev()
		.filter_map(|char| match char {
			'+' => Some(Token::Addition),
			'*' => Some(Token::Multiplication),
			'(' => Some(Token::OpeningParenthesis),
			')' => Some(Token::ClosingParenthesis),
			' ' => None,
			x => Some(Token::Number(x.to_string().parse::<u64>().unwrap())),
		})
		.collect()
}

fn evaluate_expression(expression: &[Token]) -> u64 {
	let mut iter = expression.iter().enumerate();
	let first = match iter.next() {
		Some((_, Token::ClosingParenthesis)) => {
			let mut open_parentheses = 1;
			let end = loop {
				match iter.next() {
					Some((_, Token::ClosingParenthesis)) => open_parentheses += 1,
					Some((index, Token::OpeningParenthesis)) => {
						open_parentheses -= 1;
						if open_parentheses == 0 {
							break index;
						}
					}
					None => panic!("Could not find closing parenthesis"),
					_ => (),
				}
			};
			evaluate_expression(&expression[1..end])
		}
		Some((_, Token::Number(n))) => *n,
		_ => {
			eprintln!("Unexpected first token on {:?}", expression);
			panic!("Syntax error 1");
		}
	};
	if let Some((index, operator)) = iter.next() {
		let second = evaluate_expression(&expression[index + 1..]);
		match operator {
			Token::Addition => first + second,
			Token::Multiplication => first * second,
			_ => panic!("Syntax error 2"),
		}
	} else {
		first
	}
}

fn tokenize_expression_2(expression: &str) -> Vec<Token> {
	expression
		.chars()
		.filter_map(|char| match char {
			'+' => Some(Token::Addition),
			'*' => Some(Token::Multiplication),
			'(' => Some(Token::OpeningParenthesis),
			')' => Some(Token::ClosingParenthesis),
			' ' => None,
			x => Some(Token::Number(x.to_string().parse::<u64>().unwrap())),
		})
		.collect()
}

fn evaluate_expression_2(expression: Vec<Token>) -> Vec<Token> {
	if let Some(first_index) = expression
		.iter()
		.position(|token| matches!(token, Token::OpeningParenthesis))
	{
		let mut iter = expression[first_index + 1..].iter().enumerate();
		let mut open_parentheses = 1;
		let end = loop {
			match iter.next() {
				Some((_, Token::OpeningParenthesis)) => open_parentheses += 1,
				Some((index, Token::ClosingParenthesis)) => {
					open_parentheses -= 1;
					if open_parentheses == 0 {
						break first_index + index + 1;
					}
				}
				None => panic!("Could not find closing parenthesis"),
				_ => (),
			}
		};
		let mut parenthesized = evaluate_expression_2(expression[first_index + 1..end].to_vec());
		loop {
			if parenthesized.len() == 1 {
				break;
			} else {
				parenthesized = evaluate_expression_2(parenthesized);
			}
		}

		let mut output = expression.clone();
		output.splice(first_index..end + 1, parenthesized);
		output
	} else if let Some(addition) = expression
		.iter()
		.position(|token| matches!(token, Token::Addition))
	{
		if let (Some(Token::Number(n1)), Some(Token::Number(n2))) =
			(expression.get(addition - 1), expression.get(addition + 1))
		{
			let mut output = expression.clone();
			output.splice(addition - 1..addition + 2, vec![Token::Number(n1 + n2)]);
			output
		} else {
			panic!();
		}
	} else if let (Some(Token::Number(n1)), Some(Token::Number(n2))) =
		(expression.get(0), expression.get(2))
	{
		let mut output = expression.clone();
		output.splice(0..3, vec![Token::Number(n1 * n2)]);
		output
	} else {
		eprintln!("{:?}", expression);
		panic!();
	}
}

fn get_answer_1(input: &str) -> String {
	let result: u64 = input
		.lines()
		.map(tokenize_expression)
		.map(|expression| evaluate_expression(&expression))
		.sum();
	format!("{}", result)
}

fn get_answer_2(input: &str) -> String {
	let result: u64 = input
		.lines()
		.map(tokenize_expression_2)
		.map(|mut expression| loop {
			if expression.len() == 1 {
				if let Some(Token::Number(n)) = expression.get(0) {
					break *n;
				} else {
					panic!();
				}
			} else {
				expression = evaluate_expression_2(expression);
			}
		})
		.sum();
	format!("{}", result)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_input() {
		let expression = tokenize_expression("2 * 3 + (4 * 5)");
		assert_eq!(evaluate_expression(&expression), 26);
		let expression = tokenize_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)");
		assert_eq!(evaluate_expression(&expression), 437);
		let expression = tokenize_expression("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
		assert_eq!(evaluate_expression(&expression), 12240);
		let expression = tokenize_expression("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
		assert_eq!(evaluate_expression(&expression), 13632);
	}
	#[test]
	fn sample_input_2() {
		let expression = "1 + (2 * 3) + (4 * (5 + 6))";
		println!("{}", expression);
		assert_eq!(get_answer_2(expression), "51");
		let expression = "2 * 3 + (4 * 5)";
		println!("{}", expression);
		assert_eq!(get_answer_2(expression), "46");
		let expression = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
		println!("{}", expression);
		assert_eq!(get_answer_2(expression), "1445");
		let expression = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
		println!("{}", expression);
		assert_eq!(get_answer_2(expression), "669060");
		let expression = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
		println!("{}", expression);
		assert_eq!(get_answer_2(expression), "23340");
	}
}
