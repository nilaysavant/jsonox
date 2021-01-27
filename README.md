# jsonox 🛰 ![Build Release](https://github.com/nilaysavant/jsonox/workflows/Build%20Release/badge.svg?branch=v0.1.0)

CLI based RESTful JSON server + store written in Rust.

## Features 🚀

- Setup **API endpoints** on **any route** by simply **POST**ing `JSON` to that route.
- Endpoints support **GET** for fetching and **DELETE** for deleting stored JSON along with **POST**.
- **JSON** data is stored as `*.json` files under the `json_data` dir.
- View all **active endpoints** on the root (`/`) path.

**[DISCLAIMER: This program is designed for development purposes. Use in production at your own risk!]**

## Installation 🔧

You can install in 3 ways: Using pre-compiled binary, from Crate or by manually building from source using rust tool-chain. Give necessary executable permissions for the binary and if building from source.

### Pre-compiled Binary

- Download binary for your platform from the latest [release](https://github.com/nilaysavant/jsonox/releases).

  | Binary               | Platform                                  |
  | -------------------- | ----------------------------------------- |
  | jsonox-linux-amd64   | 64-bit Linux (Ubuntu, Debian etc)         |
  | jsonox-macos-amd64   | 64-bit Mac OS                             |
  | jsonox-win-amd64.exe | 64-bit Windows 7+                         |
  | jsonox-linux-armv7   | ARMv7 Linux: Raspberry PI, Debian, Ubuntu |

### Install from [Crate](https://crates.io/crates/jsonox)

- Use cargo install:

  ```bash
  cargo install jsonox
  ```

### Or Build From Source

- Clone the repository and run:

  ```bash
  cargo build --lock --release
  ```

- Compiled binary will be located at `target/release/jsonox`

### Initial Setup (when installing from binary/source)

- Set executable permission:

  ```bash
  chmod +x jsonox
  ```

- Copy binary inside your `$PATH` directory (optional):

  ```bash
  cp jsonox ~/.local/bin/ #for linux
  ```

## Usage 📡

Run the server via the **CLI**, then setup **REST API endpoints** or if using in **Read Only** mode.

### Command line (CLI)

Note: In the following examples you may need to use `./jsonox` if using local binary.

- Simple server with logging:

  ```bash
  jsonox
  ```

  - Runs at:
    - http://localhost:8080/
    - http://127.0.0.1:8080/
    - http://0.0.0.0:8080/

- Specify custom bind address:

  ```bash
  jsonox -b localhost:7000
  ```

  - Use `-b` or `--bind-addr`
  - Address format: `<IP:PORT>`

- Disable logging:

  ```bash
  jsonox --quiet
  ```

  - Use `-q` or `--quiet` for quiet mode.

- Use **ReadOnly** mode:

  ```bash
  jsonox --read-only
  ```

  - Use `-r` or `--read-only` for read-only mode.

- View help and guide:

  ```bash
  jsonox --help
  ```

  - Use `-h` or `--help` for help.

### REST API (normal mode)

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

### Read Only Mode

In this mode, jsonox _only reads the json files stored_ and **does NOT create/delete** them in case of **POST/DELETE** unlike in the normal mode explained above. This is useful when you **only need to simulate API responses** and when your back-end does not strictly follow the REST standards. You can also record the `./jsonox_data` in your version control to store your API response structures as it won't change based on the simulations/testing in this mode.

- Start by creating files in `./jsonox_data`:

  ```bash
  - pets/
    - dog/
      - get.json
      - post.json
    - cat/
      - get.json
    - get.json
    - delete.json
  - toys/
    - doll/
      - get.json
      - post.json
      - put.json
      - delete.json
  ```

  - In **Read Only** mode we create files like `get.json`, `post.json`, `put.json`, and `delete.json`, instead of `index.json`.
  - `get.json` will contain the **response body** for **GET** requests to that path. Similarly `post.json`, `put.json`, and `delete.json` will contain the **response body** for **POST**, **PUT**, and **DELETE** requests to that path respectively.

- The files and paths created by you will not be deleted even if you do a **DELETE** on a path.
- If you change modes in between, and do a **DELETE** in **normal mode**, this will _only delete `index.json` files at the respective paths_ and NOT delete the other `get.json`, `post.json` etc files created by you.

- Similar to normal mode `GET` on `/` will show active endpoints:

  ```json
  { "active_paths": ["pets", "pets/cat", "pets/dog", "toys/doll"] }
  ```
