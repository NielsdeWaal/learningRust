extern crate curl;
extern crate env_logger;
extern crate futures;
extern crate tokio_core;
extern crate tokio_curl;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::{self, Write};
use curl::easy::Easy;
use futures::Future;
use tokio_core::reactor::Core;
use tokio_curl::Session;
use serde_json::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Order {
    order_id: u64,
    type_id: u32,
    location_id: u32,
    volume_total: u32,
    volume_remain: u32,
    min_volume: u8,
    price: u64,
    is_buy_order: bool,
    duration: u32,
    issued: Vec<String>,
    range: Vec<String>,
}

fn main() {
    let mut lp = Core::new().unwrap();
    let session = Session::new(lp.handle());

    let mut json = Vec::new();

    // Once we've got our session available to us, execute our two requests.
    // Each request will be a GET request and for now we just ignore the actual
    // downloaded data.
    let mut a = Easy::new();
    a.get(true).unwrap();
    a.url("https://esi.tech.ccp.is/latest/markets/10000002/orders/?datasource=tranquility&order_type=all&page=1&type_id=18").unwrap();
    a.write_function(|data| {
        //io::stdout().write_all(data).unwrap();
        json.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();

    let requests = session.perform(a);

    let order_result: Order = serde_json::from_slice(&json);

    // Run both requests, waiting for them to finish. Once done we print out
    // their response codes and errors.
    let mut a = lp.run(requests).unwrap();
    println!("{:?}", a.response_code());

    println!(" ");

    println!("{:?}", order_result);
}
