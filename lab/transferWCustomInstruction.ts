import { TransactionInstruction, Connection, Transaction, SystemProgram, sendAndConfirmTransaction, PublicKey, clusterApiUrl, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js"
import * as fs from "fs"

(async () => {
    const senderKeypair = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("beta.json").toString()) as number[]))
    const receiverKeypair = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("sigma.json").toString()) as number[]))
    
    const lamports = BigInt(LAMPORTS_PER_SOL * 0.1)     // Converting 0.1 SOL into lamports
    const instructionData : Buffer = Buffer.alloc(4+8)  // Bytes 0-3 instruction code, 4-11 amount of SOL  
    instructionData.writeUInt32LE(2,0)                  // Starting from 0th byte, Code 2 for transfer instrcution
    instructionData.writeBigUInt64LE(lamports,4)        // Starting from 4th byte, lamports to be transferred

    const connection = new Connection(clusterApiUrl('devnet'))
    const transaction = new Transaction()

    const instruction = new TransactionInstruction({
        programId: SystemProgram.programId,
        keys: [
          {
            pubkey: senderKeypair.publicKey,
            isSigner: true,
            isWritable: true,
          },
          {
            pubkey: receiverKeypair.publicKey,
            isSigner: false,
            isWritable: true,
          },
          
        ],
        data: instructionData
    });

    transaction.add(instruction)

    const signature = await sendAndConfirmTransaction(connection, transaction, [senderKeypair])   // credentials of sender account should be provided, will get error otherwise

    console.log(`Signature of the transaction sent: ${signature}`)
})();