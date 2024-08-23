import { TransactionInstruction, Connection, Transaction, SystemProgram, sendAndConfirmTransaction, PublicKey, clusterApiUrl, Keypair } from "@solana/web3.js"
import * as fs from "fs"

(async () => {
    const senderKeypair = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("beta.json").toString()) as number[]))
    const affectedKeypair = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("sigma.json").toString()) as number[]))
    const pingProgramDataId = new PublicKey("Ah9K7dQ8EHaZqcAsgBW8w37yN2eAy3koFmUn4x3CJtod");
    const programId = new PublicKey("ChT1B39WKLS8qUrkLvFDXMhEJ4F1XZzwUNHUt4AU9aVa");

    const connection = new Connection(clusterApiUrl('devnet'))
    const transaction = new Transaction()

    const instruction = new TransactionInstruction({
        programId: programId,
        keys: [
          {
            pubkey: pingProgramDataId,
            isSigner: false,
            isWritable: true,
          },
        ]
    });

    transaction.add(instruction)

    const signature = await sendAndConfirmTransaction(connection, transaction, [senderKeypair])   // credentials of sender account should be provided, will get error otherwise

    console.log(`Signature of the transaction sent: ${signature}`)
})();