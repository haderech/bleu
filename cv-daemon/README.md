# Clairvoyance Daemon
The Clairvoyance Daemon (CV Daemon) is a tool for crawling Tendermint date.
The app consists of several task plugins to crawl data and a postgresql plugin to store data.
The task plugin is controlled by the JSON-RPC plugin, and when there is an issue in the operation of the task, the issue can be forwarded to the admin through Slack and Email.
The state of each task is synced to RocksDB, and even if the app goes down and restarts, it continues from the synced state.

## Task Plugin
The task plugin is responsible for synchronizing data through crawling.
There are plugins that sync data through Loop and Polling (l2_block_tx, l2_enqueue, l2_state_batch, l2_tx_batch), and 2 plugins that are triggered and operated by the previous plugin (l1_tx_log, l2_tx_receipt) exist.

### Load Task
At startup, each task plugin attempts to read the task state through a function called `task_loader`.
At this time, it checks if there is synced task information in RocksDB, and if there is no saved task state information, the task json file existing in the `root/task` path is loaded to create the first task state.  
The task json has the form below.

```json
{
  "l2_block_tx": {
    "start_idx": 0,
    "end_points": [
      "http://localhost:8545"
    ],
    "filter": ""
  }
}
```
`start_idx` is a parameter value required when fetching data by polling method. Block Height or index values are these.
`end_point` means the end point requesting data, and multiple end points can be input as an array. When requesting polling, the request is made using the first value of the array, and if an error occurs, the request is automatically made to the next end point of the array.
`filter` is used to filter data. "filter": in the form of "to=0xabdc&queue_origin=l1", currently provides four operators: `=`, `()`, `&`, and `|`. The meaning of the preceding filter means that only data in which 'to' is '0xabcd' and the value of 'queue_origin' is 'l1' in json data will be used and the rest will be skipped.

### Control Task
Loop Polling tasks are controlled via JSON-RPC.
There are a total of 4 methods, which are `star_task`, `stop_task`, `remove_task`, and `get_tasks`. `get_tasks` is a method that can check the status of the currently running task, and the rest are methods that control the state of the task.
`start_task` restarts a task that is stopped or a task that is in an error state.
`stop_task` stops the running task.
`remove_task` deletes the task state on RocksDB and stops working. On restart, it is initialized according to the task json.
The above three methods to control the task state all require a task as params.
```json
{
    "jsonrpc": "2.0",
    "id": "1",
    "method": "start_task",
    "params": {
        "task": "l2_block_tx"
    }
}
```
`get_tasks` checks the state of tasks that have been synced so far. No params required.

### Retry Strategy
Unlike the Loop Polling task, the Trigger task works by being triggered by a message that is delivered, so reprocessing is not easy if data synchronization fails.
Therefore, when the Trigger task fails to process, it stores the task in the retry queue and retries for a retry-count.
If it doesn't succeed until the retry-count goes to 0, it sends a 'curl' command to Slack that can be retried.
In this case, retry-endpoint, which is an endpoint for retry requests, can be set in config.toml.

### Polling Interval
The Loop Polling task can adjust the synchronization speed by adjusting the `poll-interval` value in `config.toml`.
```toml
[l2txbatch]
poll-interval=1000
```

### Retry Count
The Trigger task can adjust the number of retries by adjusting the `retry-count` value in `config.toml`.
```toml
[l1txlog]
retry-count=3
retry-endpoint="http://0.0.0.0:9999"
```

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
  "ethereum_tx_logs": {
    "attributes": {
      ...
    },
    "indexes": [ [ "address" ], [ "block_number" ], [ "tx_hash" ], [ "block_hash" ], [ "queue_index" ] ],
    "uniques": [ [ "tx_hash", "log_index" ] ]
  }
}
```

### Loading Schema
postgres plugin executes `load_schema` method to load schema data according to the predefined schema json.
It reads the `optimism.json` and `ethereum.json` files in the `schema` path, and if necessary, if you add the schema file to the `schema_files` array, it can also be read when the plugin starts.
```rust
fn load_schema() -> Result<HashMap<String, PostgresSchema>, ExpectedError> {
    let schema_files = vec![String::from("schema/optimism.json"), String::from("schema/ethereum.json")];
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
Various configuration values required to run the Clairvoyance Daemon are managed in `config.toml`.
These values can also be entered in the form of `--jsonrpc-host 0.0.0.0` at run time.
The path of `config.toml` is located in `~/.config/cv-damon/config`, but the path has been modified so that the project root path can be used in the following executable statements and docker.
When building and executing images with docker, be careful because `config.docker.toml` in the root path is used.

## Run
```shell
RUST_LOG=INFO && cargo run --package cv-daemon --bin cv-daemon -- --config-dir .
```

## Docker
### Build Docker Image
When creating a docker image, `config.docker.toml`, `schema`, `abi`, and `task` in the project folder are used in the docker image. You can add and edit files as needed and then build the image.

```shell
docker build -t cv-daemon .
```

### Run Docker
```shell
docker run -d -p 9999:9999 \
-v /absolute/host/path/task:/cv-daemon/task \
-v /absolute/host/path/schema:/cv-daemon/schema \
-v /absolute/host/path/config.docker.toml:/cv-daemon/config.toml \
--name cv-daemon \
cv-daemon:latest
```
