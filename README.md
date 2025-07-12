
# Solana_Property_Registration

A decentralized property registry built on Solana blockchain using Anchor, with Tests . Users can register properties, manage ownership history, assign nominees, and handle property dispute/freeze status — all on-chain.


# Features
🧾 Property Registration
Register a property with ID, location, and area.

Store current owner and last 5 previous owners.

👥 Nominee Assignment
Property owner can add up to 10 nominees.

Each nominee can be assigned a specific percentage share (max total: 100%).

Nominees can later claim their shares.

🔒 Dispute Handling
Admin can freeze/unfreeze a property (dispute status).

When frozen, transfers are blocked.

🛠️ Admin Panel (Governance)
Admin initializes and controls freeze/unfreeze.

Admin authorization is enforced on-chain.


