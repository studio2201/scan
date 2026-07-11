# Scan — Orbital Sector Scanner <img src="https://raw.githubusercontent.com/etecoons/unraid-apps/main/icons/scan.png" width="48" height="48" alt="scan logo" align="right">

Scan is a clean, secure, and optimized planetary hazard sector scanner (Minesweeper clone) built in Rust and WebAssembly, served by a high-performance Axum backend.

---

## 🏛️ Architecture & Stack
*   **Frontend**: Yew (WASM)
*   **Backend**: Axum (Rust) / Tokio
*   **Deployment**: UBI container (Red Hat UBI9) on Docker Hub / Unraid / Podman / Docker Compose

---

## 🟢 Key Features
*   **Standardized UI Alignment**: Completely integrated with `shared-assets` for a uniform theme engine, navigation header, footer, and authentication layout.
*   **Orbital Sector Environments**: Selectable atmospheric navigation sectors (Alpha, Beta, Gamma, Delta, Epsilon, Zeta) with custom sci-fi HUD color schemes.
*   **Classic Scanning Rules**: Geothermal hazard (mine) sweep validation, adjacent count warnings, beacon placement (flags), and custom difficulty presets.
*   **Secure PIN Access**: Optional lock screen gate with client IP rate-limiting, timing-attack protections, and session cookie validation.
*   **Performance First**: Tiny resource footprint, zero external JS engine dependencies, and rapid page load speeds.

---

## 💾 Deployment & Installation

### Container images (Docker Hub)

Images are **UBI9-minimal** based (Red Hat Universal Base Image). Tags:

| Tag | Meaning |
| :--- | :--- |
| `latest` | Current recommended build |
| `ubi` | Explicit UBI image (same lineage as `latest`) |
| `0.2.3` | Immutable release pin |

```bash
# Pull examples
podman pull docker.io/etecoons/scan:latest
podman pull docker.io/etecoons/scan:ubi
podman pull docker.io/etecoons/scan:0.2.3
```

Hub: [https://hub.docker.com/r/etecoons/scan](https://hub.docker.com/r/etecoons/scan)

### Docker Compose
Create a `docker-compose.yml` file with the following service definition:

```yaml
services:
  scan:
    image: etecoons/scan:latest
    container_name: scan
    restart: unless-stopped
    volumes:
      - ${SCAN_DATA_PATH:-./data}:/app/data
    ports:
      - ${PORT:-4503}:4503
    environment:
      PORT: 4503
      BASE_URL: ${SCAN_BASE_URL:-http://localhost:4503}
      SCAN_PIN: ${SCAN_PIN:-}
      ALLOWED_ORIGINS: ${SCAN_ALLOWED_ORIGINS:-*}
      MAX_ATTEMPTS: ${MAX_ATTEMPTS:-5}
      SITE_TITLE: ${SCAN_SITE_TITLE:-Scan}
      ENABLE_TRANSLATION: ${ENABLE_TRANSLATION:-true}
      ENABLE_THEMES: ${ENABLE_THEMES:-true}
      ENABLE_PRINT: ${ENABLE_PRINT:-true}
      TZ: ${TZ:-UTC}
```

### Build the UBI image locally

Requires [Podman](https://podman.io/) (or Docker) and network access to pull base images and crates.

```bash
# From the repository root
podman build --format docker -f Containerfile.ubi \
  -t docker.io/etecoons/scan:0.2.3 \
  -t docker.io/etecoons/scan:latest \
  -t docker.io/etecoons/scan:ubi \
  .

# Optional: push all three tags
podman push docker.io/etecoons/scan:0.2.3
podman push docker.io/etecoons/scan:latest
podman push docker.io/etecoons/scan:ubi
```

---

## ⚙️ Configuration Options

| Environment Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4503` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Scan` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies. | `http://localhost:4503` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). | `*` |
| `SCAN_PIN` | Optional 4–10 digit PIN (numerical only) to lock access to the interface. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header. | `true` |
| `ENABLE_THEMES` | Enable the theme selector in the navigation header. | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header. | `true` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before rate lockout. | `5` |
| `LOCKOUT_TIME_MINUTES` | Lockout duration in minutes for IPs exceeding `MAX_ATTEMPTS`. | `15` |
| `COOKIE_MAX_AGE_HOURS` | Duration in hours that the user's PIN session cookie remains valid. | `24` |
| `SHUTDOWN_DRAIN_SECONDS` | Seconds to wait for active connections to finish before shutting down. | `5` |
| `SHOW_VERSION` | Display the application version number in the footer. | `true` |
| `SHOW_GITHUB` | Display the GitHub repository link in the footer. | `true` |
| `TRUST_PROXY` | Set `true` if backend is hosted behind a reverse proxy. | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated IP/CIDR list of trusted upstream proxies. | None |

---

## 🛠️ Local Development

Ensure you have the Rust toolchain and Trunk installed.

```bash
# 1. Run workspace tests
cargo test

# 2. Run clippy workspace checks
cargo clippy --workspace --all-targets

# 3. Start frontend Yew dev server (from frontend/)
cd frontend && trunk serve

# 4. Start backend Axum server (from backend/)
cd backend && cargo run
```

---

## 📄 License
Licensed under the [Apache License, Version 2.0](LICENSE). Copyright 2026 etecoons.
