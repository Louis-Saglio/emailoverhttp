# Email over HTTP Service Deployment

## Configuration
The service can be configured using environment variables in the `/etc/emailoverhttp/env` file:
- `PORT`: The port the service listens on (default: 3000).
- `SMTP_SERVER`: The SMTP server address.
- `SMTP_PORT`: The SMTP server port.
- `<USER>_TOKEN`: The token for a specific user.
- `<USER>_SMTP_USERNAME`: The SMTP username for a specific user.

## Build
```bash
cargo build --release
```

## Installation
1. Copy the binary to `/usr/local/bin`:
   ```bash
   sudo cp target/release/emailoverhttp /usr/local/bin/
   ```
2. Create the configuration directory:
   ```bash
   sudo mkdir -p /etc/emailoverhttp
   ```
3. Copy the environment file:
   ```bash
   sudo cp emailoverhttp.env /etc/emailoverhttp/env
   sudo chmod 600 /etc/emailoverhttp/env
   ```
4. Install the systemd service:
   ```bash
   sudo cp emailoverhttp.service /etc/systemd/system/
   ```

## Management
- Start the service: `sudo systemctl start emailoverhttp`
- Enable on boot: `sudo systemctl enable emailoverhttp`
- Check status: `sudo systemctl status emailoverhttp`
- View logs: `journalctl -u emailoverhttp`
