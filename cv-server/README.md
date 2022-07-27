# Clairvoyance Server
Clairvoyance Server is a backend that provides the data collected by the Clairvoyance Daemon in the form required by the user.
It was developed based on `rust`, `actix-web`, and `diesel`, and supports Swagger using `paperclip`.

## Actix Web
- [actix-web](https://actix.rs/)
- Actix Web is a web framework that works on the basis of Actix.
  Actix Web has always been at the top of the Web Framework Benchmark, and it is so easy to use that developers who have used other Web Frameworks can easily handle it.

### Server Configuration
You can set the binding url in Server Configuration.
It requires two values, `host` and `port`, and can be set to `SERVER_HOST` and `SERVER_PORT` in `.env` in the root path.
```
# BLEU SERVER CONFIG
SERVER_HOST=0.0.0.0
SERVER_PORT=8888
```

## Diesel
- [diesel](https://diesel.rs/)
- Diesel is an ORM that helps the server interact with the database.
  Clairvoyance uses PostgreSQL as its data store, and Clairvoyance Server accesses PostgreSQL DB through Diesel.
  Internally, the connection pool of r2d2 is used, and if you want to use a customized setting value other than the default setting value, you can change the setting value when initializing the pool in `PostgresConfig`.

### PostgreSQL Configuration
You can set the URL for PostgreSQL DB access by modifying the `POSTGRES_URL` in `.env`.
```
# POSTGRESQL DB
POSTGRES_URL=postgres://root:postgresql@localhost:5432/postgres
```

## Swagger
### Paperclip
- [paperclip](https://paperclip.waffles.space/)
- Paperclip is a generator that analyzes HTTP API and generates OpenAPI code.
  You can check the generated code in the `{server_url}/api/spec` path.

### Swagger Configuration
In Swagger Configuration, information such as `title`, `version`, and `tags` required when generating OpenAPI code is set.

### Swagger UI
It provides a Swagger UI where you can check the details of the HTTP API provided by the server and actually make a request to the API.
Files composing Swagger UI exist in root's swagger-ui path.
The URL to access the Swagger UI is `{server_url}/swagger`.

## Environment Parameters
Clairvoyance Server requires you to enter a few environment variables before running and you can simply use `.env` to handle it.
The `.env` contains the values needed to run the server and the PostgreSQL access endpoint.
If `.env` is not included in the root path, you can refer to `.env.example` and be careful because docker uses `.env.docker`.

## Run
```shell
RUST_LOG=INFO && cargo run --package cv-server --bin cv-server
```

## Docker
### Build Docker Image
When creating a docker image, `.env.docker` and `swagger-ui` in the project folder are used in the docker image. You can add and edit files as needed and then build the image.

```shell
docker build --tag cv-server .
```

### Run Docker
```shell
docker run -d -p 8888:8888 \
-v /absolute/host/path/.env.docker:/cv-server/.env \
--name cv-server \
cv-server:latest
```
or
```shell
docker run -d -p 8888:8888 \
-e POSTGRES_URL=postgres://root:postgresql@host.docker.internal:5432/postgres \
-e SERVER_HOST=0.0.0.0 \
-e SERVER_PORT=8888 \
--name cv-server \
cv-server:latest
```
