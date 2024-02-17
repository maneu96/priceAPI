/* ***************************************************************************************************************************************/
// This is an implementation of a struct that introduces a method to parse and extract information (price,latency) from a Binance data feed
/* ***************************************************************************************************************************************/

use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct BinanceFeedProcessor;

impl BinanceFeedProcessor {
    pub fn process(data: &str) -> Result<String, Box<dyn std::error::Error>> {
        /***********************************************************************************************************************/
        //
        //  inputs : ***********************************************************************************************************
        //          data: &str // binance data feed
        // outputs: ************************************************************************************************************
        //          Result<String, Box<dyn std::error::Error>> // Either a string with both the price and latency
        //                                                        (between the caller and binance timestamp) or an error message
        /***********************************************************************************************************************/

        // Convert the data string into JSON Value so that it is easier to parse
        let data_json: Value = serde_json::from_str(data)?;
        //Parse the timeStamp of Binance (present in data_json["E"]) and cast it to u128
        let binance_timestamp: u128 = (data_json["E"].to_string()).parse()?;
        // Get the server's timestamp and calculate latency
        //let latency = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() - binance_timestamp;

        // Build the output string by accessing the proper fields in the data_json, according to the structure of the binance feed (data_json["c"] for the price)
        let output = format!(
            "Price BTC/USDT: {} \nBinance Timestamp (ms): {} \nServer Timestamp (ms): {}\n",
            data_json["c"].to_string().trim_matches('\"'),
            binance_timestamp,
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()
        );
        Ok(output) //Return the output, with proper error handling
    }
}
