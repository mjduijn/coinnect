//! Types definition used for handling returned data when generic API is used.

use std::collections::HashMap;

use currency::Currency;
use pair::Pair;

pub type Amount = f64;
pub type Price = f64;
pub type Volume = f64;

pub type Balances = HashMap<Currency, Amount>;

#[derive(Debug)]
pub struct Ticker {
    /// UNIX timestamp in ms (when the response was received)
    pub timestamp: i64,
    /// The Pair corresponding to the Ticker returned (maybe useful later for asynchronous APIs)
    pub pair: Pair,
    /// Last trade price found in the history
    pub last_trade_price: Price,
    /// Lowest ask price found in Orderbook
    pub lowest_ask: Price,
    /// Highest bid price found in Orderbook
    pub highest_bid: Price,
    // Bittrex does not support Volume for ticker so volume could be None
    /// Last 24 hours volume (quote-volume)
    pub volume: Option<Volume>,
}

#[derive(Debug)]
pub struct Orderbook {
    /// UNIX timestamp in ms (when the response was received)
    pub timestamp: i64,
    /// The Pair corresponding to the Orderbook returned (maybe useful later for asynchronous APIs)
    pub pair: Pair,
    /// Vec containing the ask offers (by ascending price)
    pub asks: Vec<(Price, Volume)>,
    /// Vec containing the bid offers (by descending price)
    pub bids: Vec<(Price, Volume)>,
}

impl Orderbook {
    /// Convenient function that returns the average price from the orderbook
    /// Return None if Orderbook is empty
    /// `Average price = (lowest ask + highest bid)/2`
    pub fn avg_price(&self) -> Option<Price> {
        if self.asks.is_empty() || self.bids.is_empty() {
            return None;
        }
        Some((self.asks[0].0 + self.bids[0].0) / 2.0)
    }
}

#[derive(Debug)]
pub struct OrderInfo {
    /// UNIX timestamp in ms (when the response was received)
    pub timestamp: i64,
    /// This identifiers list is specific to the platform you use. You must store it somewhere if
    /// you want to modify/cancel the order later
    pub identifier: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum OrderType {
    BuyLimit,
    SellLimit,
    BuyMarket,
    SellMarket,
}
