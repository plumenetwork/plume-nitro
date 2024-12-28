## Usage

Building the image locally:

```
git clone https://github.com/plumenetwork/plume-nitro
cd plume-nitro 
git submodule update --init --recursive --force
docker build . -t nitro-plume:0.0.3 --no-cache
```

Pulling the image from Docker Hub:
```
docker pull ghcr.io/conduitxyz/plume-nitro:latest
```

Running the node:

```
Command:
  /usr/local/bin/nitro
Args:
  --http.api=net,web3,eth,debug
  --http.corsdomain=*
  --http.addr=0.0.0.0
  --http.vhosts=*
  --parent-chain.connection.url=http://eth-l1:8545
  --parent-chain.blob-client.beacon-url=http://eth-beacon:3000
  --metrics
  --metrics-server.addr=127.0.0.1
  --metrics-server.port=6070
  --metrics-server.update-interval=5s
  --chain.id=98865
  --chain.info-json=[{"chain-id":98865,"parent-chain-id":1,"chain-name":"conduit-orbit-deployer","chain-config":{"chainId":98865,"homesteadBlock":0,"daoForkBlock":null,"daoForkSupport":true,"eip150Block":0,"eip150Hash":"0x0000000000000000000000000000000000000000000000000000000000000000","eip155Block":0,"eip158Block":0,"byzantiumBlock":0,"constantinopleBlock":0,"petersburgBlock":0,"istanbulBlock":0,"muirGlacierBlock":0,"berlinBlock":0,"londonBlock":0,"clique":{"period":0,"epoch":0},"arbitrum":{"EnableArbOS":true,"AllowDebugPrecompiles":false,"DataAvailabilityCommittee":true,"InitialArbOSVersion":32,"InitialChainOwner":"0xEE6bEc438c70B20A916069508D109DAabA5B4E7E","GenesisBlockNum":0}},"rollup":{"bridge":"0xd53645c6b5e19b3CE2d00bA27d734dCC928FCC54","inbox":"0xC45276467BDb1a9D083010c7CA7Fe2d593a10d01","sequencer-inbox":"0x3fD761A6eFC2137F03f03Da3d46933dD2e6FF0BB","rollup":"0x59EF2FBa6ED4366cb1C3F67f232aaf824B536AB9","validator-utils":"0x84eA2523b271029FFAeB58fc6E6F1435a280db44","validator-wallet-creator":"0x0A5eC2286bB15893d5b8f320aAbc823B2186BA09","deployed-at":20895111}}]
  --chain.name=conduit-orbit-deployer
  --execution.caching.archive
  --execution.forwarding-target=wss://relay-plume-mainnet-0.t.conduit.xyz
  --node.data-availability.enable=true
  --node.data-availability.rest-aggregator.enable=true
  --node.data-availability.rest-aggregator.urls=https://das-plume-mainnet-0.t.conduit.xyz
  --node.staker.enable=false
  --node.feed.input.url=wss://relay-plume-mainnet-0.t.conduit.xyz
```

<br />
<p align="center">
  <a href="https://arbitrum.io/">
    <img src="https://arbitrum.io/assets/arbitrum/logo_color.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">Arbitrum Nitro</h3>

  <p align="center">
    <a href="https://developer.arbitrum.io/"><strong>Next Generation Ethereum L2 Technology »</strong></a>
    <br />
  </p>
</p>

## About Arbitrum Nitro

<img src="https://arbitrum.io/assets/arbitrum/logo_color.png" alt="Logo" width="80" height="80">

Nitro is the latest iteration of the Arbitrum technology. It is a fully integrated, complete
layer 2 optimistic rollup system, including fraud proofs, the sequencer, the token bridges,
advanced calldata compression, and more.

See the live docs-site [here](https://developer.arbitrum.io/) (or [here](https://github.com/OffchainLabs/arbitrum-docs) for markdown docs source.)

See [here](https://docs.arbitrum.io/audit-reports) for security audit reports.

The Nitro stack is built on several innovations. At its core is a new prover, which can do Arbitrum’s classic
interactive fraud proofs over WASM code. That means the L2 Arbitrum engine can be written and compiled using
standard languages and tools, replacing the custom-designed language and compiler used in previous Arbitrum
versions. In normal execution,
validators and nodes run the Nitro engine compiled to native code, switching to WASM if a fraud proof is needed.
We compile the core of Geth, the EVM engine that practically defines the Ethereum standard, right into Arbitrum.
So the previous custom-built EVM emulator is replaced by Geth, the most popular and well-supported Ethereum client.

The last piece of the stack is a slimmed-down version of our ArbOS component, rewritten in Go, which provides the
rest of what’s needed to run an L2 chain: things like cross-chain communication, and a new and improved batching
and compression system to minimize L1 costs.

Essentially, Nitro runs Geth at layer 2 on top of Ethereum, and can prove fraud over the core engine of Geth
compiled to WASM.

Arbitrum One successfully migrated from the Classic Arbitrum stack onto Nitro on 8/31/22. (See [state migration](https://developer.arbitrum.io/migration/state-migration) and [dapp migration](https://developer.arbitrum.io/migration/dapp_migration) for more info).

## License

Nitro is currently licensed under a [Business Source License](./LICENSE.md), similar to our friends at Uniswap and Aave, with an "Additional Use Grant" to ensure that everyone can have full comfort using and running nodes on all public Arbitrum chains.

The Additional Use Grant also permits the deployment of the Nitro software, in a permissionless fashion and without cost, as a new blockchain provided that the chain settles to either Arbitrum One or Arbitrum Nova.

For those that prefer to deploy the Nitro software either directly on Ethereum (i.e. an L2) or have it settle to another Layer-2 on top of Ethereum, the [Arbitrum Expansion Program (the "AEP")](https://docs.arbitrum.foundation/assets/files/Arbitrum%20Expansion%20Program%20Jan182024-4f08b0c2cb476a55dc153380fa3e64b0.pdf) was recently established. The AEP allows for the permissionless deployment in the aforementioned fashion provided that 10% of net revenue (as more fully described in the AEP) is contributed back to the Arbitrum community in accordance with the requirements of the AEP.

## Contact

Discord - [Arbitrum](https://discord.com/invite/5KE54JwyTs)

Twitter: [Arbitrum](https://twitter.com/arbitrum)
