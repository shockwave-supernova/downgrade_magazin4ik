# 🕵️‍♀️ DG-MAG Adaptive Scanner

A robust, automated Rust-based tool designed to exfiltrate **Downgrade Magazine** PDF payloads from `dgmag.in`. It features an adaptive brute-force engine to handle inconsistent naming conventions and delivers findings directly to your secure SMTP relay.

Developed by **Rachel**.

## ⚡ Features

* **Cyber-Noir Interface**: Includes a slow-printing COBOL-style terminal initialization for that "logged-in-to-the-mainframe" aesthetic.
* **Adaptive Suffix Probe**: Automatically detects irregular filenames (e.g., `DowngradeN1a.pdf`, `DowngradeN5b.pdf`, `.PDF` vs `.pdf`) using iterative brute-force logic.
* **Zero-Inference State Tracking**: Maintains a `dgmag_state.sys` file to remember the last intercepted issue, ensuring you never scan the same sector twice.
* **SMTP Integration**: Optimized for secure relays (like **Autistici**) to send immediate alerts with high-fidelity logs.
* **Bash Automation**: Generates a ready-to-use `curl` loop in the email body for one-command batch downloads of newly discovered issues.

## 🛠 Prerequisites

* **OS**: Fedora Linux (Native Bash environment).
* **Language**: Rust (Edition 2021).
* **Dependencies**: `reqwest`, `lettre`, `dotenvy`, `regex`, `anyhow`.

## 🚀 Installation & Setup

1.  **Clone the repository**:
    ```bash
    git clone [https://github.com/your-username/dgmag_checker.git](https://github.com/your-username/dgmag_checker.git)
    cd dgmag_checker
    ```

2.  **Configure Environment**:
    Create a `.env` file in the root directory. 
    **Important**: If your SMTP password contains special characters (like `#`), you **must** wrap it in double quotes.
    ```env
    BASE_URL=[http://dgmag.in](http://dgmag.in)
    EMAIL=your_email@cryptolab.net
    SMTP_SERVER=smtp.autistici.org
    SMTP_PASSWORD="your_secure_password"
    STATE_FILE=dgmag_state.sys
    ```

3.  **Build the binary**:
    ```bash
    cargo build --release
    ```

## 📋 Usage

### Manual Execution
To trigger a manual scan from your Fedora terminal:
```bash
cargo run
```

### Automation (systemd)
To run the scanner daily at 10:00 AM using `systemd --user` units (standard for Fedora power users):

1.  **Create the Service Unit** (`~/.config/systemd/user/dgmag.service`):
    ```ini
    [Unit]
    Description=DG-MAG Covert PDF Scanner
    After=network-online.target

    [Service]
    Type=oneshot
    WorkingDirectory=%h/RustroverProjects/downgrade
    ExecStart=%h/RustroverProjects/downgrade/target/release/dgmag_checker

    [Install]
    WantedBy=default.target
    ```

2.  **Create the Timer Unit** (`~/.config/systemd/user/dgmag.timer`):
    ```ini
    [Unit]
    Description=Run DG-MAG Scanner Daily

    [Timer]
    OnCalendar=*-*-* 10:00:00
    Persistent=true

    [Install]
    WantedBy=timers.target
    ```

3.  **Enable and Start**:
    ```bash
    systemctl --user daemon-reload
    systemctl --user enable --now dgmag.timer
    ```

## 🛡 Security & OpSec

* **Environment Variables**: All sensitive credentials are kept in `.env`. Never commit this file to version control.
* **Git Ignore**: The `.gitignore` file is configured to exclude `target/`, `.env`, and `dgmag_state.sys`.
* **User-Agent Spoofing**: The scanner mimics a standard Chrome client on Fedora to bypass basic server-side bot detection.

---
**END OF LINE.**
