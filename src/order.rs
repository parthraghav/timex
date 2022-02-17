use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum OrderAction {
	Bid,
	Ask,
}

#[derive(Debug)]
pub enum OrderFamily {
	Market,
	Limit,
}

#[derive(Debug)]
pub enum OrderStatus {
	Open,
	Partial,
	Cancelled,
	Executed,
}

#[derive(Debug)]
pub enum OrderDuration {
	Day,
	GoodTillCancelled,
	ImmidiateOrCancel,
}

#[derive(Debug)]
pub struct Order {
	pub id: String,
	pub quantity: i32,
	pub price: f64,
	pub family: OrderFamily,
	pub action: OrderAction,
	pub status: OrderStatus,
	pub duration: OrderDuration,
	pub market_ticker: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}
