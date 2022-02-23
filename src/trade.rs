use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Trade {
	pub bid_order_id: String,
	pub ask_order_id: String,
	pub trade_price: f64,
	pub trade_quantity: i32,
	pub created_at: DateTime<Utc>,
}
