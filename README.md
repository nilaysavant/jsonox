# jsonox 🛰

CLI based RESTful JSON server + store written in Rust.

## Features 🚀

- Setup **API endpoints** on **any route** by simply **POST**ing `JSON` to that route.
- Endpoints support **GET** for fetching and **DELETE** for deleting stored JSON along with **POST**.
- **JSON** data is stored as `*.json` files under the `json_data` dir.
- View all **active endpoints** on the root (`/`) path.
  
**[DISCLAIMER: This program is designed for development purposes. Use in production at your own risk!]**

## Installation 🔧

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

- Compiled binary will be located at `target/release/jsonox`

### Give Permissions

- Set executable permission:

  ```bash
  chmod +x jsonox
  ```

## Usage 📡

Run the server via the **CLI**, then setup **REST API endpoints**.

### Command line (CLI)

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

  - Use `-h` or `--help` for help.

### REST API

Construct **REST API** endpoints on **arbitrary routes** in the following way(s):

- **POST** the following to `/pets/cat`:

  ```json
  { "cute": true }
  ```

- Then **GET** at `/pets/cat` will receive:

  ```json
  { "cute": true }
  ```

- Similarly you can **DELETE** data stored at `/pets/cat`, this will also receive:

  ```json
  { "cute": true }
  ```

- The above requests will setup files under `./jsonox_data` with the following structure:

  ```bash
  - pets/
    - cat/
      - index.json
  ```

- `GET` on root endpoint `/` will display all active endpoints:

  ```json
  { "active_paths": ["pets/cat"] }
  ```

You can also setup your own API by creating files under `./jsonox_data` in the structure similar as above:

```bash
- pets/
  - dog/
    - index.json
  - cat/
    - index.json
  - index.json
- toys/
  - doll/
    - index.json
```

- Then `GET` on `/` will show active endpoints:

  ```json
  { "active_paths": ["pets", "pets/cat", "pets/dog", "toys/doll"] }
  ```

- You can then do **GET**,**POST** and **DELETE** similarly, on the endpoint paths above.
