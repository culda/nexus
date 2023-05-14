# Nexus - defi like a boss

A CLI tool for buying/selling tokens on EVM chains using the 1INCH Aggregator protocol.

![ladys](https://github.com/culda/nexus/assets/48595067/f288aa64-50d0-4d3f-8ff9-2cb6fe403825)

**Warning**
This is an untested tool, bad things can happen. Always use small amounts to test first

Future plans:

- airdrop farming. i.e use destReceiver from 1INCH to receive swap tokens directly into wallets generated from mnemonic
- more advanced order types like TWAP for larger positions
- volume/price alerts and automatic execution
- whatever else I think about

## Installation

1. Create an `.env` file using `.env.template` as an example.

2. Install rustup

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Build binary

```
cargo build --release
```

4. Add binary to PATH and enjoy

Assuming you cloned the repo at `~/nexus`, add the following to `.bashrc` or `.zshrc` depending on OS

```
export PATH=$PATH:~/nexus/target/release
```

### Example commands

Swap 200 ARB into ETH on arbitrum

```
nexus sell -t 0x912CE59144191C1204E64559FE8253a0e49E6548 -a 200 -c arbitrum --allowmax
```

Swap 1 ETH into ARB on arbitrum

```
nexus buy -a 1 -t 0x912ce59144191c1204e64559fe8253a0e49e6548 -c arbitrum --allowmax
```

Swap 1 WETH for MUTE on zksync

```
nexus buy -a 1 -t 0xa49d7499271ae71cd8ab9ac515e6694c755d400c -c zksync --allowmax -n false
```

### Tips

If you need to modify the API swagger file, use this to generate new bindings:

```
openapi-generator-cli generate --package-name inch_api -i inch-api/v5-swagger.json -g rust -o inch-api
```
