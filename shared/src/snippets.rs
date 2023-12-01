#[inline]
fn subset(a: u64, b: u64) -> bool {
	a & b == a
}

#[inline]
fn strict_subset(a: u64, b: u64) -> bool {
	a != b && subset(a, b)
}

#[test]
fn subset() {
	assert!(strict_subset(0b010, 0b011));
	assert!(!strict_subset(0b110, 0b011));
	assert!(!strict_subset(0b011, 0b011));
}