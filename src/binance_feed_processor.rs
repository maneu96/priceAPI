use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;

pub struct BinanceFeedProcessor;

impl BinanceFeedProcessor {
    pub fn process(data: &str) -> Result<String, Box<dyn std::error::Error>> {

        let data_json: Value = serde_json::from_str(data)?; // Convert the data string into JSON Value so that it is easier to parse
        let binance_timestamp: u128 =   (data_json["E"].to_string()).parse()?; //P arse the timeStamp of Binance and cast it to u128
        let latency = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()- binance_timestamp; // Get the server's timestamp and calculate latency
        //let output = String::from("Latency (ms): ") + &latency.to_string()+ &String::from("\n") + &String::from("BTC Price (USDT): ") + data_json["c"].to_string().trim_matches('\"') + &String::from("\n");
        let output = format!("Price BTC/USDT: {} \n Latency: {}\n",data_json["c"].to_string().trim_matches('\"'),latency);
        Ok(output)
    }
}