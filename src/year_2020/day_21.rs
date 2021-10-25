use std::collections::HashSet;

struct Product {
	ingredients: Vec<String>,
	allergens: Vec<String>,
}

struct ProductList {
	ingredients: HashSet<String>,
	allergens: HashSet<String>,
	products: Vec<Product>,
}

impl ProductList {
	fn from_str(str: &str) -> Self {
		let mut all_ingredients = HashSet::new();
		let mut all_allergens = HashSet::new();
		let products = str
			.lines()
			.map(|line| {
				let (ingredients_str, rest) = line.split_once(" (contains ").unwrap();
				let mut ingredients = Vec::new();
				for ingredient in ingredients_str.split(' ') {
					let ingredient = ingredient.to_string();
					all_ingredients.insert(ingredient.clone());
					ingredients.push(ingredient);
				}
				let mut allergens = Vec::new();
				for allergen in rest.strip_suffix(')').unwrap().split(", ") {
					let allergen = allergen.to_string();
					all_allergens.insert(allergen.clone());
					allergens.push(allergen);
				}
				Product {
					ingredients,
					allergens,
				}
			})
			.collect();
		ProductList {
			ingredients: all_ingredients,
			allergens: all_allergens,
			products,
		}
	}
	fn narrow_down_allergens(&self) -> std::collections::HashMap<&String, Vec<&String>> {
		self.ingredients
			.iter()
			.map(|ingredient| {
				let allergens = self
					.allergens
					.iter()
					.filter(|allergen| {
						!self
							.products
							.iter()
							.filter(|product| !product.ingredients.contains(ingredient))
							.any(|product| product.allergens.contains(allergen))
					})
					.collect();
				(ingredient, allergens)
			})
			.collect()
	}
}

pub fn get_answer_1(input: String) -> String {
	let product_list = ProductList::from_str(&input);
	let ingredients = product_list.narrow_down_allergens();
	let count: usize = ingredients
		.iter()
		.filter(|(_, allergens)| allergens.is_empty())
		.map(|(ingredient, _)| {
			product_list
				.products
				.iter()
				.filter(|product| product.ingredients.contains(ingredient))
				.count()
		})
		.sum();
	format!("{}", count)
}

pub fn get_answer_2(input: String) -> String {
	let product_list = ProductList::from_str(&input);
	let ingredients = product_list.narrow_down_allergens();
	let mut dangerous_ingredients: Vec<(_, _)> = ingredients
		.into_iter()
		.filter(|(_, allergens)| !allergens.is_empty())
		.collect();
	let mut found_ingredients: Vec<String> = Vec::new();
	let mut found_allergens: Vec<String> = Vec::new();
	loop {
		let known_index = dangerous_ingredients
			.iter()
			.position(|(_, allergens)| allergens.len() == 1)
			.unwrap();
		let (ingredient, allergens) = dangerous_ingredients.remove(known_index);
		found_ingredients.push(ingredient.to_string());
		let found_allergen = allergens.get(0).unwrap();
		found_allergens.push(found_allergen.to_string());
		for (_, allergens) in dangerous_ingredients.iter_mut() {
			if let Some(allergen_index) = allergens
				.iter()
				.position(|allergen| allergen == found_allergen)
			{
				allergens.remove(allergen_index);
			}
		}
		if dangerous_ingredients.is_empty() {
			break;
		}
	}
	let mut ingredients_allergens: Vec<(String, String)> = found_ingredients
		.into_iter()
		.zip(found_allergens.into_iter())
		.collect();
	ingredients_allergens
		.sort_by(|(_, allergen), (_, other_allergen)| allergen.cmp(other_allergen));
	ingredients_allergens
		.into_iter()
		.map(|(ingredient, _)| ingredient)
		.collect::<Vec<_>>()
		.join(",")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample_data_narrowing() {
		let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
		let product_list = ProductList::from_str(input);
		let allergens = product_list.narrow_down_allergens();
		println!("{:?}", allergens);
	}
	#[test]
	fn sample_data_part_1() {
		let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)".to_string();
		assert_eq!(get_answer_1(input), "5".to_string())
	}
	#[test]
	fn sample_data_part_2() {
		let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)".to_string();
		assert_eq!(get_answer_2(input), "mxmxvkd,sqjhc,fvjkl".to_string())
	}
}
