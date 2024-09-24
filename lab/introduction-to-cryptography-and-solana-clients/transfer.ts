// Lab of this article https://solana.com/developers/courses/intro-to-solana/intro-to-writing-data

import { Connection, Transaction, SystemProgram, sendAndConfirmTransaction, PublicKey, clusterApiUrl } from "@solana/web3.js"
import { getKeypairFromEnvironment } from "@solana-developers/helpers"
import "dotenv/config"


(async () => {
    const senderKeypair = getKeypairFromEnvironment("SOLANA_ALFAACC_PRIVATEKEY")
    const receiverKeypair = getKeypairFromEnvironment("SOLANA_SIGMAACC_PRIVATEKEY")

    const connection = new Connection(clusterApiUrl('devnet'))
    const transaction = new Transaction()

    const sendSolInstruction = SystemProgram.transfer({
        fromPubkey: senderKeypair.publicKey,
        toPubkey: receiverKeypair.publicKey,
        lamports: 5,
    });

    transaction.add(sendSolInstruction)

    const signature = await sendAndConfirmTransaction(connection, transaction, [receiverKeypair])   // credentials of sender account should be provided, will get error otherwise

    console.log(`The balance of SIGMA account is now ${await connection.getBalance(receiverKeypair.publicKey)} lamports`)
    console.log(`Signature of transaction is ${signature}`)

    // SIGMA account's balance was 0, so it wasn't rent-exempt and wouldn't perform any action in network, even getting lamports from a different account
    // So to make my test valid I just filled SIGMA's balance with 5 SOL via the faucet

    // Initially ALFA had 10 SOL, I transfered 5 lamport to SIGMA, the fee was 0.000005 SOL = 5000 lamport
    // So now Alfa's balance is 9.999994995 SOL

    // I don't find any information about the block time or transaction period

    // Different levels of commitment can be specified when creating a new connection
    // to the network. It defines after which stage of proccessing the API call made by that connection will
    // return control back to the program. By default the value is `finalized`
})();