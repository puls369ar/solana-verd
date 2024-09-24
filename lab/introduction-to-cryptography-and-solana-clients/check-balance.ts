// Lab of this article https://solana.com/developers/courses/intro-to-solana/intro-to-reading-data

import { Connection, LAMPORTS_PER_SOL, clusterApiUrl, PublicKey } from "@solana/web3.js"
import dotenv from 'dotenv'


(async ()=>{
    
    dotenv.config()
    
    const myPublicKey = new PublicKey(process.env.SOLANA_ALFAACC_PUBLICKEY as string)
    const tolyAddress = new PublicKey("GgJJRwLg9NzFQ97o1CJLGLp1KLSUMBwFc6eQNVEr4fbW")

    const devnet_connection = new Connection(clusterApiUrl('devnet'))
    const mainnet_connection = new Connection(clusterApiUrl('mainnet-beta'))
    
    let my_devnet_balance = await devnet_connection.getBalance(myPublicKey) / LAMPORTS_PER_SOL 
    let toly_mainnet_balance = await mainnet_connection.getBalance(tolyAddress) / LAMPORTS_PER_SOL 


    console.log(`The balance of my test ALFA account is ${my_devnet_balance} SOL`)
    console.log(`The balance of Toly is ${toly_mainnet_balance} SOL`)

})();