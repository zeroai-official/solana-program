
# 0ai.ai solana-program
A Solana program developed using the Anchor framework to publish ZERO AI's cryptocurrency predictions on the Solana.

[web](https://0ai.ai) | [twitter](https://twitter.com/zeroai_official) | [telegram channel](https://t.me/ZEROAI_channel)

## Overview
The transaction speed and low gas fees of the Solana blockchain align perfectly with the goals of ZEROAI. Therefore, we have chosen to deploy our smart contracts on the Solana blockchain. ZEROAI will publish daily price predictions for various cryptocurrencies on-chain, including predictions at the minute, hour, and day levels. This enables users to easily access ZEROAI's predictions both on and off the blockchain. Furthermore, based on the predictive records on the blockchain, we plan to launch some engaging activities in collaboration with the PYTH oracle. Stay tuned for these updates.

## Pre-installation Requirements
Before getting started, ensure your local environment is set up with the following:
* Rust
* Solana CLI
* Anchor
* Yarn

## Installation
### 1. Clone the Repository
Start by cloning the repository to your local machine:
```bash
git clone https://github.com/zeroai-official/zeroai-program.git
```
### 2. Navigate to the Project Directory
After cloning the repository, move into the project directory:
```bash
cd zeroai-program
```

### 3. Generate the Program Key
```bash
solana-keygen new -o program_address.json
```

### 4. Replace Your Program Public Key and Wallet Address
a. In Anchor.toml, update the program public key:
```bash
[programs.localnet]
zeroai_program = "FDkZRiRJapBGTmcr9u8dQtHEk9VbDsb4E9dY4NYPLkJ3"
[programs.devnet]
zeroai_program = "FDkZRiRJapBGTmcr9u8dQtHEk9VbDsb4E9dY4NYPLkJ3"
```
Update the wallet:
```bash
[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"
```

b.  In programs/zeroai-program/src/lib.rs, declare the ID:
```bash
declare_id!("FDkZRiRJapBGTmcr9u8dQtHEk9VbDsb4E9dY4NYPLkJ3");
```

### 5. Build
Compile the ZEROAI program:
```bash
anchor build
```

### 6. Install Dependencies
```bash
yarn install
```

### 57. Testing
To test locally, simply run in the terminal:
```bash
anchor test
```
If successful, you should see the following output:

```bash
  initVault
Admin:  AvGuTZYQ1hapdFSXYkXdte4uZemQU7jxHw5N8fr5TT4h
    ✔ Vault account initialized! (208ms)

  initGames
ZEROAI Instant Win Games
    ✔ Games initialized! (467ms)

  addGames
BTC Price:  712944776
    ✔ Add Game! (459ms)

  eventParser
Signature:  2LGX3jjD9ZySfkAPENFDgknv2WSUhcvetMJAcx9Yg2ukDinAuVYAMTThwwmND3H8Zfh6HNBktsFDgDefpqFb4wae
BTC 71294.4776
ETH 3874.3693
SOL 169.9601
BNB 602.9586
DOGE 0.1775
JUP 0.9147
    ✔ Last Signatures! (1016ms)

  closeGames
    ✔ Close Game! (376ms)


  5 passing (3s)
```

## Deployment
### 1. Pre-deployment Setup
Solana defaults to using the "mainnet" network, so we need to switch to "devnet":
```bash
solana config set --url devnet
```

### 2. Ensure Your Wallet Has Sufficient Balance for Deployment Fees
For instructions on how to create a wallet and receive an airdrop, refer to:
[Solana Wallet Balance Guide](https://www.quicknode.com/guides/solana-development/getting-started/how-to-look-up-the-balance-of-a-solana-wallet)

### 3. Modify the Cluster
In Anchor.toml, change the cluster from "localnet" to "devnet":
```bash
[provider]
cluster = "devnet"
```

### 4. Deploy
```bash
anchor deploy --program-keypair program_address.json --program-name zeroai-program
```
After deployment, you can view your program on Solana Explorer:
https://explorer.solana.com/address/{your program id}?cluster=devnet

### 5. Run
```bash
anchor run test
```

## Contributing
We welcome contributions from data scientists, AI enthusiasts, and financial analysts. Your expertise can help us refine our models and explore new frontiers in AI-driven financial forecasting.