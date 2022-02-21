use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
pub enum OrderAction {
	Buy,
	Sell,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderFamily {
	Market,
	Limit,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderStatus {
	Open,
	Partial,
	Cancelled,
	Executed,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderDuration {
	Day,
	GoodTillCancelled,
	ImmidiateOrCancel,
}

#[derive(Debug, Clone)]
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

impl PartialEq for Order {
	fn eq(&self, other: &Self) -> bool {
		((self.price - other.price) < f64::EPSILON) && self.created_at == other.created_at
	}
}
impl Eq for Order {}
