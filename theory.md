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
* 
So I already know the simplest principle of blockchain mechanism,I have some
little experience in solidity coding and ethereum smart contract development. So
this time I'd like to start my learning journey from Solana blockchain and writing
smart contracts in Solana ecosystem with Rust programming language. 
But maybe it is a good idea
to firstly understand the solana ecosystem and the difference with 
the ethereum.

# Solana vs ethereum
Blocktime                       |   400-800milisec      |   12.07sec
Transactions/sec capacity       |    710000             |   12-15
Amount of stake                 |   smallAmount+Fee     |   32ETH
Consensus mechanism             |   dPoS PoH            |   PoS

In all the systems amount of transactions per block isn't fixed, but 
depends on different platform specific parameters


# Solana Ecosystem
In Solana Network there are three types of nodes that keep
network working
- Leader - check for transactions to be correct and hashes them
- Validator - check if the hashes were valid and update their version of network's state
- RPC Client - the one that makes the transaction to enter the network and start the proccesses above 

Now we can rise a level above and look to the system from the perspective 
of the developer, but not just the blockchain designer. One of the
most important concepts in Solana are Accounts. There are three types of Accounts
- Data Accounts
- Programm Accounts
- Native Programm Accounts

Each account has 5 main parameters
lamports	    The number of lamports owned by this account
owner
    The owner is always a programm account, by default it is the system programm. 
    Only the owner can change the state and data of the account by accepting 
    neccessary instructions packed in a transaction sent to it.
executable
    Exactly this parameter defines whether the account is a data account or
    programm account. If it is true then the account will be able to perform
    the instructions sent to it by executing the executabel code sent to it via transactions.
    
data	        The raw data byte array stored by this account
rent_epoch	    The next epoch that this account will owe rent


Significant concpets to learn when entering solana ecosystem
- Solana Superteam
    Community of Solana ecosystem to provide the platform for learning,
    finding projects, networking etc. Found in India and now includes six 
    other countries as a member
- dPoS (Delegated Proof of Stake)
- PoH (Proof of History)
    As we know well in PoS mechanism not all block validators do 
    complex mathematical operations, but only the one who is chosen
    randomly according to the amount of the tokens staked. These
    chosen validators are called leaders. There is a schedule of leaders
    which should be followed by the validators. But what if there are two 
    validators that want to verify the block at the same time, in that case the protocol will
    spend time on deciding which validator's clock fits to the schedule and eventually this will
    result in time delay. PoH mechanism solves this issue.
        First of all PoH is not a separate consensus mechanism, it is a way to
    know when to produce blocks for leaders and a way to verify that leaders are taking turns
    correctly for validators. Each block producing lasts a certain amount
    of time, but it is not counted in seconds, but in ticks. SHA256 single hashing
    is taken as a global reference for clocking as with almost all blockchain specific hardware 
    it lasts the same amount of time for all inputs
    with fixed size. To define 1 tick we should
    hash SHA256 five times. We may 
-Sealevel
-ZK, ZK scaling

Programs for Solana blockchain are mainly written in Rust programming language
