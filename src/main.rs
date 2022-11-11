
mod rest_async_client_with_auth_and_retry;
mod rest_client_basic;
mod rest_client_impl;
mod rest_client_trait;
mod concurrent_calls;



fn main() {


    println!("Rest Client Basics");
    let list_currency = rest_client_basic::get_all_currencies(); 
    println!("Extracted {} currencies", list_currency.data.len());
    
    println!("\n\nRest Async Client Call with timeout, retry and auth");
    // Get auth token from here https://developer.spotify.com/console/get-search-item/ and replace string below
    let spotify_auth_token = "Bearer BQDaZgvNBZvMvnBFioyV9Bvh7Yhgh9MtAIazKexeOqm1ApnIhLbhQ0H-0TxH3omALP6IstBppJAM7NER619-46t7mlr-th7Oh33bqTqdQUCr4LOwnT5tDfWVbsxg4EADAhSyJvpjZtb1XV7RVmRxVJhoCDrb9LZyZG9Xz3vwR5cyJgeXFadxfd8oYZyhY7VnFCc".to_string();
    rest_async_client_with_auth_and_retry::search_spotify_titles(spotify_auth_token);
    
    println!("\n\nRest Client with Implementation");
    rest_client_impl::get_coin_prices(&"BTC".to_string()); 
    rest_client_impl::get_coin_prices(&"ETH".to_string()); 

    println!("\n\nRest Client with Implementation and Trait");
    rest_client_trait::get_price_spread(&"BTC".to_string());
    rest_client_trait::get_price_spread(&"ETH".to_string());

    println!("\n\nRest Concurrent Calls. Time to put it all together!!");
    let list_crypto = vec!["BTC", "ETH", "AVAX", "SOL", "ADA", "USDC", "DOGE", "MATIC", "USDT"];
    concurrent_calls::get_all_prices(list_currency.data, list_crypto, "USD");

}

