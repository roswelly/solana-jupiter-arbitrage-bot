import { Provider, Program } from "@coral-xyz/anchor";
import { BonkSwap, IDL } from "./IDL";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";

export class BonkSwapSDK {
  public program: Program<BonkSwap>;
  public connection: any;
  constructor(provider?: Provider) {
    this.program = new Program<BonkSwap>(IDL as BonkSwap, provider);
    this.connection = this.program.provider.connection;
  }

  // Implemented methods based on BonkSwap IDL

  async createPool(
    accounts: {
      state: PublicKey,
      pool: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      poolXAccount: PublicKey,
      poolYAccount: PublicKey,
      adminXAccount: PublicKey,
      adminYAccount: PublicKey,
      admin: PublicKey,
      projectOwner: PublicKey,
      programAuthority: PublicKey,
      systemProgram: PublicKey,
      tokenProgram: PublicKey,
      rent: PublicKey,
    },
    args: {
      lpFee: BN,
      buybackFee: BN,
      projectFee: BN,
      mercantiFee: BN,
      initialTokenX: BN,
      initialTokenY: BN,
      bump: number,
    }
  ) {
    return await this.program.methods
      .createPool(
        args.lpFee,
        args.buybackFee,
        args.projectFee,
        args.mercantiFee,
        args.initialTokenX,
        args.initialTokenY,
        args.bump
      )
      .accounts(accounts)
      .rpc();
  }

  async createProvider(
    accounts: {
      pool: PublicKey,
      farm: PublicKey,
      provider: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      poolXAccount: PublicKey,
      poolYAccount: PublicKey,
      ownerXAccount: PublicKey,
      ownerYAccount: PublicKey,
      owner: PublicKey,
      systemProgram: PublicKey,
      tokenProgram: PublicKey,
      rent: PublicKey,
    },
    args: {
      tokenXAmount: BN,
      tokenYAmount: BN,
      bump: number,
    }
  ) {
    return await this.program.methods
      .createProvider(
        args.tokenXAmount,
        args.tokenYAmount,
        args.bump
      )
      .accounts(accounts)
      .rpc();
  }

  async createState(
    accounts: {
      state: PublicKey,
      admin: PublicKey,
      programAuthority: PublicKey,
      systemProgram: PublicKey,
    },
    args: {
      nonce: number,
    }
  ) {
    return await this.program.methods
      .createState(args.nonce)
      .accounts(accounts)
      .rpc();
  }

  async addTokens(
    accounts: {
      state: PublicKey,
      pool: PublicKey,
      farm: PublicKey,
      provider: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      tokenMarco: PublicKey,
      tokenProjectFirst: PublicKey,
      tokenProjectSecond: PublicKey,
      ownerXAccount: PublicKey,
      ownerYAccount: PublicKey,
      poolXAccount: PublicKey,
      poolYAccount: PublicKey,
      ownerMarcoAccount: PublicKey,
      ownerProjectFirstAccount: PublicKey,
      ownerProjectSecondAccount: PublicKey,
      tokenMarcoAccount: PublicKey,
      tokenProjectFirstAccount: PublicKey,
      tokenProjectSecondAccount: PublicKey,
      owner: PublicKey,
      programAuthority: PublicKey,
      systemProgram: PublicKey,
      tokenProgram: PublicKey,
      associatedTokenProgram: PublicKey,
      rent: PublicKey,
    },
    args: {
      deltaX: BN,
      deltaY: BN,
    }
  ) {
    return await this.program.methods
      .addTokens(args.deltaX, args.deltaY)
      .accounts(accounts)
      .rpc();
  }

  async swap(
    accounts: {
      state: PublicKey,
      pool: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      poolXAccount: PublicKey,
      poolYAccount: PublicKey,
      swapperXAccount: PublicKey,
      swapperYAccount: PublicKey,
      swapper: PublicKey,
      referrerXAccount: PublicKey,
      referrerYAccount: PublicKey,
      referrer: PublicKey,
      programAuthority: PublicKey,
      systemProgram: PublicKey,
      tokenProgram: PublicKey,
      associatedTokenProgram: PublicKey,
      rent: PublicKey,
    },
    args: {
      deltaIn: BN,
      priceLimit: BN,
      xToY: boolean,
    }
  ) {
    return await this.program.methods
      .swap(args.deltaIn, args.priceLimit, args.xToY)
      .accounts(accounts)
      .rpc();
  }

  async withdrawBuyback(accounts: {
    state: PublicKey,
    pool: PublicKey,
    tokenX: PublicKey,
    tokenY: PublicKey,
    buybackXAccount: PublicKey,
    buybackYAccount: PublicKey,
    poolXAccount: PublicKey,
    poolYAccount: PublicKey,
    admin: PublicKey,
    programAuthority: PublicKey,
    systemProgram: PublicKey,
    tokenProgram: PublicKey,
    associatedTokenProgram: PublicKey,
    rent: PublicKey,
  }) {
    return await this.program.methods
      .withdrawBuyback()
      .accounts(accounts)
      .rpc();
  }

