# SurrealDB Project with Actix-Web

This project demonstrates a basic Actix-Web server interfacing with SurrealDB to insert and query person records. It uses Rust's powerful async capabilities alongside SurrealDB's flexible schema-less data storage.

## Getting Started

### Prerequisites

- Rust Programming Language
- SurrealDB installed locally

### Running SurrealDB

Start your SurrealDB instance with the following command:

```sh
surreal start file:demo-db --user root --password root
```

This command initializes a new SurrealDB instance using file-based storage.

### Setting Up the Project

1. **Clone the repository**

   ```sh
   git clone https://github.com/ASoldo/surrealdb-proj.git
   cd surrealdb-proj
   ```

2. **Build the project**:

   ```sh
   cargo build
   ```

3. **Start the SurrealDB**

   ```sh
   surreal start file:db-demo --user root --password root
   ```

4. **Run the Actix-Web server**:

   ```sh
   cargo run
   ```

The server will start on `http://127.0.0.1:8080`.

### API Endpoints

The server exposes two endpoints:

1. **Insert Person**: Inserts a new person record into the database.

   ```sh
   curl -v 127.0.0.1:8080/insert_person
   ```

   Response:

   ```json
   [
     {
       "id": { "tb": "person", "id": { "String": "up0a5uelp375nziy2lx8" } },
       "marketing": true,
       "name": "Rootster",
       "title": "Founder & CEO"
     }
   ]
   ```

2. **Query Person**: Retrieves all person records from the database.

   ```sh
   curl 127.0.0.1:8080/query_person
   ```

   Response:

   ```sh
   Query result: [Object {"id": Object {"tb": String("person"), "id": Object {"String": String("up0a5uelp375nziy2lx8")}}, "marketing": Bool(true), "name": String("Rootster"), "title": String("Founder & CEO")}]
   ```

### Dependencies

The project's dependencies are defined in `Cargo.toml`:

- `actix-web` for the web server framework.
- `serde` and `serde_json` for serializing and deserializing the JSON data.
- `surrealdb` for interacting with the SurrealDB database.

### Start TiKV

```bash
tiup playground --tag surrealdb --mode tikv-slim --pd 1 --kv 1
```

or

```bash
tiup playground --tag surrealdb --pd 1 --kv 1
```

### Start SurrealDB with TiKV

```bash
surreal start --log trace tikv://127.0.0.1:2379
```

### Start SurrealDB with local db file

```bash
surreal start file:demo-db --user root --password root
```

### Connect to local SurrealDB

```bash
surreal sql --endpoint http://127.0.0.1:8000 --username root --password root --namespace test --database test
```

### Profiling

Use to profile application and generate flamegraph

```bash
cargo flamegraph --dev -- --no-rosegment
```
