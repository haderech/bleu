# Bleu Daemon
The Bleu Daemon is a tool for crawling evm machine.
The app consists of several sync plugins to crawl data and a postgresql plugin to store data.
The sync plugin is controlled by the JSON-RPC plugin, and when there is an issue in the operation of the task, the issue can be forwarded to the admin through Slack and Email.
The state of each task is synced to file, and even if the app goes down and restarts, it continues from the synced state.

## Sync Plugin
The task plugin is responsible for synchronizing data through crawling.
There are a plugin that sync data through Loop and Polling (ethereum_block), and other plugin that are triggered and operated by the previous plugin (ethereum_tx_receipt) exist.

### Load Sync State
At startup, each sync plugin attempts to read the sync state through a function called `load_state`.
At this time, it checks if there is synced state in `root/state`, and if there is no saved sync state information, the sync json file existing in the `root/sync` path is loaded to create the first sync state.  
The sync json has the form below.

```json
{
  "sync_type": "ethereum_block",
  "chain_name": "ethereum",
  "chain_id": "42",
  "from_idx": 0,
  "endpoints": [
    "http://localhost:9933"
  ],
  "filter": ""
}
```
`from_idx` is a parameter value required when fetching data by polling method. Block Height or index values are these.
`endpoints` means the end point requesting data, and multiple end points can be input as an array. When requesting polling, the request is made using the first value of the array, and if an error occurs, the request is automatically made to the next end point of the array.
`filter` is used to filter data. "filter": in the form of "to=0xabdc|from=0x1234", currently provides four operators: `=`, `()`, `&`, and `|`. The meaning of the preceding filter means that only data in which 'to' is '0xabcd' or the value of 'from' is '0x1234' in json data will be used and the rest will be skipped.

### Control Sync
Loop Polling tasks are controlled via JSON-RPC.
There are a total of 4 methods, which are `start_sync`, `stop_sync` and `get_sync`. `get_sync` is a method that can check the status of the currently running sync state, and the rest are methods that control the state of the sync.
`start_sync` restarts a sync that is stopped or a sync that is in an error state.
`stop_sync` stops the sync.
The above three methods to control the task state all require a task as params.
```json
{
    "jsonrpc": "2.0",
    "id": "1",
    "method": "start_sync",
    "params": {
        "sync_type": "ethereum_block"
    }
}
```
`get_sync` checks the state of a sync that have been synced so far.

## PostgreSQL Plugin
The postgres plugin is a plugin responsible for storing PostgreSQL DB data.
Data crawled in task is delivered in message form to postgres plugin along with schema name, and postgres plugin saves data by changing data into insert query according to predefined schema and executing the query.
Since `$field_name$` is replaced with an actual value in the process of creating an insert query, care must be taken to ensure that there is no data stored in that form.

### Defining Schema
The schema follows the rules of JSON Schema.
The schema has schema name as the key, and has an object called `attributes` whose value represents the actual schema configuration.
`attributes` consists of objects whose key is column name.
An item in `attributes` consists of a type and a description.
The types allowed in JSON Schema are `string`, `integer`, `number`, `boolean`, `object`, and `array`, and nullable is indicated as follows. ['string', 'null']
The description means the field value in the data, and it can be viewed as a key that matches the field value to the column of the DB table.
For example, in the `l2_tx_block` that gets the tx data of L2 geth, there is a field called ‘from’ in tx. However, when saving to DB, it is saved as `from_address`, so it has the form below.
```json
 "optimism_block_txs": {
    "attributes": {
      ...
      "from_address": {
        "type": [ "string", "null" ],
        "description": "from"
      },
      ...
    },
    "indexes": ...,
    "uniques": ...
  }
```
`indexes` is a field to add an index to the column. It has an array in an array, and the sub-array consists of column names. This allows you to create multi-column indexes.
`uniques` is a field for adding a unique constraint to a column. It has the same format as `indexes`, and you can also add multi-column unique conditions.
```json
{
  "ethereum_blocks": {
    "attributes": {
      ...
    },
    "indexes": [ [ "address" ], [ "block_number" ], [ "tx_hash" ], [ "block_hash" ] ],
    "uniques": [ [ "tx_hash", "log_index" ] ]
  }
}
```

### Loading Schema
postgres plugin executes `load_schema` method to load schema data according to the predefined schema json.
It reads the `ethereum.json` files in the `schema` path, and if necessary, if you add the schema file to the `schema_files` array, it can also be read when the plugin starts.
```rust
fn load_schema() -> Result<HashMap<String, PostgresSchema>, ExpectedError> {
    let schema_files = vec![String::from("schema/ethereum.json")];
    ...
    Ok(schema_map)
}
```

### Plugin Configuration
The postgres plugin requires `host`, `port`, `dbname`, `user`, and `password` settings for PostgreSQL DB access.
These values can be entered through config.toml.
```toml
[postgres]
host="localhost"
port="5432"
dbname="postgres"
user="root"
password="postgresql"
```

## Slack Plugin
The slack plugin serves to deliver the log generated during operation to the admin.

### Slack Webhook
1. create workspace and add channel
- [Create Workspace Guide](https://slack.com/intl/en-kr/help/articles/206845317-Create-a-Slack-workspace)
- [Create Channel Guide](https://slack.com/intl/en-kr/help/articles/201402297-Create-a-channel)

2. add slack app and activate incoming webhooks
- [Setup App Guide](https://api.slack.com/authentication/basics)
- [Setup Incoming Hooks Guide](https://api.slack.com/messaging/webhooks)

3. enter webhook url into config.toml
```toml
[slack]
activate=true
info="https://hooks.slack.com/services/..."
warn="https://hooks.slack.com/services/..."
error="https://hooks.slack.com/services/..."
```

### Activating Slack
To activate the slack plugin, you need to change slack activate in config.toml to true.
```toml
[slack]
activate=true
...
```

## config.toml
Various configuration values required to run the Bleu Daemon are managed in `config.toml`.
These values can also be entered in the form of `--jsonrpc-host 0.0.0.0` at run time.
The path of `config.toml` is located in `~/.config/bleu-damon/config`, but the path has been modified so that the project root path can be used in the following executable statements and docker.
When building and executing images with docker, be careful because `config.docker.toml` in the root path is used.

## Run
```shell
RUST_LOG=INFO && cargo run --package bleu-daemon --bin bleu-daemon -- --config-dir .
```

## Docker
### Build Docker Image
When creating a docker image, `config.docker.toml`, `schema` and `sync` in the project folder are used in the docker image. You can add and edit files as needed and then build the image.

```shell
docker build -t bleu-daemon .
```

### Run Docker
```shell
docker run -d -p 9999:9999 \
-v /absolute/host/path/task:/bleu-daemon/sync \
-v /absolute/host/path/schema:/bleu-daemon/schema \
-v /absolute/host/path/config.docker.toml:/bleu-daemon/config.toml \
--name bleu-daemon \
bleu-daemon:latest
```