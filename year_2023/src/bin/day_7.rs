use std::array;

fn main() {
	shared::print_answers(7, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let mut hands: Vec<_> = input.lines().map(Hand::from_line).collect();
	hands.sort();
	hands
		.into_iter()
		.enumerate()
		.map(|(index, hand)| (index + 1) as u32 * hand.bid)
		.sum()
}

fn get_answer_2(input: &str) -> u32 {
	let mut hands: Vec<_> = input.lines().map(Hand::from_line_part_2).collect();
	hands.sort();
	hands
		.into_iter()
		.enumerate()
		.map(|(index, hand)| (index + 1) as u32 * hand.bid)
		.sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

impl HandType {
	fn determine(cards: [u8; 5]) -> Self {
		match count_cards(cards) {
			(5, 1) => Self::HighCard,
			(4, 2) => Self::OnePair,
			(3, 2) => Self::TwoPair,
			(3, 3) => Self::ThreeOfAKind,
			(2, 3) => Self::FullHouse,
			(2, 4) => Self::FourOfAKind,
			(0 | 1, 5) => Self::FiveOfAKind,
			_ => unreachable!("All imaginable hands should be covered."),
		}
	}
}

/// Returns `(unique_count, largest_count)`.
fn count_cards(cards: [u8; 5]) -> (usize, u32) {
	let mut counts = [(0, 0); 5];
	let mut unique_count = 0;
	let mut joker_count = 0;
	'card: for card in cards {
		if card == 1 {
			joker_count += 1;
			continue;
		}
		for (counted_card, card_count) in counts.iter_mut() {
			if *counted_card == card {
				*card_count += 1;
				continue 'card;
			}
		}
		counts[unique_count] = (card, 1);
		unique_count += 1;
	}
	let largest_count = counts
		.into_iter()
		.max_by_key(|(_card, count)| *count)
		.unwrap()
		.1;
	(unique_count, largest_count + joker_count)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
	hand_type: HandType,
	cards: [u8; 5],
	bid: u32,
}

impl Hand {
	fn from_line(line: &str) -> Self {
		let (cards, bid) = line.split_once(' ').unwrap();
		let cards = array::from_fn(|n| evaluate_card(cards.as_bytes()[n]));
		Self {
			hand_type: HandType::determine(cards),
			cards,
			bid: bid.parse().unwrap(),
		}
	}
	fn from_line_part_2(line: &str) -> Self {
		let modify_joker = |card: u8| -> u8 {
			if card == 11 {
				1
			} else {
				card
			}
		};

		let (cards, bid) = line.split_once(' ').unwrap();
		let cards = array::from_fn(|n| evaluate_card(cards.as_bytes()[n])).map(modify_joker);
		Self {
			hand_type: HandType::determine(cards),
			cards,
			bid: bid.parse().unwrap(),
		}
	}
}

fn evaluate_card(byte: u8) -> u8 {
	match byte {
		n @ b'2'..=b'9' => n - b'0',
		b'T' => 10,
		b'J' => 11,
		b'Q' => 12,
		b'K' => 13,
		b'A' => 14,
		_ => panic!(),
	}
}
