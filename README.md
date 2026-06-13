# YourCopyright - Digital Asset Protection Platform

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Network: Testnet](https://img.shields.io/badge/Network-Stellar_Testnet-orange.svg)
![Status: Active](https://img.shields.io/badge/Status-Active-brightgreen.svg)

**YourCopyright** is a decentralized smart contract built on the Stellar blockchain using the Soroban SDK. It serves as an immutable registry for intellectual property, allowing creators to digitally mint, protect, and verify ownership of various digital assets such as Books, Music, Artwork, and Source Code.

---

## 🔗 Smart Contract Details

- **Network:** Stellar Testnet
- **Contract ID:** `CBRIWWK6PV7NW5UAEMHSTLECFDDD7XVZWYEZHV3MCSOOPXDBCZO5BA5I`
- **SDK Version:** Soroban SDK v25.0.2

## ✨ Key Features

- **Multi-Asset Registration:** Protect any digital asset class (Books, Art, Music, Video, Code).
- **Cryptographic Fingerprinting:** Links assets to their original files using SHA-256 hashes (`file_hash`), ensuring the content cannot be altered without detection.
- **Immutable Ownership:** Permanently binds a unique Asset ID to the creator's wallet address.
- **Anti-Overwrite Protection:** Built-in safeguards prevent malicious users from overriding existing asset IDs.
- **On-Chain Transparency:** Anyone can query the contract to verify the true owner and metadata of an asset.

## 🏗️ Architecture

The system utilizes a hybrid Web2/Web3 architecture:
1. **Off-Chain (Metadata & Storage):** Large files (e.g., PDF, MP3) are stored on traditional cloud storage. A JSON file containing metadata is generated and its URL (`token_uri`) is captured.
2. **On-Chain (Soroban):** The smart contract stores lightweight, essential data:
    - `title`: Name of the asset.
    - `asset_type`: Category of the asset.
    - `file_hash`: Security fingerprint.
    - `token_uri`: Pointer to off-chain data.

## 🛠️ Usage / Functions

### 1. Register Asset (`register_asset`)
Mint a new digital copyright record.
- **`creator`** (Address): The wallet address of the creator (Requires Auth signature).
- **`asset_id`** (u32): A unique identifier for the asset.
- **`metadata`** (AssetMetadata): A struct containing `title`, `asset_type`, `file_hash`, and `token_uri`.

### 2. Get Asset Information (`get_asset_info`)
Retrieve the public metadata of a registered asset.
- **`asset_id`** (u32): The ID of the asset to query.
- **Returns:** The `AssetMetadata` struct.

### 3. Verify Owner (`get_owner`)
Check the legal owner of a specific asset.
- **`asset_id`** (u32): The ID of the asset.
- **Returns:** The `Address` of the current owner.

## 🚀 How to Interact (CLI Example)

If you have the Stellar CLI installed, you can query the contract directly:

```bash
# Get asset information for Asset ID 101
stellar contract invoke \
  --id CBRIWWK6PV7NW5UAEMHSTLECFDDD7XVZWYEZHV3MCSOOPXDBCZO5BA5I \
  --network testnet \
  -- \
  get_asset_info \
  --asset_id 101