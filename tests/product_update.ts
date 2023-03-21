import { Keypair } from "@solana/web3.js";
import * as assert from "assert";
import { createWalletWithBalance } from "./util/test_util";
import { protocolProduct } from "./util/wrappers";

describe("Product Updates", () => {
  it("Update commission rate success", async () => {
    const productPk = await protocolProduct.createProduct(
      "UPDATE_COMMISSION_RATE",
      50.0,
    );

    const updatedCommissionRate = 75.75;
    await protocolProduct.program.methods
      .updateProductCommissionRate(
        "UPDATE_COMMISSION_RATE",
        updatedCommissionRate,
      )
      .accounts({
        product: productPk,
        authority: protocolProduct.provider.publicKey,
      })
      .rpc();

    const updatedproduct = await protocolProduct.program.account.product.fetch(
      productPk,
    );
    assert.equal(updatedproduct.commissionRate, updatedCommissionRate);
  });

  it("Update commission escrow account success", async () => {
    const productPk = await protocolProduct.createProduct(
      "UPDATE_COMMISSION_ESCROW",
      50.0,
    );

    const updatedCommissionEscrow = Keypair.generate().publicKey;
    await protocolProduct.program.methods
      .updateProductCommissionEscrow(
        "UPDATE_COMMISSION_ESCROW",
        updatedCommissionEscrow,
      )
      .accounts({
        product: productPk,
        authority: protocolProduct.provider.publicKey,
      })
      .rpc();

    const updatedproduct = await protocolProduct.program.account.product.fetch(
      productPk,
    );
    assert.equal(
      updatedproduct.commissionEscrow.toBase58(),
      updatedCommissionEscrow.toBase58(),
    );
  });

  it("Update product, authority different from payer", async () => {
    const authority = await createWalletWithBalance(protocolProduct.provider);
    const productPk = await protocolProduct.createProduct(
      "UPDATE_COMMISSION_RATE_2",
      50.0,
      authority,
    );

    const updatedCommissionRate = 75.75;
    await protocolProduct.program.methods
      .updateProductCommissionRate(
        "UPDATE_COMMISSION_RATE_2",
        updatedCommissionRate,
      )
      .accounts({
        product: productPk,
        authority: authority.publicKey,
      })
      .signers([authority])
      .rpc();

    const updatedproduct = await protocolProduct.program.account.product.fetch(
      productPk,
    );
    assert.equal(updatedproduct.commissionRate, updatedCommissionRate);
  });

  it("Update product authority - system account authority", async () => {
    const productPk = await protocolProduct.createProduct(
      "UPDATE_AUTHORITY",
      50.0,
    );
    const updatedAuthority = await createWalletWithBalance(
      protocolProduct.provider,
    );

    await protocolProduct.program.methods
      .updateProductAuthority("UPDATE_AUTHORITY")
      .accounts({
        product: productPk,
        authority: protocolProduct.provider.publicKey,
        updatedAuthority: updatedAuthority.publicKey,
      })
      .signers([updatedAuthority])
      .rpc();

    const updatedproduct = await protocolProduct.program.account.product.fetch(
      productPk,
    );
    assert.equal(
      updatedproduct.authority.toBase58(),
      updatedAuthority.publicKey.toBase58(),
    );
  });
});
