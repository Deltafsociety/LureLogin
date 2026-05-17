<div align="center">

```
██╗      ██████╗  ██████╗ ██╗███╗   ██╗██╗     ██╗   ██╗██████╗ ███████╗
██║     ██╔═══██╗██╔════╝ ██║████╗  ██║██║     ██║   ██║██╔══██╗██╔════╝
██║     ██║   ██║██║  ███╗██║██╔██╗ ██║██║     ██║   ██║██████╔╝█████╗  
██║     ██║   ██║██║   ██║██║██║╚██╗██║██║     ██║   ██║██╔══██╗██╔══╝  
███████╗╚██████╔╝╚██████╔╝██║██║ ╚████║███████╗╚██████╔╝██║  ██║███████╗
╚══════╝ ╚═════╝  ╚═════╝ ╚═╝╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝
```

**A lightweight SSH honeypot written in Rust.**  
Fake shell. GeoIP logging. Live TUI dashboard.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-22ff88?style=flat-square)
![Status](https://img.shields.io/badge/Status-Active-22ff88?style=flat-square)
![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS-blue?style=flat-square)

</div>

---

> **For research, education, and defensive security monitoring only.**  
> Do not deploy on networks you do not own. Run in an isolated VPS or container.

---

## What it does

LoginLure mimics a real SSH server to observe and log unauthorized access attempts in a safe, controlled environment. Attackers connect, enter credentials, run commands — none of it executes. All of it gets logged.

```
[+] Connection from 185.XX.XX.10 (Russia)
[+] Login attempt: root / admin123
[+] Command: ls
[+] Command: cat /etc/passwd
```

---

## Features

| | |
|---|---|
| Fake SSH banner | OpenSSH-compatible server fingerprint |
| Interactive fake shell | Accepts commands, executes nothing |
| Credential logging | Captures every username and password attempt |
| Command logging | Records all commands typed in the fake shell |
| GeoIP tracking | Resolves country from attacker IP |
| SQLite storage | Persistent local database, no external deps |
| Live TUI dashboard | Real-time terminal UI with stats and activity |
| Async networking | Built on Tokio for high-concurrency handling |

---

## Project structure

```
sshhoney/
├── Cargo.toml
├── GeoLite2-City.mmdb       # GeoIP database (see setup below)
├── honeypot.db              # SQLite log (auto-created)
└── src/
    ├── main.rs              # Entry point, TCP listener
    ├── db.rs                # SQLite read/write
    ├── geo.rs               # IP to country resolution
    ├── shell.rs             # Fake shell session handler
    └── tui.rs               # Terminal dashboard
```

---

## Installation

**1. Clone the repo**

```bash
git clone git@github.com:YOUR_USERNAME/sshhoney.git
cd sshhoney
```

**2. Install Rust** (if needed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**3. Set up the GeoIP database**

Download the free [MaxMind GeoLite2 City](https://dev.maxmind.com/geoip/geolite2-free-geolocation-data/) database and place it in the project root:

```
sshhoney/
└── GeoLite2-City.mmdb   <-- here
```

---

## Running

```bash
cargo run --release
```

Listens on port `2222` by default. Test locally with:

```bash
ssh localhost -p 2222
```

---

## TUI dashboard

The built-in terminal UI updates live and displays:

- Active connections
- Top usernames and passwords
- Countries of origin
- Attack statistics over time

---

## Docker

```dockerfile
FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/sshhoney"]
```

---

## How it works

```
attacker connects
      |
      v
fake SSH banner sent
      |
      v
credentials captured --> logged to SQLite
      |
      v
fake shell session starts
      |
      v
commands captured --> logged to SQLite
      |
      v
TUI dashboard updates in real time
```

---

## Safety checklist

- Run in an isolated VPS or container
- Do not connect to production systems
- Do not store real credentials anywhere in the config
- Use a firewall to limit exposure beyond port 2222

---

## Roadmap

- [ ] Web dashboard (Axum + HTMX)
- [ ] Telegram / Discord alerts
- [ ] Attack heatmaps
- [ ] Fake command evolution engine
- [ ] Multi-port honeypot mode
- [ ] Export logs to ELK stack

---

## License

MIT

---

<div align="center">
Built for cybersecurity research and defensive monitoring.
</div>
