use tungstenite::{connect, Message, WebSocket, stream::Stream};
use std::{net::TcpStream};
use url::Url;
use native_tls::TlsStream;
use serde_json::{json, Value};

type WebsocketCustom = WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>;

fn main() {

    let funding_rate = get_funding();
    let price = get_price();
    let oi = get_open_interest();
    let spread = get_spread();
    let vol = get_volume();
    println!("funding rate:{} \nprice:{}\noi:{}", funding_rate, price, oi);
}

fn connect_bitmex() -> WebsocketCustom {
    let bitmex_url = "wss://www.bitmex.com/realtime?";
    let (mut socket, _) = 
        connect(Url::parse(bitmex_url).unwrap())
        .expect("could not connect");

    let connect_msg = socket.read_message().unwrap();
    drop(connect_msg);
    socket
}

fn get_funding() -> f64{
    let socket = connect_bitmex();
    
    let command = json!({
        "op": "subscribe",
        "args": "funding:XBTUSD"
    });
    
    let resp_json = get_data(socket, command);
    let resp_funding_rate = resp_json["data"][0]["fundingRate"].to_string();
    let funding_rate : f64 = resp_funding_rate.parse().unwrap();
    
    funding_rate
}

fn get_price() -> f64 {
    let socket = connect_bitmex();
    
    let command = json!({
        "op": "subscribe",
        "args": "quote:XBTUSD"
    });

    let resp_json = get_data(socket, command);
    let resp_price = resp_json["data"][0]["askPrice"].to_string();
    let price : f64 = resp_price.parse().unwrap();
    
    price
}

fn get_open_interest() -> f64 {
    let socket = connect_bitmex();
    let command = json!({
        "op": "subscribe",
        "args": "instrument:XBTUSD"
    });

    let resp_json = get_data(socket, command);
    let resp_oi = resp_json["data"][0]["openInterest"].to_string();
    let oi : f64 = resp_oi.parse().unwrap();

    oi  
}

fn get_spread() -> f64 {
    let socket = connect_bitmex();
    let command = json!({
        "op": "subscribe",
        "args": "tradeBin5m:XBTUSD"
    });

    let resp_json = get_data(socket, command);
    let resp_high: f64 = resp_json["data"][0]["high"].to_string().parse().unwrap();
    let resp_low: f64 = resp_json["data"][0]["low"].to_string().parse().unwrap();
    let spread: f64 = resp_high - resp_low;

    spread
}

fn get_volume() -> f64 {
    let socket = connect_bitmex();
    let command = json!({
        "op": "subscribe",
        "args": "instrument:XBTUSD"
    });

    let resp_json = get_data(socket, command);
    let resp_vol = resp_json["data"][0]["volume"].to_string();
    let vol: f64 = resp_vol.parse().unwrap();

    vol
}

fn get_data(mut socket: WebsocketCustom, command: Value) -> Value {
    socket
    .write_message(Message::Text(command.to_string()))
    .unwrap();

    let connect_msg = socket.read_message().unwrap();
    drop(connect_msg);
    
    let resp = socket.read_message().unwrap();
    let resp_text = resp.to_text().unwrap();
    let resp_json: Value = serde_json::from_str(resp_text).unwrap();
    socket.close(None).expect("did not close properly");
    resp_json
}









