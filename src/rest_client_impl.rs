extern crate serde;
use serde::{Deserialize};

extern crate reqwest;
use reqwest::blocking::Client;

use std::time::{Instant};


#[derive(Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
    pub latency: Option<u128> // Add another optional field
}


#[derive(Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice
}


// https://developers.coinbase.com/api/v2#data-endpoints

// Add an implementation for the print function
impl CoinPrice {
    fn print_coin_price(self) {
        println!("API Responded in {:?} ms. SPOT: {base}-{currency}: {amount}",
            self.latency,
            base=self.base,
            currency=self.currency,
            amount=self.amount);
    }
}


pub fn get_coin_prices(currency: &String) {
    let spot_url = format!("https://api.coinbase.com/v2/prices/{currency}-{rates}/spot",
        currency = currency,
        rates = "USD");

    let client = Client::new();

    let start = Instant::now();

    let resp_spot_price = client.get(&spot_url)
        .send();
    
    let duration = start.elapsed();

    match resp_spot_price {
        Ok(parsed_spot_price) => {
            
            let coinprice = parsed_spot_price.json::<CoinbasePrice>().unwrap();
            
            let spot_price = CoinPrice {
                base: coinprice.data.base,
                currency: coinprice.data.currency,
                amount: coinprice.data.amount,
                latency: Some(duration.as_millis()),
            };

            spot_price.print_coin_price();

        }
        Err(e) => println!("Err: {:?}", e),
    }    
}