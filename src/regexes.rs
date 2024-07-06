use once_cell::sync::Lazy;
use regex::Regex;

pub static ALL_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new(r"(\p{Alphabetic}|\d)(\p{Alphabetic}|\d|'|-)*").unwrap());

pub static ALPHANUMERIC_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new(r"(\p{Alphabetic}|\d)+").unwrap());

pub static ALPHA_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\p{Alphabetic})+").unwrap());

pub static NUMERIC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d)+").unwrap());

pub static NODASH_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new(r"(\p{Alphabetic}|\d)(\p{Alphabetic}|\d|')*").unwrap());

pub static NOAPOSTROPHE_REGEX: Lazy<Regex> =
	Lazy::new(|| Regex::new(r"(\p{Alphabetic}|\d)(\p{Alphabetic}|\d|-)*").unwrap());

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

	#[test]
	fn alphanumeric1() {
		let rres: Vec<_> = ALPHANUMERIC_REGEX
			.find_iter("lorem ipsum dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dolor"]);
	}

	#[test]
	fn alphanumeric2() {
		let rres: Vec<_> = ALPHANUMERIC_REGEX
			.find_iter("lor.em ips!um 'dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lor", "em", "ips", "um", "dolor"]);
	}

	#[test]
	fn alphanumeric3() {
		let rres: Vec<_> = ALPHANUMERIC_REGEX
			.find_iter("lorem ipsum dol_3or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dol", "3or"]);
	}

	#[test]
	fn alphanumeric4() {
		let rres: Vec<_> = ALPHANUMERIC_REGEX
			.find_iter("123  1,23 1_2 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["123", "1", "23", "1", "2", "2d3"]);
	}

	#[test]
	fn alphanumeric5() {
		let rres: Vec<_> = ALPHANUMERIC_REGEX
			.find_iter("ömür ğğğ 式 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["ömür", "ğğğ", "式", "2d3"]);
	}

	#[test]
	fn alphanumeric6() {
		let rres: Vec<_> = ALPHANUMERIC_REGEX
			.find_iter("lorem ip-sum dol3'or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ip", "sum", "dol3", "or"]);
	}

	#[test]
	fn alpha1() {
		let rres: Vec<_> = ALPHA_REGEX
			.find_iter("lorem ipsum dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dolor"]);
	}

	#[test]
	fn alpha2() {
		let rres: Vec<_> = ALPHA_REGEX
			.find_iter("lor.em ips!um 'dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lor", "em", "ips", "um", "dolor"]);
	}

	#[test]
	fn alpha3() {
		let rres: Vec<_> = ALPHA_REGEX
			.find_iter("lorem ipsum dol_3or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dol", "or"]);
	}

	#[test]
	fn alpha4() {
		let rres: Vec<_> = ALPHA_REGEX
			.find_iter("123  1,23 1_2 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["d"]);
	}

	#[test]
	fn alpha5() {
		let rres: Vec<_> = ALPHA_REGEX
			.find_iter("ömür ğğğ 式 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["ömür", "ğğğ", "式", "d"]);
	}

	#[test]
	fn alpha6() {
		let rres: Vec<_> = ALPHA_REGEX
			.find_iter("lorem ip-sum dol3'or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ip", "sum", "dol", "or"]);
	}

	#[test]
	fn numeric1() {
		let rres: Vec<_> = NUMERIC_REGEX
			.find_iter("lorem ipsum dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, Vec::<&str>::new());
	}

	#[test]
	fn numeric2() {
		let rres: Vec<_> = NUMERIC_REGEX
			.find_iter("lor.em ips!um 'dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, Vec::<&str>::new());
	}

	#[test]
	fn numeric3() {
		let rres: Vec<_> = NUMERIC_REGEX
			.find_iter("lorem ipsum dol_3or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["3"]);
	}

	#[test]
	fn numeric4() {
		let rres: Vec<_> = NUMERIC_REGEX
			.find_iter("123  1,23 1_2 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["123", "1", "23", "1", "2", "2", "3"]);
	}

	#[test]
	fn numeric5() {
		let rres: Vec<_> = NUMERIC_REGEX
			.find_iter("ömür ğğğ 式 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["2", "3"]);
	}

	#[test]
	fn numeric6() {
		let rres: Vec<_> = NUMERIC_REGEX
			.find_iter("lorem ip-sum dol3'or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["3"]);
	}

	#[test]
	fn no_dash1() {
		let rres: Vec<_> = NODASH_REGEX
			.find_iter("lorem ipsum dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dolor"]);
	}

	#[test]
	fn no_dash2() {
		let rres: Vec<_> = NODASH_REGEX
			.find_iter("lor.em ips!um 'dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lor", "em", "ips", "um", "dolor"]);
	}

	#[test]
	fn no_dash3() {
		let rres: Vec<_> = NODASH_REGEX
			.find_iter("lorem ipsum dol_3or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dol", "3or"]);
	}

	#[test]
	fn no_dash4() {
		let rres: Vec<_> = NODASH_REGEX
			.find_iter("123  1,23 1_2 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["123", "1", "23", "1", "2", "2d3"]);
	}

	#[test]
	fn no_dash5() {
		let rres: Vec<_> = NODASH_REGEX
			.find_iter("ömür ğğğ 式 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["ömür", "ğğğ", "式", "2d3"]);
	}

	#[test]
	fn no_dash6() {
		let rres: Vec<_> = NODASH_REGEX
			.find_iter("lorem ip-sum dol3'or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ip", "sum", "dol3'or"]);
	}

	#[test]
	fn no_apostrophe1() {
		let rres: Vec<_> = NOAPOSTROPHE_REGEX
			.find_iter("lorem ipsum dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dolor"]);
	}

	#[test]
	fn no_apostrophe2() {
		let rres: Vec<_> = NOAPOSTROPHE_REGEX
			.find_iter("lor.em ips!um 'dolor")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lor", "em", "ips", "um", "dolor"]);
	}

	#[test]
	fn no_apostrophe3() {
		let rres: Vec<_> = NOAPOSTROPHE_REGEX
			.find_iter("lorem ipsum dol_3or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ipsum", "dol", "3or"]);
	}

	#[test]
	fn no_apostrophe4() {
		let rres: Vec<_> = NOAPOSTROPHE_REGEX
			.find_iter("123  1,23 1_2 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["123", "1", "23", "1", "2", "2d3"]);
	}

	#[test]
	fn no_apostrophe5() {
		let rres: Vec<_> = NOAPOSTROPHE_REGEX
			.find_iter("ömür ğğğ 式 2d3")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["ömür", "ğğğ", "式", "2d3"]);
	}

	#[test]
	fn no_apostrophe6() {
		let rres: Vec<_> = NOAPOSTROPHE_REGEX
			.find_iter("lorem ip-sum dol3'or")
			.map(|m| m.as_str())
			.collect();

		assert_eq!(rres, vec!["lorem", "ip-sum", "dol3", "or"]);
	}
}
