# EV Charging Infrastructure

A Rust-based Electric Vehicle (EV) charging infrastructure simulation with OCPP 1.6 support. This project models Charge Points (CPs) and a Central Management System (CMS), enabling real-time charger management, energy tracking, transaction logging, and analytics visualization.

## Features

- **OCPP 1.6**: Implements key messages (BootNotification, StatusNotification, StartTransaction, StopTransaction, Heartbeat, MeterValues).
- **Multiple Chargers**: Supports multiple simulated chargers with configurable power ratings.
- **Database**: Stores transactions in SQLite (extensible to PostgreSQL).
- **Analytics**: Visualizes energy usage with `plotters`.
- **Scalable**: Built with `tokio` for async WebSocket communication.
- **Hardware-Ready**: Placeholder for `embedded-hal` integration.
- **CLI Tool**: Check charger status and generate plots.

## Prerequisites

- **Rust**: 1.80 or later (ARM64 support for Mac M1).
- **SQLite**: For transaction storage.
- **Homebrew**: For dependency management on macOS.

## Getting Started (Mac M1)

### 1. Install Dependencies

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Update Rust
rustup update

# Install Homebrew (if not installed)
# /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 2. Install SQLite

```bash
# Install SQLite
brew install sqlite
```

### 3. Clone the Repository

```bash

```
