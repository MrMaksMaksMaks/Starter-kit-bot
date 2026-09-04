# Solana Starter Kit Bot

A reusable, open-source reference implementation for building Telegram-native Solana applications — without building wallet, signing, swap, and gasless infrastructure from scratch.

---

## Project Status & Roadmap

This repository is actively maintained as **open-source public infrastructure** for the Solana developer ecosystem.

**Current state:** Working implementation on Solana mainnet (see [Live mainnet proof](#live-mainnet-proof)).  
**Proposed roadmap:** Security hardening, account recovery, independent security review, operational reliability, and production documentation — [details below](#roadmap).

The project is **not a commercial trading product**. Advanced trading features (limit orders, DCA, sniping, copy trading) are explicitly out of scope and will not be added to this open-source repository.

---

## What is it?

Solana Starter Kit Bot is an open-source, working Telegram bot that gives Solana developers a reusable starting point for wallet creation, transaction signing, token swaps, and sponsored (gasless) withdrawals — instead of requiring every new project to build that infrastructure from zero.

Building a Telegram application that talks to Solana normally means integrating wallet infrastructure, secure signing, RPC communication, swap routing, transaction sponsorship, and persistent state before a single feature idea can even be tested. This repository provides a working reference for all of that, built with Rust and Teloxide, using Openfort Backend Wallets for signing, Jupiter for swaps, and Kora for sponsored transactions.

Solana fits this use case specifically because interactions are cheap and fast enough for a conversational, frequent-action UX, Jupiter's liquidity is deep enough that swaps don't need custom routing, and Kora's sponsorship infrastructure lets transaction fees be abstracted away from the end user for supported flows.

---

## What works today

| Feature| Status|
| ---| ---|
| Telegram bot (basic frontend)| ✅|
| Openfort Backend Wallet creation| ✅|
| SOL balance check| ✅ Mainnet|
| SPL / Token-2022 balances| ✅ Mainnet|
| Jupiter swaps (buy/sell)| ✅ Mainnet|
| Kora sponsored (gasless) withdrawals| ✅|
| Jupiter Referral (optional, disabled by default)| ✅ Mainnet|

> For supported flows, Kora can sponsor transaction fees so the user does not need to hold SOL specifically to pay the network fee.

> **Note:** Mainnet testing confirms that all core features work with real funds and live Solana infrastructure. Devnet is used for wallet creation and balance checks where applicable.

---

## Live mainnet proof

| Flow| Signature| Explorer|
| ---| ---| ---|
| Buy (SOL → USDC, with referral fee)| `2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1`| [View](https://explorer.solana.com/tx/2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1)|
| Sell (USDC → SOL)| `465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz`| [View](https://explorer.solana.com/tx/465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz)|
| Sponsored withdrawal (via Kora)| `27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87`| [View](https://explorer.solana.com/tx/27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87)|

---

## Demo

| Feature| Screenshot|
| ---| ---|
| **Main Menu**| <img src="assets/demo-start.png" width="400">|
| **Create Wallet**| <img src="assets/demo-wallet.png" width="400">|
| **Buy Tokens**| <img src="assets/demo-buy.png" width="400">|
| **SOL Balance**| <img src="assets/demo-balance.png" width="400">|
| **Token Portfolio**| <img src="assets/demo-tokens.png" width="400">|

---

## Wallet architecture

> **Architecture note:** Openfort is used as the wallet infrastructure layer in this reference implementation. It is treated as a **replaceable integration boundary**, not as the application's security model. The security analysis in this repository explicitly examines which protections are provided by the provider and which remain the application's responsibility. See [SECURITY.md](./SECURITY.md) for the full trust-boundary breakdown.

Before the diagrams, a quick note on terminology — the same word ("wallet," "account") gets used loosely across this space, and it's worth being precise once:

| Term| Meaning|
| ---| ---|
| Telegram user| The application-level identity — a Telegram account interacting with the bot|
| Openfort account| The wallet-infrastructure identity that owns and signs for a Solana wallet|
| Solana wallet| The on-chain address (public key) that holds SOL / SPL tokens|
| SQLite record| The mapping this application stores between a Telegram user and their Openfort account / Solana wallet|

The important security property: the Telegram bot itself does **not** store or directly manage users' private keys. The application stores the mapping above; signing operations are delegated to Openfort's wallet infrastructure.

```
Telegram User
      │
      ▼
Telegram Bot
      │
      │ wallet/account reference
      ▼
Openfort Backend Wallet
      │
      │ transaction signing
      ▼
Solana
```

**Create wallet flow:**

```
/create_wallet
      │
      ▼
Rust application
      │
      ▼
Openfort Backend Wallet API
      │
      ▼
Solana wallet/account created
      │
      ▼
Address returned to user, mapping stored in SQLite
```

> **Important:** This repository is an application integration example. Production deployments should independently evaluate their security model, access controls, authentication, rate limiting, transaction policies, and operational key management.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Telegram User                           │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Telegram Bot API                         │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────────────┐
│                  Rust / Teloxide Application                       │
│                                                                    │
│  /start  /create_wallet  /balance  /tokens  /buy  /sell  /withdraw │
└───────────────┬───────────────────────┬────────────────────────────┘
                │                       │
                ▼                       ▼
┌─────────────────────────┐   ┌───────────────────────────────────┐
│   Application Modules   │   │       External Infrastructure     │
│                         │   │                                   │
│ balance/                │   │ Openfort Backend Wallets          │
│ config/                 │   │ Jupiter API                       │
│ db/                     │   │ Kora                              │
│ jupiter/                │   │ Solana RPC                        │
│ openfort/               │   │                                   │
│ solana/                 │   └───────────────────────────────────┘
│ withdraw/               │
└────────────┬────────────┘
             │
             ▼
┌─────────────────────────┐
│        SQLite           │
│                         │
│ User ↔ Wallet mapping   │
│ Application state       │
└─────────────────────────┘
```

---

## Transaction flows

### Buy / Sell

```
Telegram User
      │
      ▼
/buy or /sell
      │
      ▼
Jupiter (/order)
      │
      ▼
Unsigned swap transaction
      │
      ▼
Openfort signs transaction.message bytes
      │
      ▼
Jupiter (/execute)
      │
      ▼
Solana
```

### Withdraw

```
Telegram User
      │
      ▼
/withdraw
      │
      ▼
Validate destination format + amount
      │
      ▼
Openfort Backend Wallet (user signs)
      │
      ▼
Kora sponsored-transaction flow (fee paid by Kora, not the user)
      │
      ▼
Solana
      │
      ▼
Transaction signature
```

---

## Tech stack

| Technology| Role|
| ---| ---|
| **Rust**| Application and backend logic|
| **Teloxide**| Telegram Bot framework|
| **Solana SDK**| Solana blockchain interaction|
| **Openfort**| Backend Wallet infrastructure and transaction signing|
| **Jupiter**| Token swap and liquidity aggregation|
| **Kora**| Sponsored (gasless) transaction infrastructure|
| **SQLite**| Persistent local application state|
| **Reqwest**| HTTP communication|
| **Serde**| Serialization and API models|

---

## Quick Start

### Prerequisites

- Rust (stable toolchain)
- A Telegram bot token from @BotFather
- An Openfort project — secret key, wallet secret, and publishable key
- For swaps: a Solana mainnet RPC endpoint and a small amount of real SOL to test with (Jupiter has no devnet liquidity — see Known Integration Gotchas)

### Installation

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
cargo build --release
```

### Configuration

```bash
cp .env.example .env
```

Fill in the required variables — see Configuration below for the full list.

Do not commit your `.env` file.

### First commands

```bash
cargo run
```

Then, in Telegram:

1. `/start` — see available commands
2. `/create_wallet` — creates a Solana wallet via Openfort
3. `/balance` — confirms the wallet is live and readable
4. `/buy USDC 0.01` — a small real swap on mainnet, once the wallet is funded

---

## Configuration

| Variable| Required| Description|
| ---| ---| ---|
| `TELEGRAM_BOT_TOKEN`| Yes| Token from @BotFather|
| `OPENFORT_SECRET_KEY`| Yes| Openfort project secret key (`sk_...`)|
| `OPENFORT_WALLET_SECRET`| Yes| Openfort wallet secret used to sign `X-Wallet-Auth` JWTs|
| `OPENFORT_PUBLISHABLE_KEY`| Yes| Openfort publishable key — used for Kora gasless RPC calls|
| `OPENFORT_BASE_URL`| No (default: `https://api.openfort.io`)| Openfort API base URL|
| `SOLANA_RPC_URL`| Yes| Solana RPC endpoint. Use a devnet URL for wallet/balance testing, mainnet for swaps|
| `SOLANA_NETWORK`| Yes| `devnet` or `mainnet` — used for explorer links and cluster context|
| `DATABASE_URL`| No (default: `sqlite:./data/bot.db`)| SQLite connection string|
| `JUPITER_API_KEY`| No| Jupiter API key, if you have one|
| `REFERRAL_FEE_BPS`| No (default: `50`)| Swap fee in basis points routed to your referral account|
| `REFERRAL_ACCOUNT`| No| Your Jupiter Referral account address — must be initialized under the Ultra project (see Known Integration Gotchas)|

---

## Jupiter Referral (Optional Integration Pattern)

This starter kit includes an optional, **disabled-by-default** integration with Jupiter's Referral mechanism, confirmed working on mainnet (see Live mainnet proof).

It exists solely as a **documented integration pattern** for developers who want to understand how transparent fee routing can be added without taking custody of user funds. It is **not a revenue model for this project** and does not affect the repository's public-good orientation.

Disable it entirely by omitting `REFERRAL_ACCOUNT` from `.env` — the core wallet, swap, and withdrawal infrastructure works identically with or without it.

The default configuration routes 50 bps (0.5%) of each swap to a referral account via Jupiter's `referralAccount` / `referralFee` parameters. Developers using this repository as a foundation can remove or replace this configuration entirely.

---

## Security

Security is a core design consideration of the project and an area of active, ongoing work.

The starter kit is designed to demonstrate a safer architecture for Telegram-native Solana applications, while being explicit about the security boundaries of the current implementation.

- The Telegram bot does **not** store users' private keys in SQLite or in the application source code.
- Transaction signing is delegated to **Openfort Backend Wallet** infrastructure.
- Each authenticated request to Openfort includes a freshly generated unique `jti`. The project treats Openfort's server-side validation of this value as part of the provider trust-boundary verification rather than assuming that the presence of `jti` alone constitutes replay protection.
- The application stores the mapping between the Telegram user and the corresponding Openfort account / Solana wallet.

**→ Full threat model, credential-scoping analysis, key-material export boundaries, known limitations, and planned hardening:** [`SECURITY.md`](./SECURITY.md)

---

## Known Integration Gotchas

Real integration pitfalls discovered while building this project. Documenting them here is meant to save the next developer the debugging time it took to find them.

- **Jupiter API is mainnet-only.** There is no Jupiter liquidity on devnet. Wallet creation, balance checks, and withdrawals work fine on devnet, but `/buy` and `/sell` will not find a route — test swaps on mainnet with small amounts.
- **Only `ExactIn` swap mode is supported.** The `/swap/v2/order` endpoint used here does not support `ExactOut`. You can specify "spend exactly this much," not "receive exactly this much."
- **Sign the transaction's _message_, not the full serialized transaction.** When delegating signing to Openfort's backend wallet `/sign` endpoint, hash and send `transaction.message.serialize()` — not `bincode::serialize(&transaction)`. Signing the wrong payload produces a signature that silently fails on-chain verification.
- **Openfort's REST API structure is not always what the public docs suggest, and v1/v2 endpoints differ.** Field names for the same conceptual operation (e.g. `player` vs `user`, snake_case vs camelCase claims, hex vs base64 payload encoding) have changed between API versions. Cross-check against the actual SDK source (`openapi-client/generated/`) rather than relying solely on public documentation.
- **The Jupiter Referral account must be initialized under the correct on-chain "project."** Creating a referral account through the web dashboard may register it under the wrong project for the Meta-Aggregator (`/order` + `/execute`) API. Use `@jup-ag/referral-sdk` with `projectPubKey = DkiqsTrw1u1bYFumumC7sCG2S8K25qc2vemJFHyW2wJc` (Jupiter Ultra Referral Project) if a dashboard-created account is rejected with "Invalid referralAccount" or a project mismatch error.

---

## Roadmap

The roadmap focuses on hardening the existing working implementation into a reusable, security-reviewed foundation — not a full-featured trading platform.

| Phase | Focus | Status |
|---|---|---|
| M1 | Transaction safety controls (confirmation, limits, validation, replay protection, history, logging) | Planned |
| M2 | Account recovery & anti-takeover (TOTP, backup codes, cooldowns, notification, audit logging) | Planned |
| M3 | Independent security review & remediation | Planned |
| M4 | Infrastructure hardening (RPC failover, least-privilege credentials, key-share export verification) | Planned |
| M5 | Testing, developer experience & reproducibility | Planned |
| M6 | Documentation, secret rotation, provider abstraction boundary, production deployment guide | Planned |

Advanced trading features — limit orders, DCA, token sniping, and copy trading — are intentionally out of scope for this open-source repository. They may be developed separately as a commercial product built on top of this open-source foundation and are not part of the proposed roadmap.

---

## Reusability

The project is released as open source so other developers can inspect the implementation, reuse it, and build on top of it.

The same infrastructure can support other kinds of Telegram-native Solana applications, not just this specific bot — for example:

- a community bot;
- a DeFi interface;
- a payment bot;
- a portfolio assistant;
- a game economy;
- or another Telegram-native Solana application.

The wallet integration, transaction signing, swap, and withdrawal logic is the same across all of these use cases.

---

## Contributing

Contributions, bug reports, and improvements are welcome.

```bash
git checkout -b feature/my-feature
cargo fmt
cargo check
git commit -m "Add my feature"
git push origin feature/my-feature
```

Then open a Pull Request.

---

## Community

This repository is maintained as open-source public infrastructure. Bug reports, security feedback, and improvements are welcome via [Issues](https://github.com/MrMaksMaksMaks/Starter-kit-bot/issues) and [Pull Requests](https://github.com/MrMaksMaksMaks/Starter-kit-bot/pulls).

The author commits to maintaining the repository for security patches and community PR review for at least 12 months from the final roadmap milestone.

---

## License

MIT License.

Copyright © 2026.

---

## Author

Built by **MAKSIM GORBUNOV**

Questions, feedback, or collaboration: jobpostgm@gmail.com
