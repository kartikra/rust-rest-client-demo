
use std::{time::{Instant, Duration}, sync::{Arc, Mutex}};

use futures::{stream, StreamExt};
use rest_client_demo::*;
use serde::{Deserialize};
use reqwest_middleware::{ClientBuilder};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

#[derive(Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: Option<CoinPrice>
}

#[derive(Deserialize, Debug)]
pub struct ResponseCryptoPrice {
    pub data: Option<CoinPrice>,
    pub latency: u128,
    pub status: String,
    pub error: String
}

fn format_coin_price(coin_price:CoinPrice) -> String {
    format!("SPOT: {base}-{currency}: {amount}",
        base=coin_price.base,
        currency=coin_price.currency,
        amount=coin_price.amount)
}

// https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest/51047786#51047786
#[tokio::main]
pub async fn get_all_prices(list_currency: Vec<String>, list_crypto: Vec<&str>, from_currency: &str) {

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let reqwest_client = reqwest::Client::builder()
                                .timeout(Duration::from_secs(60)) // Overall timeout is 60 secs connection + unwrapping json
                                .connect_timeout(Duration::from_secs(40)) // // Specify connection timeout only
                                .build()
                                .expect("Unable to connect");
    
    let client = ClientBuilder::new(reqwest_client)
                                    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                                    .build();

    // const CONCURRENT_REQUESTS: usize = 50;

    let mut final_list = list_currency;
    final_list.append(&mut list_crypto.iter().map(|s| s.to_string()).collect());

    let overall_start = Instant::now();

    let resp_payloads = stream::iter(final_list)
        .map(|currency| {
            let client = &client;
            let url = if list_crypto.contains(&&*currency.to_owned()) {
                format!("https://api.coinbase.com/v2/prices/{from_currency}-{currency}/spot",currency=from_currency,from_currency=currency)
            } else  {
                format!("https://api.coinbase.com/v2/prices/{from_currency}-{currency}/spot",currency=currency,from_currency=from_currency)
            };
            
            async move {
            
                let start = Instant::now();
                let resp = client.get(&url).send().await;
                
                match resp {
                    Ok(parsed_spot_price) => {
                        
                        // Json String to Struct
                        let coinprice = parsed_spot_price.json::<CoinbasePrice>().await;
                        
                        match coinprice {
                            Ok(price) => {
                                let final_response = ResponseCryptoPrice {
                                    data: price.data,
                                    latency: start.elapsed().as_millis(),
                                    status: "SUCCESS".to_string(),
                                    error: "".to_string(),
                                };
                                final_response
                            }
                            Err(e) =>  {
                                println!("Result Error: {}", e);
                                let final_response = ResponseCryptoPrice {
                                    data: None,
                                    latency: start.elapsed().as_millis(),
                                    status: "FAILURE".to_string(),
                                    error: format!("Result Error: {}", e),
                                };
                                final_response
                            }
                        }
                    }
                    Err(e) =>  {
                        println!("Response Error: {}", e);
                        let final_response = ResponseCryptoPrice {
                            data: None,
                            latency: start.elapsed().as_millis(),
                            status: "FAILURE".to_string(),
                            error: format!("Response Error: {}", e),
                        };
                        final_response
                    }
                }
                
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);


    let list_responses: Arc<Mutex<Vec<ResponseCryptoPrice>>> = Arc::new(Mutex::new(Vec::new()));
    // let mut list_responses: Vec<ResponseCryptoPrice> = Vec::new()

    resp_payloads
        .for_each(|response| async {
            list_responses.lock().unwrap().push(response);
        })
        .await;

    let mut response_times: Vec<u128> = vec![];
    let mut response_data: Vec<String> = vec![];

    let overall_duration = overall_start.elapsed().as_millis();


    let mut total_time: u128 = 0;
    
    // drop mutex guard
    let list_responses = Arc::try_unwrap(list_responses).unwrap().into_inner().unwrap();
    
    for resp in list_responses{

        if resp.error != "".to_string() {println!("{}", resp.error);}
        match resp.data {
                Some(coin_price) => {response_data.push(format_coin_price(coin_price))},
                None => {}, 
            }
        total_time += resp.latency;
        response_times.push(resp.latency);
        // println!("{:?}", Some(resp.data));
    }
    
    println!("{}", response_data.join("\n"));

    response_times.sort();
    let mid = response_times.len() / 2;
    let p90 = response_times.len() - (response_times.len() / 9);
    println!("Median Latency of {} ms and 90th percentile of {} ms across {} calls", response_times[mid], response_times[p90], response_times.len());
    println!("Total time spent accross threads {} ms", total_time);
    println!("Overall duration {} ms. Note this is lot smaller than total time above", overall_duration);

}