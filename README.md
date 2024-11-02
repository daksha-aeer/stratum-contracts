# Stratum: Proof-of-Work Token on NEAR

Stratum is a cryptocurrency powered by NEAR Protocol, allowing anyone to mine tokens on their computer or phone. Built using NEAR’s WASM runtime and Rust, Stratum employs a proof-of-work (PoW) algorithm within a smart contract, enabling decentralized and censorship-resistant token mining. With Stratum, we combine the security and decentralization of PoW with NEAR's efficient blockchain architecture.

### Links
- **Demo**: https://youtu.be/Ob84OdtCP0E
- **Dapp**: https://grand-sawine-1d648f.netlify.app/
- **Deck**: https://www.canva.com/design/DAGB5hj2qyo/jc6mAihQsAaEGgh_aw0OYg/view?utm_content=DAGB5hj2qyo&utm_campaign=designshare&utm_medium=link&utm_source=editor
- **Smart contract code**: https://github.com/daksha-aeer/stratum-contracts
- **UI code**: https://github.com/daksha-aeer/str-frontend

## How Stratum Works

Stratum's mining process is managed through our smart contract that verifies computational proofs. Users interact with this contract through a frontend, where they can connect their NEAR wallets and begin mining tokens. Each valid proof mined increases the token balance in the user's account, and successful transactions are displayed in real-time.

### Smart Contract

The main components of the contract include:

- **Initialization**: Sets up the contract with a counter and salt, initializing metadata for the token.
- **Proof Calculation**: Computes the proof using a `Keccak256` hash function.
- **Proof Submission**: Validates the user's proof and increments the token counter upon success.

### Frontend Application

The front end is a React-based web app where users can initiate mining sessions, view balances, and interact with the contract. The main elements include wallet connection, balance fetching, and proof submission logic.

#### Key Parts of the Frontend Code
- **Wallet Connection**: The app connects to the user's NEAR wallet using the NEAR Wallet Selector and retrieves the balance of $STR tokens.
- **Mining Logic**: After initiating mining, the app continuously submits proofs to the smart contract, which verifies and mints tokens upon successful validation.


### Proof Of Work Landscape
- **Bitcoin**: Bitcoin uses an SHA-256 hashing algorithm where miners compete to find a hash below a certain difficulty level, adjusted every two weeks. Miners earn new Bitcoins and transaction fees as rewards. However, the process is highly energy-intensive, which has sparked environmental concerns, and the mining space is increasingly dominated by large, centralized operators.

- **Monero**: Monero, on the other hand, employs RandomX—a CPU-friendly, ASIC-resistant algorithm that promotes decentralization by allowing mining on general-purpose CPUs. This makes Monero mining more accessible and reinforces its privacy-focused ethos. Yet, Monero’s commitment to privacy can lead to slower transactions, and even RandomX may still attract centralization through high-performance setups.

- **Ore**: Ore on Solana introduces a more accessible proof-of-work model, lowering technical entry barriers and allowing newcomers to participate in mining with minimal setup. Ore pioneered proof of work on smart contracts using drillx, a GPU-resistant PoW algorithm based on Monero's RandomX.


### What's next for Stratum
Looking ahead, we have a lot of ideas for Stratum. First, we’re planning to enhance both the security and complexity of our proof-of-work system. Right now, the system runs on a straightforward counter, but we’d like to implement more advanced hashing functions, such as Equihash or RandomX, to make the mining process more secure and challenging.

We also want to expand wallet compatibility, allowing users to connect with a wider range of wallets for greater convenience and flexibility. Beyond mining, we’re excited to introduce staking options, enabling users to earn higher rewards by committing tokens to the network. This could add a new dimension to user engagement, blending mining and staking for a more dynamic experience.

Ultimately, we see a lot of potential for Stratum to grow, and we're excited to explore features that could make it even more user-friendly and rewarding.
