# rest-client-demo

Testing out Rust libraries for making concurrent async rest api calls. Illustrates use of the following -

- Use of reqwest library
- using serde_json for Seriealize / Deserialize 
- Setting header parameters and using auth tokens
- Setting connection timeout, retires and exponential backoff while making REST api calls
- Struct and Traits
- async move using tokio library

We call publically available coinbase and spotify apis. Generate a spotify auth token and add under main.rs before building and running code by using -

```cargo build --release && target/release/rest-client-demo```

