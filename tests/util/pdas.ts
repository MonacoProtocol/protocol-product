import { PublicKey } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";

export async function findProductPda(title: string, program: Program) {
  const [productPk] = await PublicKey.findProgramAddress(
    [Buffer.from("product"), Buffer.from(title)],
    program.programId,
  );
  return productPk;
}
