* When develoying smart contract in Solana, everytime we should manually write smart contract's address where our app will be hosted. In Anchor framework this address is automatically generated after building the project

* when using SPL Library **Mint Authority** and **Freeze Authority** are provided in `Create Mint` instruction and **Upgrade Authority** is an optional field, which is provided in `Create Mint Metadata` instruction, as the instruction itself is optional too

* In Solana there is a possibility to extend Account's data capacity with the command `solana program extend <ACCOUNT> <AMOUNT_OF_BYTES> -u d -k <PATH_TO_KEYPAIR>`. This was needed when I was getting an error of **account's data is too small** also check this [video](https://www.youtube.com/watch?v=xfgNe8w8b3w)

* This table represents types of stablecoins

  | Type                        | Backing Mechanism            | Key Examples            | Advantages                                   | Disadvantages                                      |
  |-----------------------------|------------------------------|-------------------------|---------------------------------------------|---------------------------------------------------|
  | **Fiat-Collateralized**      | Fiat reserves (USD, EUR)      | USDT, USDC, BUSD         | Stable, simple to understand                | Centralized, reliant on trust in issuer            |
  | **Crypto-Collateralized**    | Cryptocurrencies (over-collateralized) | DAI, sUSD              | Decentralized, transparent                  | Volatile collateral, complex mechanisms            |
  | **Algorithmic**              | Algorithm-controlled supply   | Frax, Ampleforth         | Fully decentralized, no need for collateral | Risk of failure, complex to maintain               |
  | **Commodity-Collateralized** | Physical assets (gold, silver) | PAXG, Tether Gold        | Backed by tangible assets like gold         | Centralized, less liquid than fiat-backed stablecoins |
  | **Hybrid**                   | Combination of mechanisms     | Frax                    | Balances collateral and algorithm           | Complexity, requires fine-tuning of mechanisms     |


* In Anchor `anchor-cli 0.30.1` there is a bug. Project named "a2b" was being searched as "a_2b" when running `anchor test` because of the conventional [reason](https://github.com/coral-xyz/anchor/blob/0df688481f4ba93e9b7e011a92752d1d7664b68f/ts/packages/anchor/src/workspace.ts#L29-L38). The workaround is to name anchor projects by setting `_` before each digit used, but better not to use digits at all

* Solana has its own token standard called ** SPL-TOKEN **, analogous to the ** ERC20 ** standard. It's important to understand that mint accounts or token accounts on Solana don't function in the same sense as traditional smart contracts (like Ethereum's ERC20). Instead, they are accounts that follow the structure defined by the SPL Token program. The SPL Token program defines the behavior and interaction with tokens, such as minting, transferring, freezing, and burning. The functions or operations related to SPL tokens are built into the SPL Token program, which you can interact with through the Solana CLI, spl-token CLI, or programmatically
