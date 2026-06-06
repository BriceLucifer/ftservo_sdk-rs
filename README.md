# FTServo SDK for Rust

Rust SDK for controlling FEETECH/FTServo serial bus servos. The crate supports SMS/STS series servos and SCSCL series servos, including single-servo commands, synchronized writes, synchronized reads, and common telemetry reads.

中文文档: [README_zh.md](README_zh.md)

## Features

- SMS/STS and SCSCL protocol support
- Position, speed, PWM, torque-enable, EEPROM lock/unlock, ID, and baud-rate operations
- Telemetry reads for position, speed, load, voltage, temperature, movement state, current, and model number
- Sync write and sync read helpers for multi-servo control
- Cross-platform serial support through `serialport`
- Protocol behavior aligned with the official FTServo Python and Linux SDKs

## Supported Devices

SMS/STS series:

- SMS40 series
- STS3032 series
- STS3215 series
- Other servos compatible with the SMS/STS protocol

SCSCL series:

- SCSCL digital servos
- Other servos compatible with the SCSCL protocol

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
ftservo_sdk = "1.0.0"
```

Or use Cargo:

```bash
cargo add ftservo_sdk
```

## Quick Start

### SMS/STS Position Control

```rust
use ftservo_sdk::{create_port_handler, create_sms_sts, COMM, Result};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let mut port_handler = create_port_handler("/dev/ttyUSB0");
    port_handler.set_baudrate(1_000_000)?;
    port_handler.open_port()?;

    let mut servo = create_sms_sts(port_handler);

    match servo.ping(1) {
        COMM::Success => println!("[ID:001] connected"),
        result => {
            println!("[ID:001] ping failed: {:?}", result);
            return Ok(());
        }
    }

    servo.write_torque_enable(1, true);

    let (result, error) = servo.write_pos_ex(1, 2048, 2400, 0);
    println!("write_pos_ex result: {:?}, servo error: {}", result, error);

    thread::sleep(Duration::from_millis(1500));

    match servo.read_pos(1) {
        Ok(position) => println!("current position: {}", position),
        Err(result) => println!("read position failed: {:?}", result),
    }

    Ok(())
}
```

### SCSCL Position Control

```rust
use ftservo_sdk::{create_port_handler, create_scscl, COMM, Result};

fn main() -> Result<()> {
    let mut port_handler = create_port_handler("/dev/ttyUSB0");
    port_handler.set_baudrate(1_000_000)?;
    port_handler.open_port()?;

    let mut servo = create_scscl(port_handler);

    match servo.ping(1) {
        COMM::Success => println!("[ID:001] connected"),
        result => println!("[ID:001] ping failed: {:?}", result),
    }

    let (result, error) = servo.write_pos(1, 2048, 1000, 2400);
    println!("write_pos result: {:?}, servo error: {}", result, error);

    Ok(())
}
```

### Sync Write

```rust
use ftservo_sdk::{create_port_handler, create_sms_sts, COMM, Result};

