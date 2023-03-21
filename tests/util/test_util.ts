import { Keypair } from "@solana/web3.js";
import { Provider } from "@coral-xyz/anchor";

export async function createWalletWithBalance(
  provider: Provider,
  lamportBalance = 1000000000,
) {
  const payer = Keypair.generate();
  await provider.connection.confirmTransaction(
    await provider.connection.requestAirdrop(payer.publicKey, lamportBalance),
    "confirmed",
  );
  return payer;
}
