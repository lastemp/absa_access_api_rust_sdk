# absa_access_api_rust_sdk

This is an sdk that will be used by developers to seamlessly integrate with Absa Access APIs' Gateway.
Absa Group Limited, commonly known simply as Absa and formerly the Amalgamated Banks of South Africa until 2005 and Barclays Africa Group Limited until 2018, is a multinational banking and financial services conglomerate based in Johannesburg, South Africa and listed on the Johannesburg Stock Exchange..
The API endpoints provided by Absa Access API Gateway includes; Foreign Exchange(FX Rates, FX Trade) and Retail Payments(domestic EFT and RTGS payments) (https://developer.absa.africa/static/home). 

The sdk has below listed dependencies:
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client
- [serde_json](https://github.com/serde-rs/json) for serializing and deserializing Rust data structures
- [serde_urlencoded](https://github.com/nox/serde_urlencoded) for serialising to and deserialising from the application/x-www-form-urlencoded format
- [chrono](https://github.com/chronotope/chrono) provides all functionality needed to do correct operations on dates and times
- [base64](https://github.com/marshallpierce/rust-base64) Decode from Base64 format or encode into it
- [tokio](https://github.com/tokio-rs/tokio) A runtime for writing reliable, asynchronous applications

## installation

```
cargo install --git https://github.com/lastemp/absa_access_api_rust_sdk
```

## Usage

Please find below code samples and full working examples:

   - See [the code samples](./code_samples/) for more info.	
   - See [the examples](./examples/) for full working examples.
