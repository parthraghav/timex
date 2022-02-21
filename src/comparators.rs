use std::cmp::Ordering;

use crate::order::*;

// Ask
#[derive(Eq)]
pub struct Ask {
	pub order: Order,
}

// https://stackoverflow.com/questions/69665188/min-max-of-vecf64-trait-ord-is-not-implemented-for-xy
impl Ord for Ask {
	fn cmp(&self, other: &Self) -> Ordering {
		self.order
			.created_at
			.cmp(&other.order.created_at)
			.then_with(|| other.order.price.partial_cmp(&self.order.price).unwrap())
	}
}

impl PartialOrd for Ask {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Ask {
	fn eq(&self, other: &Self) -> bool {
		((self.order.price - other.order.price) < f64::EPSILON)
			&& self.order.created_at == other.order.created_at
	}
}

// Bid

#[derive(Eq)]
pub struct Bid {
	pub order: Order,
}

impl Ord for Bid {
	fn cmp(&self, other: &Self) -> Ordering {
		self.order
			.created_at
			.cmp(&other.order.created_at)
			.then_with(|| self.order.price.partial_cmp(&other.order.price).unwrap())
	}
}

impl PartialOrd for Bid {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Bid {
	fn eq(&self, other: &Self) -> bool {
		((self.order.price - other.order.price) < f64::EPSILON)
			&& self.order.created_at == other.order.created_at
	}
}
