# roggen

Basic blog written in rust as a project to learning some of the language
features. 

Prerequisites:
- Cargo
- Diesel
- PostgreSQL

### Initialization of the database

```bash
diesel setup

echo 'DATABASE_URL="postgres://[USERNAME]:[PASSWORD]@localhost/diesel_demo"' > .env
echo 'IMAGE_FOLDER="/path/to/images/folder"' >> .env
```

### Running the server

```bash
# Run the server locally as debug executable
cargo run --bin run_server

# Run the server in release mode accepting external connections
cargo run --release --bin run_server
```

### Developed

Keep in mind that this software is still in early development. As soon
as it reaches a state where i can safely launch it without risking
meltdowns i'll host an instance on my server and (hopefully) publish
post on a regular basis.