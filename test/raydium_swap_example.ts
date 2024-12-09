import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RaydiumSwapExample } from "../target/types/raydium_swap_example";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("raydium_swap_example", () => {
  // Configuración del provider y el programa
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .RaydiumSwapExample as Program<RaydiumSwapExample>;

  it("Realiza un swap", async () => {
    // Crea cuentas de usuario y fondos iniciales
    const userWallet = provider.wallet.publicKey;
    const userSourceTokenAccount = Keypair.generate();
    const userDestinationTokenAccount = Keypair.generate();

    // Configura las cuentas necesarias (placeholder)
    const ammId = new PublicKey("AmmIdPlaceholder");
    const ammAuthority = new PublicKey("AmmAuthorityPlaceholder");
    const poolCoinTokenAccount = new PublicKey("PoolCoinTokenAccountPlaceholder");
    const poolPcTokenAccount = new PublicKey("PoolPcTokenAccountPlaceholder");
    const serumProgramId = new PublicKey("SerumProgramIdPlaceholder");
    const serumMarket = new PublicKey("SerumMarketPlaceholder");

    // Ejecuta la instrucción del swap
    const tx = await program.methods
      .swap(new anchor.BN(1000)) // Cantidad a swapear
      .accounts({
        ammId,
        ammAuthority,
        poolCoinTokenAccount,
        poolPcTokenAccount,
        serumProgramId,
        serumMarket,
        userSourceTokenAccount: userSourceTokenAccount.publicKey,
        userDestinationTokenAccount: userDestinationTokenAccount.publicKey,
        userWallet,
        tokenProgram: anchor.web3.TokenInstructions.TOKEN_PROGRAM_ID,
        raydiumProgram: program.programId,
      })
      .signers([userSourceTokenAccount, userDestinationTokenAccount])
      .rpc();

    console.log("Transacción enviada:", tx);
  });
});
