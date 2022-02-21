use chrono;

mod comparators;
mod order;
mod order_book;
mod trade;

use order::*;
use order_book::*;

fn main() {
    let mut order_book = OrderBook::new(String::from("DUMMY"), 100000);

    order_book.add_order(order::Order {
        id: String::from("0x0"),
        quantity: 1,
        price: 100.001,
        family: OrderFamily::Limit,
        action: OrderAction::Buy,
        status: OrderStatus::Open,
        duration: OrderDuration::Day,
        market_ticker: String::from("dummy_token"),
        created_at: chrono::offset::Utc::now(),
        updated_at: chrono::offset::Utc::now(),
    });

    order_book.match_orders();
}
