# Telemetry Gateway - Drone Fleet Management System

A high-performance, real-time drone fleet monitoring and anomaly detection system built in Rust. This project simulates 100+ drones transmitting telemetry data over UDP, processes packets concurrently, detects anomalies, and visualizes the fleet status through a live terminal dashboard.

## Table of Contents

- [Project Overview](#project-overview)
- [Technical Architecture](#technical-architecture)
- [Key Features](#key-features)
- [System Requirements](#system-requirements)
- [Dependencies](#dependencies)
- [Building the Project](#building-the-project)
- [Running the Program](#running-the-program)
- [Usage Guide](#usage-guide)
- [Architecture & Components](#architecture--components)
- [Anomaly Detection System](#anomaly-detection-system)
- [Performance Characteristics](#performance-characteristics)
- [Testing](#testing)
- [Code Structure](#code-structure)

## Project Overview

Telemetry Gateway is a proof-of-concept system designed to handle real-time telemetry data from a fleet of drones. It demonstrates:

- **Concurrent Processing**: Handles 101 concurrent UDP data streams
- **Anomaly Detection**: Identifies GPS spoofing, excessive battery drain, and impossible speeds
- **Live Visualization**: Terminal-based dashboard showing real-time drone status
- **Type-Safe Concurrency**: Uses Rust's ownership model for thread-safe operations
- **Binary Serialization**: Efficient data transmission using postcard format

### Use Cases

- **Delivery Services**: Monitor drone fleets for package delivery
- **Emergency Response**: Track rescue drones in disaster zones
- **Security Operations**: Detect unauthorized or compromised drones
- **Performance Testing**: Load test telemetry systems with 100+ concurrent sources
- **Educational**: Learn concurrent Rust programming patterns

## Technical Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Main Application                         │
│  (Async Runtime: Tokio Multi-threaded)                     │
└──────────┬──────────────────────────────────────────────────┘
           │
    ┌──────┼──────────────────────┐
    │      │                      │
    ▼      ▼                      ▼
┌─────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│ Drone Simulator │  │ Packet Receiver  │  │ State Manager    │
│ (101 drones)    │  │ (UDP :8080)      │  │ (DashMap)        │
│ ~100ms interval │  │ Deserializes     │  │ Concurrent       │
│ Random telemetry│  │ Binary packets   │  │ drone tracking   │
└────────┬────────┘  └────────┬─────────┘  └────────┬─────────┘
         │                    │                     │
         └────────────────────┼─────────────────────┘
                UDP Port 8080  │
                     ┌────────▼──────────┐
                     │ Anomaly Detector  │
                     │ - Speed check     │
                     │ - Battery drain   │
                     │ - GPS spoofing    │
                     └────────┬──────────┘
                              │
                     ┌────────▼──────────┐
                     │  Live Dashboard   │
                     │  (Ratatui TUI)    │
                     │  Updates: 250ms   │
                     └───────────────────┘
```

## Key Features

1. **Drone Simulation**
   - 101 concurrent virtual drones
   - Sends randomized telemetry every 100ms
   - Realistic GPS coordinates (California region)
   - Variable altitude, speed, heading, and battery status

2. **UDP-Based Data Ingestion**
   - High-performance UDP socket receiver
   - Binary protocol (postcard serialization)
   - Non-blocking async I/O
   - Zero-copy packet processing

3. **Anomaly Detection Engine**
   - Real-time comparison of consecutive telemetry updates
   - Three detection categories:
     - **Speed Anomaly**: > 150 km/h (impossible for simulated drones)
     - **Battery Drain**: > 20% drop between updates
     - **GPS Spoofing**: Coordinate jump > 0.5 degrees

4. **Live Dashboard**
   - Real-time fleet overview table
   - Drone position, speed, battery, armed/safe status
   - Alert log showing recent system events
   - 250ms refresh rate

5. **Thread-Safe Concurrency**
   - DashMap for lock-free reads
   - Arc<> for safe shared state
   - Multi-threaded Tokio runtime
   - Stress-tested with concurrent updates

## System Requirements

- **OS**: Windows, macOS, or Linux
- **Rust**: 1.70+ (edition 2024)
- **RAM**: Minimum 128MB, Recommended 512MB+
- **Network**: Localhost UDP access (127.0.0.1:8080)

## Dependencies

The project uses these key Rust crates:

```toml
[dependencies]
tokio = "1"              # Async runtime with multi-threading
serde = "1"              # Serialization framework
postcard = "1.0"         # Binary serialization (compact format)
dashmap = "5.5"          # Concurrent hashmap (lock-free reads)
rand = "0.8"             # Random number generation
anyhow = "1.0"           # Error handling
tracing = "0.1"          # Structured logging
tracing-subscriber = "0.3" # Log output formatting
ratatui = "0.26"         # Terminal UI library
crossterm = "0.27"       # Terminal manipulation
```

## Building the Project

### Prerequisites

Install Rust from [rustup.rs](https://rustup.rs/)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/P0ntix/telemetry-gateway-rust-version1.git
cd telemetry-gateway-rust-version1

# Build in debug mode (faster compilation, slower runtime)
cargo build

# Build in release mode (optimized for performance)
cargo build --release
```

### Build Output

After building, the executable is located at:
- **Debug**: `target/debug/telemetry-gateway`
- **Release**: `target/release/telemetry-gateway`

## Running the Program

### Basic Execution

```bash
# Run with default settings (info-level logging)
cargo run

# Or run the compiled binary directly
./target/release/telemetry-gateway
```

### With Environment Variables

```bash
# Verbose logging (debug level)
RUST_LOG=debug cargo run

# Trace-level logging (very detailed)
RUST_LOG=trace cargo run

# Specific module logging
RUST_LOG=telemetry_gateway::drone=debug cargo run
```

### Example Output

```
2026-08-13T10:42:15.123456Z  INFO telemetry_gateway: starting telemetry gateway simulator
2026-08-13T10:42:15.234567Z  INFO telemetry_gateway: waiting for Ctrl+C
2026-08-13T10:42:16.456789Z WARN telemetry_gateway: anomaly detected drone_id=42 anomaly_type=GpsSpoof
2026-08-13T10:42:20.123456Z DEBUG telemetry_gateway: tracking drones drone_count=101
```

### Stopping the Program

Press **Ctrl+C** to gracefully shutdown the system. The terminal will be restored to normal mode and a shutdown message will be logged.

## Usage Guide

### Step-by-Step: Running Your First Simulation

1. **Open Terminal**
   ```bash
   cd c:\rust_projects\telemetry-gateway
   ```

2. **Start the System**
   ```bash
   cargo run --release
   ```

3. **Observe Output**
   - System starts 101 drone simulators
   - Receiver binds to UDP port 8080
   - Dashboard initializes with terminal UI

4. **Read the Dashboard**
   ```
   ┌─────────────────────────────────────────────────────────┐
   │ Telemetry Fleet                                         │
   ├──────┬─────────┬─────────┬────────┬──────┬──────────────┤
   │ Drone│   Lat   │   Lon   │ Speed  │ Batt%│   State      │
   ├──────┼─────────┼─────────┼────────┼──────┼──────────────┤
   │   0  │ 34.05   │-118.24  │  23.5  │  87.2│ ARMED        │
   │   1  │ 35.67   │-120.45  │  12.3  │  95.1│ SAFE         │
   │   2  │ 33.12   │-119.87  │  45.2  │  62.5│ ARMED        │
   │  ...                                                    │
   └─────────────────────────────────────────────────────────┘
   
   ┌─────────────────────────────────────────────────────────┐
   │ Alerts                                                  │
   ├─────────────────────────────────────────────────────────┤
   │ System online                                           │
   │ Monitoring 100 drones                                  │
   │ Waiting for telemetry                                  │
   │ Anomaly detected: Drone 7 GPS Spoof                    │
   │ Anomaly detected: Drone 15 Battery Drain               │
   │                                                        │
   └─────────────────────────────────────────────────────────┘
   ```

5. **Monitor Activity**
   - Watch drone positions update in real-time
   - Track battery levels decreasing over time
   - Observe anomalies logged in the alerts section

6. **Exit**
   - Press **Ctrl+C** at any time to stop the system
   - Terminal will return to normal mode

## Architecture & Components

### 1. Drone Simulator (`src/drone.rs`)

**Purpose**: Simulates a fleet of 101 autonomous drones sending telemetry data.

**Key Functions**:
- `spawn_drone(drone_id)`: Runs a single drone task
  - Binds UDP socket on random local port
  - Generates random telemetry data every 100ms
  - Sends binary-encoded packets to receiver
  - Returns errors if transmission fails

- `start_simulation()`: Orchestrates all 101 drones
  - Creates async task for each drone
  - Manages lifecycle and error handling
  - Spawns as background task in main runtime

**Data Format**:
```rust
Telemetry {
    drone_id: u32,              // 0-100
    timestamp: u64,             // Unix milliseconds
    lat: f64,                   // 32.0-36.0 (California)
    lon: f64,                   // -122.0 to -116.0
    altitude: f64,              // 10-220 meters
    speed: f64,                 // 0-55 km/h (unrealistic for test)
    heading: f64,               // 0-360 degrees
    battery_pct: f64,           // 15-100 percent
    is_armed: bool,             // 75% true, 25% false
}
```

### 2. Telemetry Data Structure (`src/telemetry.rs`)

**Purpose**: Defines the data format transmitted by drones.

**Key Features**:
- Serializable/Deserializable via `serde` and `postcard`
- `random(drone_id)`: Generates realistic-looking random data
- `to_bytes()`: Converts struct to binary format for UDP transmission
- Compact binary representation (< 100 bytes per packet)

**Binary Serialization**:
- Uses postcard crate for efficient encoding
- Significantly smaller than JSON/XML
- Suitable for high-frequency UDP transmission

### 3. Packet Receiver (`src/ingest.rs`)

**Purpose**: Receives and processes incoming UDP telemetry packets.

**Key Functions**:
- `start_receiver(tx, state)`: Main receiver loop
  - Binds to `0.0.0.0:8080` (all interfaces)
  - Receives UDP packets into 2048-byte buffer
  - Deserializes binary packets to Telemetry objects
  - Updates WorldState and triggers anomaly detection
  - Forwards to UI via message channel

**Processing Pipeline**:
```
UDP Packet (binary) 
    → Deserialize (postcard)
    → Telemetry struct
    → Anomaly Detection
    → State Update
    → UI Channel
```

**Error Handling**:
- Logs deserialization failures
- Uses `anyhow::Context` for detailed error messages
- Continues processing even on individual packet errors

**Testing**:
- Includes `test_ingest_receives_packet()` validating end-to-end packet flow
- Uses timeout to ensure non-blocking behavior
- Verifies data integrity through multiple packets

### 4. State Management (`src/state.rs`)

**Purpose**: Maintains thread-safe, concurrent state of all drones.

**Key Structure**:
```rust
pub struct WorldState {
    pub drones: DashMap<u16, Telemetry>,
}
```

**Key Functions**:
- `update(telemetry)`: Updates drone state and checks for anomalies
  - Retrieves previous telemetry
  - Calls anomaly detector
  - Inserts new telemetry into map
  - Returns detected anomaly (if any)

- `len()`: Returns count of tracked drones (for monitoring)

**Concurrency Design**:
- Uses DashMap (dashmap crate) for lock-free concurrent access
- Multiple threads can read/write without blocking
- Scales well to 100+ concurrent updates
- Arc<WorldState> for safe sharing across tasks

**Testing**:
- `test_world_state_concurrent_updates()`: Stress test with:
  - 101 concurrent drone tasks
  - 1000 updates per drone
  - Verifies all drones present and state consistency

### 5. Anomaly Detection (`src/anomaly.rs`)

**Purpose**: Identifies suspicious behavior in drone telemetry.

**Detection Rules**:

| Anomaly Type | Rule | Threshold | Implication |
|---|---|---|---|
| **Speed** | Current speed exceeds limit | > 150 km/h | Hardware failure or spoofing |
| **Battery Drain** | Excessive power loss | > 20% drop/update | Malfunction or attack |
| **GPS Spoof** | Impossible coordinate jump | > 0.5° lat/lon delta | Signal jamming or hacking |

**Key Function**:
```rust
pub fn detect_anomaly(old: &Telemetry, new: &Telemetry) -> Option<AnomalyType>
```

**Logic**:
1. Check speed (highest priority)
2. Check battery drain
3. Check GPS coordinates
4. Return first detected anomaly (if any)

**Logged Output**:
```
WARN drone_id=42 anomaly_type=GpsSpoof "anomaly detected"
```

**Testing**:
- `test_detects_spoof_packet()`: Verifies GPS spoof detection
- Creates two telemetry packets with large coordinate delta
- Confirms anomaly detection triggers correctly

### 6. Live Dashboard UI (`src/ui.rs`)

**Purpose**: Terminal-based visualization of fleet status.

**Key Features**:
- **Terminal Initialization**
  - Enables raw mode (direct input)
  - Enters alternate screen buffer
  - Creates crossterm backend for rendering

- **Main Table**
  - Displays all 101 drones in real-time
  - Shows: ID, Latitude, Longitude, Speed, Battery%, Armed/Safe status
  - Updates every 250ms
  - Columns are fixed-width for alignment

- **Alert Panel**
  - Stores system events and anomalies
  - Displays last 8 messages
  - Formatted as scrolling log

- **Cleanup**
  - `shutdown_terminal()`: Restores terminal on exit
  - Disables raw mode
  - Leaves alternate screen

**Rendering Loop**:
```
1. Iterate through WorldState drones
2. Format each as table row
3. Create table with header
4. Split screen: 70% table, 30% alerts
5. Render both widgets
6. Sleep 250ms
7. Repeat
```

## Anomaly Detection System

### Detection Methodology

The system uses **temporal analysis** comparing consecutive telemetry updates:

```
Time T0: Telemetry from Drone X
    ↓
Time T1 (100ms later): New Telemetry from Drone X
    ↓
Compare: 
  - Speed increase feasible?
  - Battery drain reasonable?
  - Location change realistic?
    ↓
Result: Anomaly detected OR Normal
```

### Thresholds Explained

1. **Speed Anomaly (> 150 km/h)**
   - Simulated drones max speed: 55 km/h
   - Threshold: 150 km/h (safety margin)
   - Real-world: Detects hardware failures or GPS spoofing

2. **Battery Drain (> 20%)**
   - Normal drain: 1-5% per update (100ms)
   - Threshold: 20% indicates extreme power draw
   - Real-world: Detects malware, excessive load, or system drain

3. **GPS Spoofing (> 0.5°)**
   - 1° latitude ≈ 111 km
   - 0.5° = 55 km jump in 100ms
   - Speed equivalent: 1980 km/h (impossible)
   - Real-world: Detects jamming, replay attacks, or signal hijacking

### Anomaly Logging

Every detected anomaly is logged with:
```
timestamp | level | drone_id | anomaly_type | message
```

Example:
```
2026-08-13T10:42:16.456789Z WARN drone_id=7 anomaly_type=GpsSpoof "anomaly detected"
```

## Performance Characteristics

### Throughput

- **Packet Rate**: 101 drones × 10 Hz = 1,010 packets/second
- **Bandwidth**: ~100 bytes/packet × 1,010 = 101 KB/s
- **Processing Latency**: < 1ms per packet (on modern hardware)

### Memory Usage

- **Per Drone**: ~200 bytes (telemetry data)
- **101 Drones**: ~20 KB
- **DashMap Overhead**: ~50 KB
- **Total Runtime**: ~200 MB (including Tokio runtime and UI buffers)

### Concurrency Scaling

- **Tokio Runtime**: Auto-scales to CPU core count
- **DashMap**: Lock-free reads, minimal contention
- **Tested**: 101 concurrent sources with 1000 updates each
- **Result**: All updates processed without loss

### Optimization Tips

1. **Release Mode**: `cargo build --release` (5-10x faster)
2. **Log Level**: Set `RUST_LOG=info` to reduce output
3. **Network**: Ensure UDP buffer isn't full (`sysctl -w net.core.rmem_max`)

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_ingest_receives_packet

# Run in release mode
cargo test --release
```

### Test Coverage

#### Anomaly Detection Tests (`src/anomaly.rs`)
- **test_detects_spoof_packet()**: Verifies GPS spoofing detection
  - Creates two positions 0.5° apart
  - Confirms anomaly is triggered

#### State Management Tests (`src/state.rs`)
- **test_world_state_concurrent_updates()**: Stress test
  - 101 concurrent drone tasks
  - 1000 updates per drone (101,000 total)
  - Verifies no data loss, all drones tracked
  - Ensures no race conditions

#### Ingestion Tests (`src/ingest.rs`)
- **test_ingest_receives_packet()**: End-to-end validation
  - Creates real UDP packet
  - Sends to receiver
  - Verifies reception and deserialization
  - Checks data integrity
  - Uses 2-second timeout

### Adding New Tests

Example test structure:
```rust
#[test]
fn test_my_feature() {
    // Arrange
    let setup = MyStruct::new();
    
    // Act
    let result = setup.do_something();
    
    // Assert
    assert_eq!(result, expected);
}
```

## Code Structure

```
telemetry-gateway/
├── Cargo.toml                    # Project manifest & dependencies
├── src/
│   ├── main.rs                   # Entry point, async runtime setup
│   ├── drone.rs                  # Drone simulator (UDP sender)
│   ├── ingest.rs                 # Packet receiver (UDP listener)
│   ├── telemetry.rs              # Data structures & serialization
│   ├── state.rs                  # Concurrent state management
│   ├── anomaly.rs                # Anomaly detection logic
│   └── ui.rs                     # Terminal dashboard rendering
├── target/                       # Build artifacts (auto-generated)
├── LICENSE                       # License file
└── README.md                     # This file
```

### Module Dependencies

```
main.rs
├── drone.rs (sends packets)
├── ingest.rs (receives packets)
│   └── telemetry.rs (data format)
├── state.rs (tracks drones)
│   └── anomaly.rs (detects issues)
└── ui.rs (displays dashboard)
    └── state.rs (reads data)
```

## Development Workflow

### Adding a New Feature

1. **Create New Module**: `src/feature.rs`
2. **Register in main.rs**: `mod feature;`
3. **Add Tests**: Inline tests or `src/feature/tests.rs`
4. **Integrate**: Call from main or other modules
5. **Test**: `cargo test`
6. **Build**: `cargo build --release`

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Use with Rust debugger (requires lldb/gdb)
# On Windows: Install Visual Studio Build Tools
# Then: cargo build --debug
#       rust-gdb ./target/debug/telemetry-gateway

# Check for warnings
cargo clippy

# Format code
cargo fmt
```

### Performance Profiling

```bash
# Build optimized binary
cargo build --release

# Profile with time command
time ./target/release/telemetry-gateway

# Monitor with system tools
# Windows: Task Manager
# Linux: htop
# macOS: Activity Monitor
```

## Troubleshooting

### Program Won't Start

```
error: failed to bind UDP socket
→ Port 8080 in use. Kill process using netstat:
  Windows: netstat -ano | findstr :8080
           taskkill /PID <PID> /F
  Linux:   lsof -i :8080
           kill -9 <PID>
```

### Dashboard Not Rendering

```
error: failed to enable raw mode
→ Terminal doesn't support raw mode
  Solution: Try different terminal emulator (Windows Terminal, iTerm2, etc.)
```

### Slow Performance

```
→ Running in debug mode
  Solution: cargo build --release && ./target/release/telemetry-gateway

→ High logging level
  Solution: RUST_LOG=info cargo run --release
```

### Compilation Errors

```
error: edition 2024 not supported
→ Rust version too old
  Solution: rustup update
```

## Future Enhancements

Potential improvements to the project:

1. **Persistent Storage**: Save telemetry to database (PostgreSQL, ClickHouse)
2. **Distributed System**: Multi-process receiver with message broker (Kafka)
3. **Advanced Analytics**: ML-based anomaly detection with isolation forests
4. **REST API**: HTTP endpoint for external system integration
5. **Configuration File**: YAML/TOML for tuning thresholds
6. **Metrics Export**: Prometheus metrics for monitoring

## License

This project is licensed under the MIT License. See LICENSE file for details.

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Commit changes with descriptive messages
4. Push and create a Pull Request

## Support

For issues, questions, or suggestions:
- Open an GitHub issue
- Check existing documentation above
- Review code comments in source files