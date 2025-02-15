# **MyStrom Collector**

The **MyStrom Collector** is a simple command-line utility that retrieves the current status report of one or more MyStrom adapters. It calls the report API for each provided IP and returns a JSON response containing detailed information, including the original data from MyStrom's [Report API endpoint](https://api.mystrom.ch/#fbb2c698-e37a-4584-9324-3f8b2f615fe2), with the addition of the provided IP addresses.

## **Features**

- Collects real-time data from multiple MyStrom adapters concurrently.
- Returns a JSON response with detailed reports, including the adapter's IP address.
- Supports multiple adapter IP addresses in a single command.
- Handles errors gracefully and includes timeouts to prevent hanging requests.

## **Usage**

To use the **MyStrom Collector**, simply provide one or more IP addresses of the MyStrom adapters as arguments:

```bash
./mystrom-collector 192.168.42.13 192.168.42.14 192.168.42.15 [...]
```

### **Example Commands**

```bash
./mystrom-collector 192.168.42.13
```

This will fetch the report for the MyStrom adapter with the IP `192.168.42.13`.

### **Output**

The output is a JSON array containing the report for each adapter, with the following structure:

```json
[
  {
    "Ws": 787.43,
    "boot_id": "A425D9AD",
    "energy_since_boot": 64880353.7,
    "ip": "192.168.42.13",
    "power": 588,
    "relay": true,
    "temperature": 11.86,
    "time_since_boot": 1491275
  },
  {
    "Ws": 2.47,
    "boot_id": "A625D9AD",
    "energy_since_boot": 26958184.68,
    "ip": "192.168.42.14",
    "power": 2.98,
    "relay": true,
    "temperature": 23.19,
    "time_since_boot": 6923279
  },
  {
    "Ws": 6.18,
    "boot_id": "A425D8AD",
    "energy_since_boot": 26458414.02,
    "ip": "192.168.42.15",
    "power": 5.92,
    "relay": true,
    "temperature": 22.48,
    "time_since_boot": 3111985
  }
]
```

### **Error Handling**

If a request fails (e.g., due to a timeout or network issue), the program will log an error message to stderr but continue processing the remaining IP addresses. The final stdout JSON output will only include successfully retrieved reports.

---

## **Installation**

1. Ensure you have Rust installed. If not, follow the instructions at [rustup.rs](https://rustup.rs/).
2. Clone this repository:
   ```bash
   git clone https://github.com/your-repo/mystrom-collector.git
   cd mystrom-collector
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The binary will be located at `./target/release/mystrom-collector`.

---

## **Dependencies**

- [reqwest](https://crates.io/crates/reqwest): For making HTTP requests.
- [tokio](https://crates.io/crates/tokio): For asynchronous runtime and concurrency.
- [serde_json](https://crates.io/crates/serde_json): For JSON serialization and deserialization.

---

## **Contributing**

Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.

---

## **License**

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---