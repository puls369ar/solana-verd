import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

import {
    getExplorerLink,
} from "@solana-developers/helpers"
import { Keypair, Connection, PublicKey, clusterApiUrl } from "@solana/web3.js"
import * as fs from "fs"



(async () => {
    const connection = new Connection(clusterApiUrl("devnet"))
    const user = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("beta.json").toString()) as number[]))


    const tokenMintAccount = new PublicKey("4YwAiY4EjVrdtFvYPUxXXUUwAGsXjXyYR6ounytC44nF")
    const recipient = user.publicKey

    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        user,
        tokenMintAccount,
        recipient
    )

    const link = getExplorerLink(
        "address",
        tokenAccount.address.toBase58(),
        "devnet"
    )

    console.log(`Created Associated Token Account: ${link}`);
})();