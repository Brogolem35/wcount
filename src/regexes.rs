use once_cell::sync::Lazy;
use regex::Regex;

pub static ALL_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new(r"(\p{Alphabetic}|\d)(\p{Alphabetic}|\d|'|-)*").unwrap());

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn all1() {
		let rres: Vec<_> = ALL_REGEX
			.find_iter("lorem ipsum dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dolor"]);
	}

	#[test]
	fn all2() {
		let rres: Vec<_> = ALL_REGEX
			.find_iter("lor.em ips!um 'dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lor", "em", "ips", "um", "dolor"]);
	}

	#[test]
	fn all3() {
		let rres: Vec<_> = ALL_REGEX
			.find_iter("lorem ipsum dol_3or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dol", "3or"]);
	}

	#[test]
	fn all4() {
		let rres: Vec<_> = ALL_REGEX
			.find_iter("123  1,23 1_2 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["123", "1", "23", "1", "2", "2d3"]);
	}

	#[test]
	fn all5() {
		let rres: Vec<_> = ALL_REGEX
			.find_iter("ömür ğğğ 式 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["ömür", "ğğğ", "式", "2d3"]);
	}
}
