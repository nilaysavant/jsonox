# jsonox

RESTful JSON server + store written in Rust.

## Features

- Setup **API endpoints** on **any route** by simply **POST**ing `JSON` to that route.
- Endpoints support `GET` for fetching and `DELETE` for deleting stored JSON.
- JSON data is stored as `*.json` files under the `json_data` dir.
- View active endpoints on the root (`/`) path.

## Installation

### Pre-compiled Binary

- Download binary for your platform from the latest [release](https://github.com/nilaysavant/jsonox/releases).

  | Binary               | Platform                                  |
  | -------------------- | ----------------------------------------- |
  | jsonox-linux-amd64   | 64-bit Linux (Ubuntu, Debian etc)         |
  | jsonox-macos-amd64   | 64-bit Mac OS                             |
  | jsonox-win-amd64.exe | 64-bit Windows 7+                         |
  | jsonox-linux-armv7   | ARMv7 Linux: Raspberry PI, Debian, Ubuntu |

### Build From Source

- Clone the repository and run:

  ```bash
  cargo build --lock --release
  ```

- Compiled binary will be located at `target/release/jsonox`
