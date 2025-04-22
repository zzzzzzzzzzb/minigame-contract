import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

// import { Connection, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import type {Instructions} from "../target/types/instructions";

describe("Test", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Instructions as anchor.Program<Instructions>;

  it("buy", async () => {
    const owner = pg.wallets.wallet1.keypair;
    const owner_balance = await program.provider.connection.getBalance(owner.publicKey);
    console.log(`send balance: ${owner_balance / web3.LAMPORTS_PER_SOL} SOL`);
    const sender = pg.wallets.wallet2.keypair;

    const sender_balance = await program.provider.connection.getBalance(sender.publicKey);
    console.log(`send balance: ${sender_balance / web3.LAMPORTS_PER_SOL} SOL`);

    // 创建nft
    const mint_nft = await createMint(
        program.provider.connection,
        sender,
        owner.publicKey,
        null,
        0
    );
    console.log("mint_nft");

    const senderNftAta = await createAssociatedTokenAccount(
        program.provider.connection,
        sender,
        mint_nft,
        sender.publicKey
    );
    console.log("senderNftAta");

    const ownerNftAta = await createAssociatedTokenAccount(
        program.provider.connection,
        sender,
        mint_nft,
        owner.publicKey
    );
    console.log("ownerNftAta");

    // 创建新的代币
    const mint = await createMint(
        program.provider.connection,
        sender,
        owner.publicKey,
        null,
        0
    );
    console.log("mint");

    // 创建关联账户
    const senderAta = await createAssociatedTokenAccount(
        program.provider.connection,
        sender,
        mint,
        sender.publicKey
    );
    console.log("senderAta");

    const ownerAta = await createAssociatedTokenAccount(
        program.provider.connection,
        sender,
        mint,
        owner.publicKey
    );
    console.log("ownerAta");

    // 给sender账户铸造spl代币
    const amount = 1000;
    await mintTo(
        program.provider.connection,
        owner,
        mint,
        senderAta,
        owner.publicKey,
        amount
    );
    console.log("mintTo");

    let src_nft = mint_nft;
    let token_type = 2;
    let price = 2;
    let value = 100;

    let [price_account] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("price"),
          src_nft.toBuffer(),
          new BN(token_type).toArrayLike(Buffer, "le", 1),
        ],
        program.programId
    );
    // Send transaction
    const txHash = await program.methods
        .setPrice(src_nft, token_type, new BN(price))
        .accounts({
          payer: program.provider.publicKey,
          priceAccount: price_account,
        })
        // .signers([program.provider.wallet.payer])
        .rpc();
    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);
    await program.provider.connection.confirmTransaction(txHash);

    const buyTxHash = await program.methods
        .buy(src_nft, token_type, new BN(value))
        .accounts({
          sender: sender.publicKey,
          priceAccount: price_account,
          owner: owner.publicKey,
          senderAta: senderAta,
          ownerAta: ownerAta,
          mintAta: mint_nft,
          senderNftAta: senderNftAta,
          ownerNftAta: ownerNftAta,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([owner, sender])
        .rpc();
    console.log(`Use 'solana confirm -v ${buyTxHash}' to see the logs`);
    await program.provider.connection.confirmTransaction(buyTxHash);
  });
});
