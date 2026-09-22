# webfetch-server

An HTTP service that renders webpages with a shared Chromium instance and returns either Markdown or screenshots split into vertical frames.

## Disclaimer

Intended for **local use only**; do not expose it to untrusted networks:

- **No SSRF protections**: URLs are fetched without restrictions, so callers can reach internal/localhost services and cloud metadata endpoints.
- **Lax resource limiting**: Viewport, frame, and image dimensions are only loosely bounded; requests can consume significant memory, CPU, and disk.
- **No authentication or rate limiting**: Any process that can reach the socket can drive the browser.
- **Arbitrary page content**: Chromium executes fetched pages; treat it as untrusted and sandbox it (container, dedicated user, network policy).

## Endpoints

All routes are nested under `/v1`. Requests are limited by a semaphore (503 when busy) and a global timeout.

### `GET /v1/status`

Health check; returns `OK`.

### `GET /v1/markdown`

Navigates to `url` in Chromium, extracts the rendered HTML, and converts it to Markdown.

| Param | Default | Description |
|---|---|---|
| `url` | required | Page URL |
| `vw`, `vh` | 1920, 1080 | Viewport size |
| `timeout_ms` | 10000 | Per-request timeout |

Response: `{"markdown": "..."}`

### `GET /v1/screenshot`

Captures a full-page screenshot, resizes it, and splits it into vertical frames written to disk.

| Param | Default | Description |
|---|---|---|
| `url` | required | Page URL |
| `vw`, `vh` | 1920, 1080 | Viewport size |
| `ow` | 720 | Output width after resize |
| `frames` | 5 | Max number of vertical frames |
| `format` | `webp` | `jpeg`, `png`, or `webp` |
| `timeout_ms` | 10000 | Per-request timeout |

Response: `{"frames": ["/tmp/webfetch/.../frame_0.webp", ...]}`

## Usage

```sh
webfetch-server \
  --chromium-path /usr/bin/chromium \
  --user-data-dir /tmp/chromium-profile \
  --incognito
```

| Option | Default | Description |
|---|---|---|
| `--addr` | `127.0.0.1:3382` | Server socket address |
| `--concurrency` | 20 | Max concurrent requests |
| `--timeout-ms` | 20000 | Request timeout |
| `--chromium-path` | required | Path to the Chromium executable |
| `--user-data-dir` | required | Path to the Chromium profile directory |
| `--headless` | off | Run the browser in headless mode |
| `--incognito` | off | Run the browser in incognito mode |
| `--max-viewport` | 50000000 | Max viewport width times height (px) |
| `--image-dir` | `/tmp/webfetch` | Directory for generated images |
| `--max-frames` | 20 | Max vertical frames per page |

## Building

Requires libvips and a Chromium executable at runtime.

```sh
cargo build --release
```

## Tests

Unit tests: `cargo test`. Integration tests are ignored and require environment variables:

- `tests/browser.rs`: `WEBFETCH_TEST_CHROMIUM`, `WEBFETCH_TEST_PROFILE`
- `tests/image.rs`: `WEBFETCH_TEST_IMAGE`

Run with `cargo test -- --ignored`.

## Architecture

Axum handlers validate requests and send them over a bounded mpsc channel to a single browser instance that manages Chromium pages (one per request, via chromiumoxide). Results return over oneshot channels; HTML is converted to Markdown with htmd, while screenshots are post-processed (resize, vertical split) with libvips. Concurrency is capped by a semaphore shared across endpoints.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
