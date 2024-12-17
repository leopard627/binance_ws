# Binance WebSocket Real-Time Market Data Client

A Rust-based WebSocket client that connects to Binance's public WebSocket API to receive real-time market data. This client specifically handles mini ticker data for all trading pairs.

## Features

- Real-time connection to Binance WebSocket streams
- Processes mini ticker data for all trading pairs
- Microsecond precision timestamp handling
- Automatic JSON deserialization into strongly-typed structures
- Error handling and connection monitoring
- Clean and efficient data processing

## Prerequisites

- Rust (latest stable version)
- Cargo package manager

## Dependencies

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
tokio-tungstenite = { version = "0.20", features = ["native-tls"] }
futures-util = "0.3"
url = "2.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/leopard627/binance_ws
cd binance_ws
```

2. Build the project:
```bash
cargo build --release
```

3. Run the client:
```bash
cargo run --release
```

## Usage

The client connects to Binance's WebSocket stream and processes mini ticker data in real-time. By default, it connects to:
```
wss://stream.binance.com:9443/stream?streams=!miniTicker@arr&timeUnit=MICROSECOND
```

The received data includes:
- Symbol name
- Current price
- Trading volume
- High/Low prices
- Timestamp (microsecond precision)

Example output:
```
Symbol: BTCUSDT, Price: 50000.00, Time: 2024-03-17T10:30:15.123456Z, Volume: 100.50
```

## Data Structure

The client processes the following data structure:

```rust
struct MiniTicker {
    e: String,    // Event type
    E: i64,       // Event time
    s: String,    // Symbol
    c: String,    // Close price
    o: String,    // Open price
    h: String,    // High price
    l: String,    // Low price
    v: String,    // Total traded base asset volume
    q: String,    // Total traded quote asset volume
}
```

## Error Handling

The client includes comprehensive error handling for:
- WebSocket connection issues
- JSON parsing errors
- Message processing failures

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Binance for providing the WebSocket API
- The Rust community for excellent WebSocket and async runtime libraries

## Disclaimer

This project is not affiliated with Binance. Please review Binance's terms of service and API documentation before using this client in a production environment.
