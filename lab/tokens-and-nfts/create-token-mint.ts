import { createMint } from "@solana/spl-token"
import "dotenv/config"
import {
    getKeypairFromEnvironment,
    getExplorerLink
} from "@solana-developers/helpers"
import { Connection, clusterApiUrl, Keypair } from "@solana/web3.js"
import * as fs from "fs"

(async () => {
    const user = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync("beta.json").toString()) as number[]))
    const connection = new Connection(clusterApiUrl("devnet"))
    const tokenMint = await createMint(connection, user, user.publicKey, null, 2)

    const link = getExplorerLink("address", tokenMint.toString(), "devnet");
    
    console.log(`Token mint: ${link}`);
})();