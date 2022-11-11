extern crate serde;
use serde::{Serialize, Deserialize};

extern crate reqwest;
use reqwest::blocking::Client;
use std::time::{Instant};

#[derive(Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Deserialize, Debug)]
pub struct CoinTime {
    pub iso: String,
    pub epoch: i64,    
}

#[derive(Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice
}

#[derive(Deserialize, Debug)]
pub struct CoinbaseTime {
    pub data: CoinTime
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormattedCoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
    pub latency: u128,
}

pub trait Price {
    fn format_coin_amount(&self) -> String;
    fn print_response_time(&self) -> String;
    fn print_coin_price(&self) -> String;
}

impl Price for FormattedCoinPrice {
    fn format_coin_amount(&self) -> String {
        return format!("{amount}",
            amount=self.amount);
    }
    fn print_response_time(&self) -> String {
        return format!("API Responded in {latency} ms",
            latency=self.latency);
    }
    fn print_coin_price(&self) -> String {
        return format!("{base}-{currency}: {amount}",
            base=self.base,
            currency=self.currency,
            amount=self.amount);
    }
}

fn get_coin_time() -> String {    
    let client = Client::new();
    let resp_coin_time = client.get("https://api.coinbase.com/v2/time")
        .send();

    match resp_coin_time {
        Ok(parsed_coin_time) => {
            let coinbasetime = parsed_coin_time.json::<CoinbaseTime>()
                .unwrap();
            let cointime = CoinTime {
                iso: coinbasetime.data.iso,
                epoch: coinbasetime.data.epoch                
            };
            return cointime.iso;
        }
        Err(e) => println!("Err: {:?}", e),
    }
    return "".to_string();
}

fn get_coin_price(request_type: String, request_currency: String, request_rates: String) -> String {
    let request_url = format!("https://api.coinbase.com/v2/prices/{currency}-{rates}/{type}",
        currency = request_currency,
        rates = request_rates,
        type = request_type);

    let client = Client::new();

    let start = Instant::now();

    let resp_price = client.get(&request_url)
        .send();

    let duration = start.elapsed();

    match resp_price {
        Ok(parsed_price) => {
            let coinbaseprice = parsed_price.json::<CoinbasePrice>()
                .unwrap();            
            let price = FormattedCoinPrice {
                base: coinbaseprice.data.base,
                currency: coinbaseprice.data.currency,
                amount: coinbaseprice.data.amount,
                latency: duration.as_millis(),
            };
            println!("{}. {} : {}", price.print_response_time(), request_type, price.print_coin_price());
            return price.format_coin_amount();
        }        
        Err(e) => println!("Err: {:?}", e),
    }    
    return "".to_string();
}

pub fn get_price_spread(currency: &String){
    let quote_time = get_coin_time();
    let spot_price = get_coin_price("spot".to_string(), currency.to_string(), "USD".to_string());
    let buy_price = get_coin_price("buy".to_string(), currency.to_string(), "USD".to_string());
    let sell_price = get_coin_price("sell".to_string(), currency.to_string(), "USD".to_string());
    let spread_price: f32 = (buy_price.parse::<f32>().unwrap()) - (sell_price.parse::<f32>().unwrap());
    
    println!("{}: BTC-USD SPOT Price: {} | BUY Price: {} | SELL Price: {} | Price Spread: {}", 
            quote_time, spot_price, buy_price, sell_price, spread_price.to_string());    
}
