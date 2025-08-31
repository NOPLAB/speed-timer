# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Architecture

This is a dual-component IoT speed timer system:

### ESP32 Firmware (`esp32/`)
- **Hardware**: ESP32 microcontroller with I2C LCD display (via PCF8574 port expander) and BLE connectivity
- **Core Function**: High-precision timer that displays elapsed time on LCD and broadcasts timer data via Bluetooth Low Energy
- **Key Components**:
  - Timer interrupt handler running at 1ms precision (`TIMER_COUNTER` global variable)
  - BLE GATT server with custom UUIDs for timer data broadcasting
  - LCD display management via ag-lcd library
  - I2C communication on pins GPIO12 (SDA) and GPIO13 (SCL)

### Desktop Application (`speed-timer-app/`)
- **Stack**: Tauri v2 + React + TypeScript + Vite
- **Purpose**: Cross-platform desktop app to interface with ESP32 timer (currently template code)
- **Architecture**: Rust backend (`src-tauri/`) with React frontend (`src/`)

## Development Commands

### ESP32 Firmware
```bash
cd esp32/

# Build firmware
./scripts/build.sh              # Release build
./scripts/build.sh debug        # Debug build

# Flash to device
./scripts/flash.sh              # Flash release build
./scripts/flash.sh debug        # Flash debug build

# Direct cargo commands
cargo build --release
cargo espflash flash -b no-reset -M
cargo espflash flash -b no-reset -M --no-stub
```

### Desktop Application
```bash
cd speed-timer-app/

# Development
npm run dev                     # Start Vite dev server
npm run tauri dev              # Start Tauri development mode

# Build
npm run build                  # Build frontend (TypeScript + Vite)
npm run tauri build           # Build complete Tauri application

# Preview
npm run preview               # Preview built frontend
```

## Hardware Configuration

### ESP32 Pinout
- **GPIO12**: I2C SDA (LCD via PCF8574)
- **GPIO13**: I2C SCL (LCD via PCF8574)
- **I2C Frequency**: 400kHz
- **Timer**: Timer00 with 1ms precision interrupts

### BLE Configuration
- **Device Name**: "ESP32-GATT-Server"
- **Service UUID**: `fafafafa-fafa-fafa-fafa-fafafafafafa`
- **Characteristics**:
  - Static: `d4e0e0d0-1a2b-11e9-ab14-d663bd873d93` (READ)
  - Timer Data: `a3c87500-8ed3-4bdf-8a39-a01bebede295` (READ + NOTIFY)
  - Control: `3c9a3f00-8ed3-4bdf-8a39-a01bebede295` (READ + WRITE)

## Key Dependencies

### ESP32
- `esp-idf-svc`: ESP-IDF service layer
- `ag-lcd`: LCD display driver with I2C support
- `port-expander`: PCF8574 I2C port expander driver
- `esp32-nimble`: Bluetooth Low Energy stack

### Desktop App
- `@tauri-apps/api`: Tauri frontend API
- `@tauri-apps/plugin-opener`: System opener plugin
- React 18 + TypeScript for frontend

## Build Requirements

### ESP32
- Rust toolchain with ESP32 target
- ESP-IDF environment (sourced via `~/export-esp.sh`)
- Required tools: `ldproxy`, `espup`, `espflash`, `cargo-espflash`

### Desktop App
- Node.js with npm
- Rust toolchain for Tauri backend
- Platform-specific build tools for native compilation

## Development Notes

- ESP32 firmware uses unsafe global `TIMER_COUNTER` accessed from interrupt handler
- Timer broadcasts 64-bit little-endian counter value via BLE notifications
- Tauri app currently contains template code - needs integration with BLE timer data
- Wokwi simulator configuration available for ESP32 debugging