extern crate serde;
use serde::{Deserialize, Serialize};

extern crate reqwest;
use reqwest::blocking::Client;


#[derive(Deserialize, Debug, Clone)]
pub struct Currency {
    pub id: String,
    pub name: String,
    pub min_size: String,
}

#[derive(Deserialize, Debug)]
pub struct CoinbaseCurrency {
    pub data: Vec<Currency>
}

#[derive(Serialize, Debug)]
pub struct ListAllCurrency {
    pub data: Vec<String>
}

// https://developers.coinbase.com/api/v2#data-endpoints


pub fn get_all_currencies() -> ListAllCurrency {
    let url = "https://api.coinbase.com/v2/currencies";

    let client = Client::new();

    let resp_spot_price = client.get(url)
        .send();
    
    let mut list_currencies: Vec<String> = vec![];

    // https://docs.rs/reqwest/latest/reqwest/struct.Response.html
    match resp_spot_price {
        Ok(parsed_spot_price) => {
            // Example of Unwrap
            let coinbase_currency = parsed_spot_price.json::<CoinbaseCurrency>().unwrap();
            for currency in coinbase_currency.data.iter(){
                println!("{:?}", currency);
                // println!("{:#?}", currency);
                list_currencies.push(currency.clone().id);
            }
        }
        Err(e) => println!("Err: {:?}", e),
    }

    // Example of Wrap
    ListAllCurrency {
        data: list_currencies
    }

}
