# Bonkfun trading bot | Bonkfun bundler | Bonkfun volume bot | Bonkfun sniper SDK

This project is a BonkSwap bundler that allows you to interact with the BonkSwap protocol on Solana. It provides an SDK for creating pools, adding liquidity, swapping tokens, and more, using the BonkSwap on-chain program.

## Features
- Create and manage BonkSwap pools
- Add liquidity and become a provider
- Swap tokens using the BonkSwap AMM
- Withdraw fees and rewards
- Fully typed TypeScript SDK

## Usage Example

```typescript
import { BonkSwapSDK } from "./src";
import { AnchorProvider } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";
import { BN } from "bn.js";

const connection = new Connection("https://api.devnet.solana.com");
const wallet = Keypair.generate(); // Replace with your wallet
const provider = new AnchorProvider(connection, wallet, {});

const sdk = new BonkSwapSDK(provider);

async function main() {
  const accounts = {
    state: new PublicKey("STATE_PUBKEY_HERE"),
    pool: new PublicKey("POOL_PUBKEY_HERE"),
    tokenX: new PublicKey("TOKEN_X_PUBKEY_HERE"),
    tokenY: new PublicKey("TOKEN_Y_PUBKEY_HERE"),
    poolXAccount: new PublicKey("POOL_X_ACCOUNT_PUBKEY_HERE"),
    poolYAccount: new PublicKey("POOL_Y_ACCOUNT_PUBKEY_HERE"),
    adminXAccount: new PublicKey("ADMIN_X_ACCOUNT_PUBKEY_HERE"),
    adminYAccount: new PublicKey("ADMIN_Y_ACCOUNT_PUBKEY_HERE"),
    admin: wallet.publicKey,
    projectOwner: new PublicKey("PROJECT_OWNER_PUBKEY_HERE"),
    programAuthority: new PublicKey("PROGRAM_AUTHORITY_PUBKEY_HERE"),
    systemProgram: new PublicKey("11111111111111111111111111111111"),
    tokenProgram: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    rent: new PublicKey("SysvarRent111111111111111111111111111111111"),
  };

  const args = {
    lpFee: new BN(100),
    buybackFee: new BN(50),
    projectFee: new BN(25),
    mercantiFee: new BN(25),
    initialTokenX: new BN(1000000),
    initialTokenY: new BN(1000000),
    bump: 255,
  };

  const tx = await sdk.createPool(accounts, args);
  console.log("Create pool transaction signature:", tx);
}

main().catch(console.error);
```
## Contact

#### Discord: caterpillardev

#### Twitter: [@caterpillardev](https://twitter.com/caterpillardev)

#### Telegram: [@caterpillardev](https://t.me/caterpillardev) 