fn sync_write_example() -> Result<()> {
    let mut port_handler = create_port_handler("/dev/ttyUSB0");
    port_handler.set_baudrate(1_000_000)?;
    port_handler.open_port()?;

    let mut servo = create_sms_sts(port_handler);
    let ids = [1, 2, 3];
    let positions = [1024, 2048, 3072];
    let speeds = [2400, 2400, 2400];
    let accs = [0, 0, 0];

    servo.group_sync_write.clear_param();
    for i in 0..ids.len() {
        servo.sync_write_pos_ex(ids[i], positions[i], speeds[i], accs[i])?;
    }

    match servo.group_sync_write.tx_packet(&mut servo.ph) {
        COMM::Success => println!("sync write sent"),
        result => println!("sync write failed: {:?}", result),
    }

    Ok(())
}
```

## API Overview

### `PortHandler`

Owns the serial port and packet timeout state.

```rust
let mut port_handler = ftservo_sdk::PortHandler::new("/dev/ttyUSB0");
port_handler.set_baudrate(1_000_000)?;
port_handler.open_port()?;
```

Supported baud rates:

- `4800`
- `9600`
- `14400`
- `19200`
- `38400`
- `57600`
- `76800`
- `115200`
- `128000`
- `250000`
- `500000`
- `1000000`

### `SmsSts`

Controller for SMS/STS protocol servos.

Common methods:

- `ping(id)`
- `ping_model(id)`
- `read_model(id)`
- `write_pos_ex(id, position, speed, acc)`
- `reg_write_pos_ex(id, position, speed, acc)`
- `sync_write_pos_ex(id, position, speed, acc)`
- `read_pos(id)`
- `read_speed(id)`
- `read_pos_speed(id)`
- `read_load(id)`
- `read_voltage(id)`
- `read_temperature(id)`
- `read_current(id)`
- `read_moving(id)`
- `write_spec(id, speed, acc)`
- `wheel_mode(id)`
- `write_torque_enable(id, enabled)`
- `lock_eprom(id)`
- `unlock_eprom(id)`
- `set_id(old_id, new_id)`
- `set_baudrate(id, baudrate_code)`

### `Scscl`

Controller for SCSCL protocol servos.

Common methods:

- `ping(id)`
- `ping_model(id)`
- `read_model(id)`
- `write_pos(id, position, time, speed)`
- `reg_write_pos(id, position, time, speed)`
- `sync_write_pos(id, position, time, speed)`
- `read_pos(id)`
- `read_speed(id)`
- `read_pos_speed(id)`
- `read_load(id)`
- `read_voltage(id)`
- `read_temperature(id)`
- `read_moving(id)`
- `pwm_mode(id)`
- `write_pwm(id, pwm)`
- `write_angle_limit(id, min_angle, max_angle)`
- `write_dead_zone(id, cw_dead, ccw_dead)`
- `write_offset(id, offset)`
- `write_torque_enable(id, enabled)`
- `lock_eprom(id)`
- `unlock_eprom(id)`
- `write_id(id, new_id)`
- `write_baudrate(id, baudrate_code)`

### `GroupSyncWrite`

`GroupSyncWrite` builds a broadcast sync-write packet. High-level servo types expose a configured `group_sync_write` field and convenience methods for adding correctly encoded parameters.

```rust
servo.group_sync_write.clear_param();
servo.sync_write_pos_ex(1, 1024, 2400, 0)?;
servo.sync_write_pos_ex(2, 2048, 2400, 0)?;
let result = servo.group_sync_write.tx_packet(&mut servo.ph);
```

### `GroupSyncRead`

`GroupSyncRead` owns a protocol handler and stores the latest returned data for each registered servo.

```rust
use ftservo_sdk::{GroupSyncRead, PortHandler, ProtocolPacketHandler, Endian};

let port_handler = PortHandler::new("/dev/ttyUSB0");
let protocol = ProtocolPacketHandler::new(port_handler, Endian::SmallEndian);
let mut group_sync_read = GroupSyncRead::new(protocol, 56, 2);

group_sync_read.add_param(1)?;
group_sync_read.tx_rx_packet();
let position = group_sync_read.get_data(1, 56, 2);
```

## Error Handling

The crate exposes a crate-level `Result<T>` using `FtServoError` for serial setup and general API helpers:

```rust
pub enum FtServoError {
    SerialPort(serialport::Error),
    Communication(COMM),
    InvalidParameter(String),
    Timeout,
    ChecksumError,
    Io(std::io::Error),
}
```

Low-level servo commands return the official-style communication result enum:

```rust
pub enum COMM {
    Success,
    PortBusy,
    TxFail,
    RxFail,
    TxError,
    RxWaiting,
    RxTimeout,
    RxCorrupt,
    NotAvailable,
}
```

Write commands return `(COMM, u8)`, where the second value is the servo status error byte returned by the device.

## Examples

Run examples with:

```bash
cargo run --example basic_control
cargo run --example sync_control
cargo run --example scscl_control
cargo run --example status_monitor
```

Update `/dev/ttyUSB0`, servo IDs, baud rate, and power wiring for your hardware before running examples.

## Hardware Notes

Typical USB-to-TTL adapter wiring:

- Adapter GND to servo power GND
- Servo power supply positive to servo VCC
- Adapter TX and RX tied to the servo data line for half-duplex bus communication

Use a power supply sized for servo stall current. Do not power multiple high-torque servos from a weak USB adapter.

## Troubleshooting

Permission denied on Linux:

```bash
sudo usermod -a -G dialout $USER
```

Then log out and log back in.

No response from servos:

- Check the servo ID.
- Check the baud rate.
- Confirm the selected protocol family, SMS/STS vs SCSCL.
- Confirm the adapter supports the half-duplex bus wiring.
- Confirm the servo has adequate external power and common ground.

Timeouts or corrupt packets:

- Reduce baud rate for testing.
- Shorten or improve the bus wiring.
- Avoid commanding multiple servos from an undersized supply.

## Development

```bash
cargo fmt
cargo check --examples
cargo clippy --all-targets -- -D warnings
cargo test
```

Package check:

```bash
cargo package --allow-dirty
```

## Release Notes

### v1.0.0

- Promoted the crate to the first stable release.
- Fixed `ping` and `action` protocol semantics.
- Added parameter-oriented read helpers and model-reading APIs.
- Fixed `GroupSyncRead` parsing and checksum handling.
- Made sync-write packet ordering deterministic.
- Corrected the SCSCL EEPROM lock address.
- Added `76800` baud-rate support.
- Aligned examples and documentation with the current API.

## License

Licensed under `MIT OR Apache-2.0`, as declared in `Cargo.toml`.
