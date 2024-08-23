import { mintTo } from "@solana/spl-token"
import {
    getExplorerLink,
    getKeypairFromEnvironment
} from "@solana-developers/helpers"
import { Keypair, Connection, PublicKey, clusterApiUrl } from "@solana/web3.js"
import * as fs from "fs"


(async () => {
    const connection = new Connection(clusterApiUrl("devnet"))
    const MINOR_UNITS_PER_MAJOR_UNITS = Math.pow(10, 2)
    const user = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("beta.json").toString()) as number[]))

    const tokenMintAccount = new PublicKey("4YwAiY4EjVrdtFvYPUxXXUUwAGsXjXyYR6ounytC44nF")
    const recipientAssociatedAccount = new PublicKey("4ve3XgwRsUyNSyanJA8iyQ8EVzK6iAUYicP2Lwdkh4VM")

    const transactionSignature = await mintTo(
        connection,
        user,
        tokenMintAccount,
        recipientAssociatedAccount,
        user,
        10 * MINOR_UNITS_PER_MAJOR_UNITS
    )

    const link = getExplorerLink("transaction", transactionSignature, "devnet");

})()