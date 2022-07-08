# Bleu

**BLockchain Explorer sUite**

Bleu is a collection of tools for building blockchain explorer.

## Configurations

### bleu-app config

- See an example config at [`.env.example`](bleu-app/.env.example) copy into a .env file before running.

- [`consts.ts`](bleu-app/src/utils/consts.ts)

```ts
export const L1ExplorerEndpoint = RINKEBY OR ETHEREUM MAINNET EXPLORER;
export const BleuServerEndpoint = BLEU SERVER ENDPOINT (DEFAULT PORT:8888);
export const L2JsonRpcEndpoint  = L2 GETH RPC ENDPOINT;

export const MainPageAutoRefresh = true;
```

### bleu-server config

- See an example config at [`.env.example`](bleu-server/.env.example) copy into a .env file before running.

- Enter the Postgres id and password.

```shell
# POSTGRESQL DB
POSTGRES_URL=postgres://[ID:PASSWORD]@localhost:5432/postgres

# BLEU SERVER CONFIG
SERVER_HOST=0.0.0.0
SERVER_PORT=8888
```

### bleu-daemon config

- bleu-daemon uses the url of the json file in the [`task`](bleu-daemon/task) folder. Open all the json files inside the [`task`](bleu-daemon/task) folder and check the settings.

- Currently, three urls are used for the json file in the task folder.

  - L1 : RINKEBY OR ETHEREUM RPC ENDPOINT (eg: infura)
  - L2 : DARIUS or OPTIMISM RPC ENDPOINT
  - DTL : DTL RPC ENDPOINT (Includes Postfix URL Path)

```json
// l2_block_tx.json
{
  "l2_block_tx": {
    "start_idx": 0,
    "end_points": [
      [L2 GETH RPC ENDPOINT]
    ],
    "filter": ""
  }
}

// l2_enqueue.json
{
  "l2_enqueue": {
    "start_idx": 0,
    "end_points": [
      "[DTL RPC ENDPOINT]/enqueue/index/"
    ],
    "filter": ""
  }
}

```

## License

[AGPL-3.0](https://github.com/turnpike/bleu/blob/main/LICENSE)
