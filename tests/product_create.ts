import * as assert from "assert";
import { findProductPda } from "./util/pdas";
import { protocolProduct } from "./util/wrappers";

describe("Product Creation", () => {
  it("Creation success", async () => {
    const productTitle = "Ewans Exchange!";

    const productPk = await findProductPda(
      productTitle,
      protocolProduct.getRawProgram(),
    );

    await protocolProduct.createProduct(productTitle, 5);

    const product = await protocolProduct.program.account.product.fetch(
      productPk,
    );
    assert.equal(product.commissionRate.toFixed(2), "5.00");
    assert.equal(
      product.authority.toBase58(),
      protocolProduct.provider.publicKey.toBase58(),
    );
    assert.equal(
      product.payer.toBase58(),
      protocolProduct.provider.publicKey.toBase58(),
    );
    assert.equal(product.productTitle, productTitle);
  });

  it("Invalid commission rate", async () => {
    try {
      await protocolProduct.createProduct("Will error", 101);
      assert.fail(
        "Test passed, but should have errored with InvalidCommissionRate",
      );
    } catch (e) {
      assert.equal(e.error.errorCode.code, "InvalidCommissionRate");
    }
  });

  it("Invalid commission rate - precision too high", async () => {
    try {
      await protocolProduct.createProduct("Will error", 99.1111);
      assert.fail(
        "Test passed, but should have errored with CommissionPrecisionTooLarge",
      );
    } catch (e) {
      assert.equal(e.error.errorCode.code, "CommissionPrecisionTooLarge");
    }
  });
});
