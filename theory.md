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
