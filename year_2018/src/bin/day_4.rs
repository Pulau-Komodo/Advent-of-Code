use shared::{Range, SmallMap};

fn main() {
	shared::print_answers(4, &[get_answer_1, get_answer_2]);
}

fn get_answer_1(input: &str) -> u32 {
	let guards = get_guard_sleeps(input);

	let (id, sleeps) = guards
		.into_iter()
		.max_by_key(|(_id, sleep)| sleep.iter().map(Range::len).sum::<u32>())
		.unwrap();
	let id: u32 = id.parse().unwrap();
	let most_slept_minute = (0..60)
		.max_by_key(|minute| sleeps.iter().filter(|sleep| sleep.contains(minute)).count())
		.unwrap();

	id * most_slept_minute
}

fn get_answer_2(input: &str) -> u32 {
	let guards = get_guard_sleeps(input);

	let (id, most_slept_minute, _sleeps) = guards
		.into_iter()
		.map(|(id, sleep)| {
			let (minute, sleeps) = (0..60)
				.map(|minute| {
					(
						minute,
						sleep.iter().filter(|sleep| sleep.contains(&minute)).count(),
					)
				})
				.max_by_key(|(_minute, sleeps)| *sleeps)
				.unwrap();
			(id, minute, sleeps)
		})
		.max_by_key(|(_id, _minute, sleeps)| *sleeps)
		.unwrap();
	let id: u32 = id.parse().unwrap();

	id * most_slept_minute
}

fn get_guard_sleeps(input: &str) -> SmallMap<&str, Vec<Range<u32>>> {
	let mut log: Vec<_> = input.lines().collect();
	log.sort();
	let mut current_guard = "";
	let mut asleep_at = 0;
	let mut guards = SmallMap::new();
	for entry in log.into_iter().map(LogEntry::from_line) {
		match entry {
			LogEntry::BeginShift(guard) => current_guard = guard,
			LogEntry::StartSleep(minute) => asleep_at = minute,
			LogEntry::EndSleep(minute) => {
				guards
					.get_mut_or_insert(current_guard, Vec::new())
					.push(Range::new(asleep_at, minute));
			}
		}
	}
	guards
}

enum LogEntry<'l> {
	BeginShift(&'l str),
	StartSleep(u32),
	EndSleep(u32),
}

impl<'l> LogEntry<'l> {
	fn from_line(line: &'l str) -> Self {
		let (timestamp, event) = line.split_once("] ").unwrap();
		if let Some(guard) = event.strip_suffix(" begins shift") {
			let guard = &guard[7..];
			Self::BeginShift(guard)
		} else {
			let minute = timestamp[15..17].parse().unwrap();
			if event == "falls asleep" {
				Self::StartSleep(minute)
			} else {
				Self::EndSleep(minute)
			}
		}
	}
}
