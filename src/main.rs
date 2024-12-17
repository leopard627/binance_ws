use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde::{Deserialize, Serialize};
use futures_util::StreamExt;
use url::Url;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
struct WebSocketResponse {
    stream: String,
    data: Vec<MiniTicker>,
}

#[derive(Debug, Deserialize)]
struct MiniTicker {
    e: String,              // Event type
    E: i64,                // Event time
    s: String,             // Symbol
    c: String,             // Close price
    o: String,             // Open price
    h: String,             // High price
    l: String,             // Low price
    v: String,             // Total traded base asset volume
    q: String,             // Total traded quote asset volume
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // WebSocket URL
    let ws_url = Url::parse(
        "wss://stream.binance.com:9443/stream?streams=!miniTicker@arr&timeUnit=MICROSECOND"
    )?;

    // Connect to WebSocket
    let (ws_stream, _) = connect_async(ws_url).await?;
    println!("WebSocket connected");

    // Split the WebSocket stream
    let (_, mut read) = ws_stream.split();

    // Handle incoming messages
    while let Some(message) = read.next().await {
        match message {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<WebSocketResponse>(&text) {
                    Ok(ticker_response) => {
                        // Process each ticker in the response
                        for ticker in ticker_response.data {
                            let time = DateTime::<Utc>::from_timestamp(
                                ticker.E / 1_000_000, // Convert microseconds to seconds
                                (ticker.E % 1_000_000) as u32 * 1000 // Remaining microseconds to nanoseconds
                            ).unwrap();
                            
                            println!(
                                "Symbol: {}, Price: {}, Time: {}, Volume: {}",
                                ticker.s, ticker.c, time, ticker.v
                            );
                        }
                    }
                    Err(e) => eprintln!("Failed to parse message: {}", e),
                }
            }
            Ok(Message::Ping(_)) => {
                println!("Received ping");
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
