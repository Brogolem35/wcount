use ustr::Ustr;

use crate::count::*;

pub enum ResultItem {
	Total(TotalCount),
	Stream(StreamWordCount),
}

impl ResultItem {
	pub fn label(&self) -> String {
		match self {
			Self::Total(_) => String::from("total_column"),
			Self::Stream(s) => s.label(),
		}
	}

        pub fn count(&self, s: &Ustr) -> usize {
                match self {
			Self::Total(total) => total.counts.get(s).unwrap_or(&0).clone(),
			Self::Stream(stream) => stream.counts.get(s).unwrap_or(&0).clone(),
		}
        }
}
