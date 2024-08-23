/* 
So what happens under the hood, we create a simple account
whose owner is Token Program, after we initialize it as a
mint account
*/

import { createMint } from "@solana/spl-token"
import "dotenv/config"
import {
    getKeypairFromEnvironment,
    getExplorerLink
} from "@solana-developers/helpers"
import { sendAndConfirmTransaction, SystemProgram, Transaction, PublicKey, Connection, clusterApiUrl, Keypair } from "@solana/web3.js"
import * as fs from "fs"
import * as token from "@solana/spl-token";

async function buildCreateMintTransaction(
    connection: Connection,
    payer: PublicKey,
    accountKeypair: PublicKey,
    decimals: number,
  ): Promise<Transaction> {
    const lamports = await token.getMinimumBalanceForRentExemptMint(connection);  // Minimum lamports for a newly creating account to be rent-exempt
    const programId = token.TOKEN_PROGRAM_ID;
   
    const transaction = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: payer,
        newAccountPubkey: accountKeypair,
        space: token.MINT_SIZE,
        lamports,
        programId,        // If token.TOKEN_PROGRAM_ID is not an owner the error will be thrown when initializing the account as mint
      }),
      token.createInitializeMintInstruction(
        accountKeypair,
        decimals,
        payer,    // mint authority
        payer,    // freeze authority
        programId,
      ),
    );
   
    return transaction;
}

(async () => {
    const user = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("beta.json").toString()) as number[]))
    const connection = new Connection(clusterApiUrl("devnet"))
    const mintAccount = Keypair.generate();
    
    const transaction = await buildCreateMintTransaction(connection, user.publicKey, mintAccount.publicKey, 2);
    const signature = await sendAndConfirmTransaction(connection, transaction, [user, mintAccount])   // credentials of sender account should be provided, will get error otherwise

    const link = getExplorerLink("address", mintAccount.publicKey.toBase58(), "devnet");
    
    console.log(`Token mint: ${link}`);
})();

