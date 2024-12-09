import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ArbProgram } from "../target/types/arb_program";
import { PublicKey } from "@solana/web3.js";
import assert from "assert";

describe("arb-program", () => {
  // Configura el proveedor
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Carga el programa
  const program = anchor.workspace.ArbProgram as Program<ArbProgram>;

  it("Performs a Raydium Swap", async () => {
    // Direcciones ficticias para la prueba
    const ammId = new PublicKey("INSERT_AMM_ID_HERE");
    const ammAuthority = new PublicKey("INSERT_AMM_AUTHORITY_HERE");
    const poolCoinTokenAccount = new PublicKey("INSERT_COIN_TOKEN_ACCOUNT_HERE");
    const poolPcTokenAccount = new PublicKey("INSERT_PC_TOKEN_ACCOUNT_HERE");
    const userSourceTokenAccount = new PublicKey("INSERT_USER_SOURCE_ACCOUNT_HERE");
    const userDestinationTokenAccount = new PublicKey("INSERT_USER_DEST_ACCOUNT_HERE");

    // Construir las cuentas para la instrucción
    const tx = await program.methods
      .swap(new anchor.BN(1000)) // Cantidad de entrada (ejemplo)
      .accounts({
        ammId,
        ammAuthority,
        poolCoinTokenAccount,
        poolPcTokenAccount,
        userSourceTokenAccount,
        userDestinationTokenAccount,
        userWallet: provider.wallet.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Transaction signature", tx);

    // Validar el resultado
    assert.ok(tx); // Asegúrate de que la transacción se completó
  });
});