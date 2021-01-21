# jsonox

CLI based RESTful JSON server + store written in Rust.

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

### Or Build From Source

- Clone the repository and run:

  ```bash
  cargo build --lock --release
  ```

### Give Permissions

- Set executable permission:

  ```bash
  chmod +x jsonox
  ```

- Compiled binary will be located at `target/release/jsonox`

## Examples

### Command line

- Simple server with logging:

  ```bash
  ./jsonox
  ```

  - Runs at:
    - http://localhost:8080/
    - http://127.0.0.1:8080/
    - http://0.0.0.0:8080/

- Specify custom bind address:

  ```bash
  ./jsonox -b localhost:7000
  ```

  - Use `-b` or `--bind-addr`
  - Address format: `<IP:PORT>`

- Disable logging:

  ```bash
  ./jsonox --quiet
  ```

  - Use `-q` or `--quiet` for quiet mode.

- View help and guide:

  ```bash
  ./jsonox --help
  ```

  - Use `-h` or `--help` for quiet mode.
