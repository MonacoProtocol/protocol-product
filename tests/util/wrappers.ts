import { Keypair } from "@solana/web3.js";

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProtocolProduct as ProtocolProductProgram } from "../../target/types/protocol_product";
import { findProductPda } from "../util/pdas";
const { SystemProgram } = anchor.web3;

export let protocolProduct: ProtocolProduct;
beforeAll(async () => {
  // Programs
  protocolProduct = new ProtocolProduct(
    anchor.getProvider() as anchor.AnchorProvider,
    anchor.workspace.ProtocolProduct,
  );
});

export class ProtocolProduct {
  readonly provider: anchor.AnchorProvider;
  readonly program: Program<ProtocolProductProgram>;

  constructor(
    provider: anchor.AnchorProvider,
    program: Program<ProtocolProductProgram>,
  ) {
    this.provider = provider;
    this.program = program;
  }

  getRawProgram(): Program {
    return this.program as Program;
  }

  async createProduct(
    productTitle: string,
    commissionRate: number,
    authority?: Keypair,
  ) {
    const defaultAuthority = authority == undefined;
    const productPk = await findProductPda(productTitle, this.getRawProgram());
    await protocolProduct.program.methods
      .createProduct(productTitle, commissionRate)
      .accounts({
        product: productPk,
        commissionEscrow: Keypair.generate().publicKey,
        authority: defaultAuthority
          ? protocolProduct.provider.publicKey
          : authority.publicKey,
        payer: protocolProduct.provider.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers(defaultAuthority ? [] : [authority])
      .rpc()
      .catch((e) => {
        throw e;
      });

    return productPk;
  }
}