  async withdrawShares(
    accounts: {
      state: PublicKey,
      pool: PublicKey,
      farm: PublicKey,
      provider: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      tokenMarco: PublicKey,
      tokenProjectFirst: PublicKey,
      tokenProjectSecond: PublicKey,
      poolXAccount: PublicKey,
      poolYAccount: PublicKey,
      tokenMarcoAccount: PublicKey,
      tokenProjectFirstAccount: PublicKey,
      tokenProjectSecondAccount: PublicKey,
      ownerXAccount: PublicKey,
      ownerYAccount: PublicKey,
      ownerMarcoAccount: PublicKey,
      ownerProjectFirstAccount: PublicKey,
      ownerProjectSecondAccount: PublicKey,
      owner: PublicKey,
      programAuthority: PublicKey,
      systemProgram: PublicKey,
      tokenProgram: PublicKey,
      associatedTokenProgram: PublicKey,
      rent: PublicKey,
    },
    args: {
      shares: BN,
    }
  ) {
    return await this.program.methods
      .withdrawShares(args.shares)
      .accounts(accounts)
      .rpc();
  }

  async withdrawLpFee(accounts: {
    state: PublicKey,
    pool: PublicKey,
    provider: PublicKey,
    tokenX: PublicKey,
    tokenY: PublicKey,
    ownerXAccount: PublicKey,
    ownerYAccount: PublicKey,
    poolXAccount: PublicKey,
    poolYAccount: PublicKey,
    owner: PublicKey,
    programAuthority: PublicKey,
    systemProgram: PublicKey,
    tokenProgram: PublicKey,
    associatedTokenProgram: PublicKey,
    rent: PublicKey,
  }) {
    return await this.program.methods
      .withdrawLpFee()
      .accounts(accounts)
      .rpc();
  }

  async withdrawProjectFee(accounts: {
    state: PublicKey,
    pool: PublicKey,
    tokenX: PublicKey,
    tokenY: PublicKey,
    projectOwnerXAccount: PublicKey,
    projectOwnerYAccount: PublicKey,
    poolXAccount: PublicKey,
    poolYAccount: PublicKey,
    projectOwner: PublicKey,
    programAuthority: PublicKey,
    systemProgram: PublicKey,
    tokenProgram: PublicKey,
    associatedTokenProgram: PublicKey,
    rent: PublicKey,
  }) {
    return await this.program.methods
      .withdrawProjectFee()
      .accounts(accounts)
      .rpc();
  }

  async createFarm(
    accounts: {
      state: PublicKey,
      pool: PublicKey,
      farm: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      tokenMarco: PublicKey,
      tokenMarcoAccount: PublicKey,
      adminMarcoAccount: PublicKey,
      admin: PublicKey,
      programAuthority: PublicKey,
      systemProgram: PublicKey,
      tokenProgram: PublicKey,
      rent: PublicKey,
    },
    args: {
      supply: BN,
      duration: BN,
      bump: number,
    }
  ) {
    return await this.program.methods
      .createFarm(args.supply, args.duration, args.bump)
      .accounts(accounts)
      .rpc();
  }

  async createDualFarm(
    accounts: {
      state: PublicKey,
      pool: PublicKey,
      farm: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      tokenMarco: PublicKey,
      tokenProjectFirst: PublicKey,
      tokenMarcoAccount: PublicKey,
      tokenProjectFirstAccount: PublicKey,
      adminMarcoAccount: PublicKey,
      adminProjectFirstAccount: PublicKey,
      admin: PublicKey,
      programAuthority: PublicKey,
      systemProgram: PublicKey,
      tokenProgram: PublicKey,
      rent: PublicKey,
    },
    args: {
      supplyMarco: BN,
      supplyProjectFirst: BN,
      duration: BN,
      bump: number,
    }
  ) {
    return await this.program.methods
      .createDualFarm(args.supplyMarco, args.supplyProjectFirst, args.duration, args.bump)
      .accounts(accounts)
      .rpc();
  }

  async createTripleFarm(
    accounts: {
      state: PublicKey,
      pool: PublicKey,
      farm: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      tokenMarco: PublicKey,
      tokenProjectFirst: PublicKey,
      tokenProjectSecond: PublicKey,
      tokenMarcoAccount: PublicKey,
      tokenProjectFirstAccount: PublicKey,
      tokenProjectSecondAccount: PublicKey,
      adminMarcoAccount: PublicKey,
      adminProjectFirstAccount: PublicKey,
      adminProjectSecondAccount: PublicKey,
      admin: PublicKey,
      programAuthority: PublicKey,
      systemProgram: PublicKey,
      tokenProgram: PublicKey,
      rent: PublicKey,
    },
    args: {
      supplyMarco: BN,
      supplyProjectFirst: BN,
      supplyProjectSecond: BN,
      duration: BN,
      bump: number,
    }
  ) {
    return await this.program.methods
      .createTripleFarm(args.supplyMarco, args.supplyProjectFirst, args.supplyProjectSecond, args.duration, args.bump)
      .accounts(accounts)
      .rpc();
  }

