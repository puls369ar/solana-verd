import * as web3 from "@solana/web3";
import * as token from "@solana/spl-token";
 
// We could use createMint function from @solana/spl-token
// But below is demonstrated manuall proccess where we create an account and initialize it as
// a mint account

async function buildCreateMintTransaction(
  connection: web3.Connection,
  payer: web3.PublicKey,
  decimals: number,
): Promise<web3.Transaction> {
  const lamports = await token.getMinimumBalanceForRentExemptMint(connection);  // Minimum lamports for a newly creating account to be rent-exempt
  const accountKeypair = web3.Keypair.generate();
  const programId = token.TOKEN_PROGRAM_ID;
 
  const transaction = new web3.Transaction().add(
    web3.SystemProgram.createAccount({
      fromPubkey: payer,
      newAccountPubkey: accountKeypair.publicKey,
      space: token.MINT_SIZE,
      lamports,
      programId,        // If token.TOKEN_PROGRAM_ID is not an owner the error will be thrown when initializing the account as mint
    }),
    token.createInitializeMintInstruction(
      accountKeypair.publicKey,
      decimals,
      payer,    // mint authority
      payer,    // freeze authority
      programId,
    ),
  );
 
  return transaction;
}

async function buildCreateTokenAccountTransaction(
  connection: web3.Connection,
  payer: web3.PublicKey,
  mint: web3.PublicKey,
): Promise<web3.Transaction> {
  const mintState = await token.getMint(connection, mint);
  const accountKeypair = await web3.Keypair.generate();
  const space = token.getAccountLenForMint(mintState);
  const lamports = await connection.getMinimumBalanceForRentExemption(space);
  const programId = token.TOKEN_PROGRAM_ID;
 
  const transaction = new web3.Transaction().add(
    web3.SystemProgram.createAccount({
      fromPubkey: payer,
      newAccountPubkey: accountKeypair.publicKey,
      space,
      lamports,
      programId,
    }),
    token.createInitializeAccountInstruction(
      accountKeypair.publicKey,
      mint,
      payer,
      programId,
    )
  );
 
  return transaction;
}