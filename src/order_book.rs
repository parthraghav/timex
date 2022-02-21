use std::collections::BinaryHeap;

use crate::comparators::{Ask, Bid};
use crate::order::*;
use crate::trade::Trade;

pub struct OrderBook {
	ticker: String,
	low: f64,
	high: f64,
	day_low: f64,
	day_high: f64,
	volume: i32,
	spread: f64,
	valuation: f64,
	last_traded_price: f64,
	circulation: i32,
	ask_queue: BinaryHeap<Ask>,
	bid_queue: BinaryHeap<Bid>,
	trades: Vec<Trade>,
}

impl OrderBook {
	// Creates a new order book for a given ticker
	pub fn new(ticker: String, circulation: i32) -> Self {
		return Self {
			ticker: ticker,
			low: 0.0,
			high: 0.0,
			day_low: 0.0,
			day_high: 0.0,
			volume: 0,
			spread: 0.0,
			valuation: 0.0,
			last_traded_price: 0.0,
			circulation: circulation,
			ask_queue: BinaryHeap::new(),
			bid_queue: BinaryHeap::new(),
			trades: Vec::new(),
		};
	}

	pub fn add_order(&mut self, order: Order) {
		if matches!(order.action, OrderAction::Buy) {
			if self.bid_queue.len() != 0 && !matches!(order.family, OrderFamily::Market) {
				self.bid_queue.push(Bid {
					order: Order::clone(&order),
				});
				println!("Buy order successfully added:\n{:?}", order);
			} else {
				println!(
					"Cannot add a market order to an illiquid book:\n{:?}",
					order
				);
			}
		} else if matches!(order.action, OrderAction::Sell) {
			if self.ask_queue.len() != 0 && !matches!(order.family, OrderFamily::Market) {
				self.ask_queue.push(Ask {
					order: Order::clone(&order),
				});
				println!("Sell order successfully added:\n{:?}", order);
			} else {
				println!(
					"Cannot add a market order to an illiquid book:\n{:?}",
					order
				);
			}
		}
	}

	pub fn print_bids(&mut self) {
		while let Some(bid) = self.bid_queue.pop() {
			println!("{:?}", bid.order);
		}
	}

	pub fn print_asks(&mut self) {
		while let Some(ask) = self.ask_queue.pop() {
			println!("{:?}", ask.order);
		}
	}
}