  async withdrawRewards(accounts: {
    state: PublicKey,
    pool: PublicKey,
    farm: PublicKey,
    provider: PublicKey,
    tokenX: PublicKey,
    tokenY: PublicKey,
    tokenMarco: PublicKey,
    tokenProjectFirst: PublicKey,
    tokenProjectSecond: PublicKey,
    tokenMarcoAccount: PublicKey,
    tokenProjectFirstAccount: PublicKey,
    tokenProjectSecondAccount: PublicKey,
    ownerMarcoAccount: PublicKey,
    ownerProjectFirstAccount: PublicKey,
    ownerProjectSecondAccount: PublicKey,
    owner: PublicKey,
    programAuthority: PublicKey,
    systemProgram: PublicKey,
    tokenProgram: PublicKey,
    associatedTokenProgram: PublicKey,
    rent: PublicKey,
  }) {
    return await this.program.methods
      .withdrawRewards()
      .accounts(accounts)
      .rpc();
  }

  async closePool(accounts: {
    state: PublicKey,
    pool: PublicKey,
    farm: PublicKey,
    tokenX: PublicKey,
    tokenY: PublicKey,
    tokenMarcoAccount: PublicKey,
    tokenProjectFirstAccount: PublicKey,
    tokenProjectSecondAccount: PublicKey,
    poolXAccount: PublicKey,
    poolYAccount: PublicKey,
    buybackXAccount: PublicKey,
    buybackYAccount: PublicKey,
    admin: PublicKey,
    programAuthority: PublicKey,
    tokenProgram: PublicKey,
  }) {
    return await this.program.methods
      .closePool()
      .accounts(accounts)
      .rpc();
  }

  async withdrawMercantiFee(accounts: {
    state: PublicKey,
    pool: PublicKey,
    tokenX: PublicKey,
    tokenY: PublicKey,
    mercantiXAccount: PublicKey,
    mercantiYAccount: PublicKey,
    poolXAccount: PublicKey,
    poolYAccount: PublicKey,
    admin: PublicKey,
    programAuthority: PublicKey,
    tokenProgram: PublicKey,
  }) {
    return await this.program.methods
      .withdrawMercantiFee()
      .accounts(accounts)
      .rpc();
  }

  async addSupply(
    accounts: {
      state: PublicKey,
      pool: PublicKey,
      farm: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      tokenMarcoAccount: PublicKey,
      tokenProjectFirstAccount: PublicKey,
      tokenProjectSecondAccount: PublicKey,
      adminMarcoAccount: PublicKey,
      adminProjectFirstAccount: PublicKey,
      adminProjectSecondAccount: PublicKey,
      admin: PublicKey,
      tokenProgram: PublicKey,
    },
    args: {
      supplyMarco: BN,
      supplyProjectFirst: BN,
      supplyProjectSecond: BN,
      duration: BN,
    }
  ) {
    return await this.program.methods
      .addSupply(args.supplyMarco, args.supplyProjectFirst, args.supplyProjectSecond, args.duration)
      .accounts(accounts)
      .rpc();
  }

  async updateFees(
    accounts: {
      state: PublicKey,
      pool: PublicKey,
      tokenX: PublicKey,
      tokenY: PublicKey,
      admin: PublicKey,
      programAuthority: PublicKey,
    },
    args: {
      newBuybackFee: BN,
      newProjectFee: BN,
      newProviderFee: BN,
      newMercantiFee: BN,
    }
  ) {
    return await this.program.methods
      .updateFees(args.newBuybackFee, args.newProjectFee, args.newProviderFee, args.newMercantiFee)
      .accounts(accounts)
      .rpc();
  }

  async resetFarm(accounts: {
    state: PublicKey,
    pool: PublicKey,
    farm: PublicKey,
    tokenX: PublicKey,
    tokenY: PublicKey,
    tokenMarco: PublicKey,
    tokenMarcoAccount: PublicKey,
    tokenProjectFirstAccount: PublicKey,
    tokenProjectSecondAccount: PublicKey,
    adminMarcoAccount: PublicKey,
    adminProjectFirstAccount: PublicKey,
    adminProjectSecondAccount: PublicKey,
    admin: PublicKey,
    programAuthority: PublicKey,
    systemProgram: PublicKey,
    tokenProgram: PublicKey,
    rent: PublicKey,
  }) {
    return await this.program.methods
      .resetFarm()
      .accounts(accounts)
      .rpc();
  }

  async updateRewardTokens(accounts: {
    state: PublicKey,
    pool: PublicKey,
    farm: PublicKey,
    tokenMarcoAccount: PublicKey,
    tokenProjectFirstAccount: PublicKey,
    tokenProjectSecondAccount: PublicKey,
    tokenMarco: PublicKey,
    newTokenMarcoAccount: PublicKey,
    admin: PublicKey,
    programAuthority: PublicKey,
    systemProgram: PublicKey,
    tokenProgram: PublicKey,
  }) {
    return await this.program.methods
      .updateRewardTokens()
      .accounts(accounts)
      .rpc();
  }
} 