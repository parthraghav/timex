use chrono;
use std::cmp::min;
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
	opening_price: f64,
	circulation: i32,
	ask_queue: BinaryHeap<Ask>,
	bid_queue: BinaryHeap<Bid>,
	trades: Vec<Trade>,
}

impl OrderBook {
	// Creates a new order book for a given ticker
	pub fn new(ticker: String, circulation: i32, opening_price: f64) -> Self {
		return Self {
			ticker: ticker,
			low: 0.0,
			high: 0.0,
			day_low: 0.0,
			day_high: 0.0,
			volume: 0,
			spread: 0.0,
			valuation: 0.0,
			last_traded_price: opening_price,
			opening_price: opening_price,
			circulation: circulation,
			ask_queue: BinaryHeap::new(),
			bid_queue: BinaryHeap::new(),
			trades: Vec::new(),
		};
	}

	pub fn add_order(&mut self, order: Order) {
		if matches!(order.action, OrderAction::Buy) {
			if self.bid_queue.len() == 0 && matches!(order.family, OrderFamily::Market) {
				println!(
					"Cannot add a market order to an illiquid book:\n{:?}",
					order
				);
			} else {
				self.bid_queue.push(Bid {
					order: Order::clone(&order),
				});
				println!("Buy order successfully added:\n{:?}", order);
			}
		} else if matches!(order.action, OrderAction::Sell) {
			if self.ask_queue.len() == 0 && matches!(order.family, OrderFamily::Market) {
				println!(
					"Cannot add a market order to an illiquid book:\n{:?}",
					order
				);
			} else {
				self.ask_queue.push(Ask {
					order: Order::clone(&order),
				});
				println!("Sell order successfully added:\n{:?}", order);
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

	pub fn settle(&mut self) {
		loop {
			if self.bid_queue.len() == 0 || self.ask_queue.len() == 0 {
				break;
			}

			// Safe to us[[unwrap]] because we handle the None case before
			let ask = self.ask_queue.peek().unwrap();
			let bid = self.bid_queue.peek().unwrap();

			let mut trade_price: Option<f64> = None;

			// When both orders are limit orders
			if !matches!(ask.order.family, OrderFamily::Market)
				&& !matches!(bid.order.family, OrderFamily::Market)
			{
				if ask.order.price <= bid.order.price {
					trade_price = if ask.order.created_at < bid.order.created_at {
						Some(ask.order.price)
					} else {
						Some(bid.order.price)
					}
				} else {
					break;
				}
			}
			// When one is a limit order and other is a market order
			// trade price is set by the limit order
			else if matches!(ask.order.family, OrderFamily::Market)
				&& !matches!(bid.order.family, OrderFamily::Market)
			{
				trade_price = Some(bid.order.price);
			} else if !matches!(ask.order.family, OrderFamily::Market)
				&& matches!(bid.order.family, OrderFamily::Market)
			{
				trade_price = Some(ask.order.price);
			}
			// When both are market orders, the opening price is the trade price
			else {
				trade_price = Some(self.opening_price);
			}

			if let Some(traded_price) = trade_price {
				let traded_quantity = min(ask.order.quantity, bid.order.quantity);

				let trade = Trade {
					bid_order_id: bid.order.id.clone(),
					ask_order_id: ask.order.id.clone(),
					trade_price: traded_price,
					trade_quantity: traded_quantity,
					created_at: chrono::offset::Utc::now(),
				};

				// Commit trade
				self.trades.push(trade);
				self.last_traded_price = traded_price;

				// Alter ask
				if traded_quantity < ask.order.quantity {
					if let Some(mut ask) = self.ask_queue.peek_mut() {
						(*ask).order.quantity -= traded_quantity;
						(*ask).order.status = OrderStatus::Partial;
					}
				} else {
					self.ask_queue.pop();
				}
				// Alter bid
				if traded_quantity < bid.order.quantity {
					if let Some(mut bid) = self.bid_queue.peek_mut() {
						(*bid).order.quantity -= traded_quantity;
						(*bid).order.status = OrderStatus::Partial;
					}
				} else {
					self.bid_queue.pop();
				}
			} else {
				println!("Trade could not be completed.");
			}
		}
	}
}
