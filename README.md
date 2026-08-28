# Solana Starter Kit Bot

**Open-source starter kit for building Telegram-native Solana applications.**

Solana Starter Kit Bot is an open-source Telegram bot built with Rust that combines Solana wallet infrastructure, token swaps, withdrawals, and gasless transaction flows into a single working application.

The repository is a working reference implementation and a reusable starting point for developers building Telegram-based Solana applications.

---

## Why this project?

Building a Telegram application that interacts with Solana requires several pieces of infrastructure to work together:

- wallet creation and management;
- secure transaction signing;
- Solana RPC communication;
- token swaps and liquidity routing;
- transaction sponsorship;
- persistent user and wallet state;
- Telegram bot infrastructure.

For an individual developer or a small team, integrating all of these components can become a significant barrier before the actual application idea can even be tested.

**The repository is designed to reduce that infrastructure barrier.**

The repository provides a working reference implementation that brings together:

- **Openfort Backend Wallets** for wallet infrastructure and transaction signing;
- **Jupiter** for token swaps and liquidity aggregation;
- **Kora** for gasless transaction flows;
- **Solana RPC** for blockchain interaction;
- **Rust + Teloxide** for the Telegram application layer;
- **SQLite** for persistent application data.

The repository is intended to be a **public, reusable starting point** rather than a closed application.

Developers can fork the project, inspect the implementation, replace individual components, add their own business logic, and use the existing integration patterns as a foundation for new Solana applications.

### Problem

> **Developers should be able to experiment with Telegram-native Solana applications without first building the entire wallet and transaction infrastructure themselves.**

---

## Live demo — proof it works

The following transactions were executed on Solana mainnet using this implementation:

| Flow | Signature | Explorer link |
|---|---|---|
| Buy (SOL → USDC, with Jupiter Referral fee applied) | `2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1` | [View on Solana Explorer](https://explorer.solana.com/tx/2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1) |
| Sell (USDC → SOL) | `465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz` | [View on Solana Explorer](https://explorer.solana.com/tx/465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz) |
| Withdraw (gasless, via Kora) | `27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87` | [View on Solana Explorer](https://explorer.solana.com/tx/27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87) |

---

## Why Solana?

The project exposes common on-chain operations through a Telegram interface.

That experience requires a blockchain environment where transactions can be:

- fast enough for interactive applications;
- inexpensive enough for frequent user actions;
- supported by a mature token and DeFi ecosystem;
- compatible with modern transaction sponsorship infrastructure.

Solana is a particularly strong fit for this use case.

### Fast and inexpensive interactions

A Telegram application can generate many small and frequent blockchain interactions.

Users may check balances, swap tokens, send funds, or perform other on-chain actions directly from a conversational interface. Keeping those interactions practical requires low transaction costs and fast confirmation.

Solana's performance and transaction economics make this type of user experience practical.

### A mature DeFi ecosystem

The project uses **Jupiter** as its swap infrastructure.

This provides token swaps through Solana's existing liquidity infrastructure instead of requiring each application to implement its own routing and liquidity logic.

### Gasless transaction infrastructure

A major usability problem for blockchain applications is requiring users to maintain a separate balance of the native token just to pay transaction fees.

This project integrates **Kora** to support gasless transaction flows.

Combined with Openfort Backend Wallets, this allows developers to build Telegram experiences where transaction-fee management can be abstracted away from the user for supported flows.

### A strong developer ecosystem

Solana already provides the building blocks needed for this type of application:

- Solana RPC infrastructure;
- a mature token ecosystem;
- Jupiter liquidity infrastructure;
- Kora transaction sponsorship;
- Openfort wallet infrastructure;
- Rust tooling and libraries;
- a large ecosystem of DeFi applications and protocols.

The repository connects these components in a single reference implementation.

### Why build specifically for Solana?

This project is intentionally **Solana-first**, not chain-agnostic.

The objective is not to provide a generic multi-chain wallet abstraction. It is to provide a practical foundation for Telegram-native applications specifically on Solana.

The combination of low-cost transactions, high performance, mature DeFi infrastructure, and gasless transaction primitives creates a strong foundation for this type of application.

---

## What this repository provides

- **A working implementation**, not just a description of the integration — see [Live demo](#live-demo--proof-it-works) for real mainnet transactions produced by this codebase.
- **Reusable integrations.** Wallet, swap, transaction, and bot logic are separated into independent modules that can be inspected, modified, or replaced individually.
- **Documented integration pitfalls.** Problems found while building this — API version mismatches, signing payload format, mainnet-only dependencies — are written down in [Known Limitations & Gotchas](#known-limitations--gotchas) instead of left for the next developer to rediscover.
- **Open by default.** The application code is public under the MIT license, so it can be forked, inspected, and built upon.

---

## Current functionality

The current implementation includes:

| Feature | Description |
|---|---|
| Backend Wallets | Solana wallets implemented through Openfort Backend Wallet infrastructure |
| Remote transaction signing | The Telegram bot does not store users' private keys |
| Jupiter swaps | Buy and sell supported Solana tokens through Jupiter (SOL ↔ USDC/USDT confirmed live on mainnet) |
| Gasless transactions | Gasless transaction flow using Kora |
| SOL withdrawals | Withdraw SOL to an external Solana address |
| Balance checking | View SOL and SPL token balances (including Token-2022) |
| Jupiter referral integration | Optional swap monetization through Jupiter Referral — confirmed working on mainnet |
| Secret isolation | Sensitive wallet credentials are kept outside the application database |

> **Token support note:** USDC and USDT are wired up and directly tested end-to-end on mainnet. wBTC and wETH mint addresses are included and resolvable by symbol, but their swap path and decimals have not yet been independently verified against a live trade — treat them as "should work" rather than "confirmed" until tested. Any other SPL token can be used by passing its raw mint address directly to `/buy` or `/sell`.

---

## Known Limitations & Gotchas

This section exists specifically for developers using the repository as a reference. These are real integration pitfalls discovered while building this project — documenting them here is meant to save you the debugging time it took to find them.

- **Jupiter API is mainnet-only.** There is no Jupiter liquidity on devnet. Wallet creation, balance checks, and withdrawals work fine on devnet, but `/buy` and `/sell` will not find a route — test swaps on mainnet with small amounts.
- **Only `ExactIn` swap mode is supported.** The `/swap/v2/order` endpoint used here does not support `ExactOut`. You can specify "spend exactly this much," not "receive exactly this much." Plan your UX around that constraint rather than fighting it.
- **Sign the transaction *message*, not the full serialized transaction.** When delegating signing to Openfort's backend wallet `/sign` endpoint, hash and send `transaction.message.serialize()` — not `bincode::serialize(&transaction)`. Signing the wrong payload produces a signature that silently fails verification on-chain.
- **Openfort's REST API structure is not always what the public docs suggest, and v1/v2 endpoints differ.** Field names for the same conceptual operation (e.g. `player` vs `user`, snake_case vs camelCase claims, hex vs base64 payload encoding) have changed between API versions. When integrating, cross-check against the actual SDK source (`openapi-client/generated/`) rather than relying solely on public documentation or AI-agent guesses.
- **Withdrawals execute immediately, with no confirmation step.** There is currently no "are you sure?" prompt before `/withdraw` sends funds. This is a deliberate MVP simplification, not an oversight — add a confirmation step before using this in front of real users.
- **The Jupiter Referral account must be initialized under the correct on-chain "project."** Creating a referral account through the web dashboard may register it under the wrong project for the Meta-Aggregator (`/order` + `/execute`) API. Use the `@jup-ag/referral-sdk` with `projectPubKey = DkiqsTrw1u1bYFumumC7sCG2S8K25qc2vemJFHyW2wJc` (Jupiter Ultra Referral Project) if a dashboard-created account is rejected with "Invalid referralAccount" or a project mismatch error.

---

## Wallet architecture

The project uses **Openfort Backend Wallets**.

The important security property is that the Telegram bot itself does **not** store or directly manage users' private keys.

The application stores the information required to associate a Telegram user with their Openfort account/wallet, while signing operations are delegated to Openfort's wallet infrastructure.

Conceptually:

```text
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

> **Important:** This repository is an application integration example. Production deployments should independently evaluate their security model, access controls, authentication, rate limiting, transaction policies, and operational key management.

---

## Architecture

```text
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
┌─────────────────────────────────────────────────────────────────┐
│                  Rust / Teloxide Application                    │
│                                                                 │
│  /start  /create_wallet  /balance  /tokens  /buy  /sell  /withdraw │
└───────────────┬───────────────────────┬─────────────────────────┘
                │                       │
                ▼                       ▼
┌─────────────────────────┐   ┌───────────────────────────────────┐
│   Application Modules   │   │       External Infrastructure     │
│                         │   │                                   │
│ balance/                │   │ Openfort Backend Wallets          │
│ config/                 │   │ Jupiter API                      │
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

## Main transaction flows

### Create wallet

```text
Telegram User
      │
      ▼
/create_wallet
      │
      ▼
Rust application
      │
      ▼
Openfort Backend Wallet API
      │
      ▼
Solana wallet/account
      │
      ▼
Wallet address returned to user
```

### Token swap

```text
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

### SOL withdrawal

```text
Telegram User
      │
      ▼
/withdraw
      │
      ▼
Validate destination + amount
      │
      ▼
Openfort Backend Wallet (user signs)
      │
      ▼
Kora gasless transaction flow (Kora co-signs as fee payer)
      │
      ▼
Solana
      │
      ▼
Transaction signature
```

---

## Telegram commands

| Command | Description |
|---|---|
| `/start` | Welcome message and available commands |
| `/create_wallet` | Create a new Solana wallet |
| `/balance` | Check SOL balance |
| `/tokens` | View all SPL token balances (SOL + Token / Token-2022) |
| `/buy <token> <amount>` | Spend SOL to buy a token |
| `/sell <token> <amount>` | Sell a token for SOL |
| `/withdraw <amount> <address>` | Withdraw SOL to an external Solana address |

### Buy

```text
/buy <token> <SOL amount to spend>
```

Example:

```text
/buy USDC 0.1
```

Spends 0.1 SOL to buy USDC — the amount is always denominated in what you're spending, not what you'll receive (a consequence of `ExactIn`-only swap mode; see Known Limitations).

### Sell

```text
/sell <token> <token amount to sell>
```

Example:

```text
/sell USDC 5
```

Currently directly tested tokens:

- USDC
- USDT

Also resolvable by symbol (not yet independently verified — see Known Limitations):

- wBTC
- wETH

Any other SPL token works by passing its raw mint address in place of the symbol.

### Withdraw

```text
/withdraw <amount> <address>
```

Example:

```text
/withdraw 0.1 7xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

---

## Tech stack

| Technology | Role |
|---|---|
| **Rust** | Application and backend logic |
| **Teloxide** | Telegram Bot framework |
| **Solana SDK** | Solana blockchain interaction |
| **Openfort** | Backend Wallet infrastructure and transaction signing |
| **Jupiter** | Token swap and liquidity aggregation |
| **Kora** | Gasless transaction infrastructure |
| **SQLite** | Persistent local application state |
| **Reqwest** | HTTP communication |
| **Serde** | Serialization and API models |

---

## Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
```

### 2. Create your environment file

```bash
cp .env.example .env
```

Fill in the required environment variables — see [Configuration](#configuration) below for the full list.

Do not commit your `.env` file.

### 3. Build the project

```bash
cargo build --release
```

### 4. Run the bot

```bash
cargo run
```

For a production build:

```bash
./target/release/<binary-name>
```

---

## Configuration

Configuration is provided through environment variables. Copy `.env.example` to `.env` and fill in the following:

| Variable | Required | Description |
|---|---|---|
| `TELEGRAM_BOT_TOKEN` | Yes | Token from [@BotFather](https://t.me/BotFather) |
| `OPENFORT_SECRET_KEY` | Yes | Openfort project secret key (`sk_...`) |
| `OPENFORT_WALLET_SECRET` | Yes | Openfort wallet secret used to sign `X-Wallet-Auth` JWTs |
| `OPENFORT_PUBLISHABLE_KEY` | Yes | Openfort publishable key — used for Kora gasless RPC calls |
| `OPENFORT_BASE_URL` | No (default: `https://api.openfort.io`) | Openfort API base URL |
| `SOLANA_RPC_URL` | Yes | Solana RPC endpoint. Use a devnet URL for wallet/balance testing, mainnet for swaps |
| `SOLANA_NETWORK` | Yes | `devnet` or `mainnet` — used for explorer links and cluster context |
| `DATABASE_URL` | No (default: `sqlite:./data/bot.db`) | SQLite connection string |
| `JUPITER_API_KEY` | No | Jupiter API key, if you have one |
| `REFERRAL_FEE_BPS` | No (default: `50`) | Swap fee in basis points routed to your referral account |
| `REFERRAL_ACCOUNT` | No | Your Jupiter Referral account address — must be initialized under the Ultra project (see Known Limitations) |

> **Tip:** Wallet creation, `/balance`, and `/withdraw` work against devnet. `/buy` and `/sell` require `SOLANA_RPC_URL` and `SOLANA_NETWORK` pointed at mainnet, since Jupiter has no devnet liquidity.

---

## Jupiter Referral / Monetization

The bot includes integration with the **Jupiter Referral** mechanism, confirmed working on mainnet (see [Live demo](#live-demo--proof-it-works)).

The current configuration uses a **50 bps (0.5%)** swap commission, routed automatically through the `referralAccount` + `referralFee` parameters on Jupiter's `/order` endpoint — no custody of user funds is required to collect it.

The purpose of this integration is to demonstrate how developers building on top of this starter kit can add an optional and transparent revenue mechanism while keeping the core project open source.

For developers using the repository as a foundation, the referral configuration can be replaced or removed according to their own application model. See [Known Limitations](#known-limitations--gotchas) for a note on correctly initializing your own referral account.

---

## Security

Security is a core design consideration of the project.

### Private keys

The Telegram bot does not store users' private keys in SQLite or in the application source code.

Transaction signing is delegated to Openfort Backend Wallet infrastructure.

### Environment secrets

Sensitive credentials must be provided through environment variables.

Never commit:

```text
.env
```

or any file containing production credentials.

This repository is a starter kit and should not be considered a complete production security framework. In particular, **withdrawals currently execute immediately with no confirmation step** — this is the single most important gap to close before any production use.

### Security roadmap

Security work is an ongoing part of the project.

Planned work includes:

- withdrawal confirmation;
- withdrawal limits;
- address validation;
- rate limiting;
- replay protection;
- transaction simulation;
- improved authentication and authorization;
- protection against common application-layer attacks;
- security audit.

### Account recovery

A planned feature is account recovery after a Telegram ID change.

The intended flow is to associate the new Telegram identity with the existing Openfort account using a verified recovery mechanism, such as an Openfort account ID or verified phone number.

This feature will be implemented with additional authentication and anti-takeover protections.

### Other production considerations

Beyond the security work above, a production deployment should also account for:

- monitoring and alerting;
- audit logging;
- error handling;
- RPC reliability;
- database backups;
- secret rotation;
- Openfort access policies.

---

## Development

Check the project:

```bash
cargo check
```

Format Rust code:

```bash
cargo fmt
```

Run Clippy:

```bash
cargo clippy
```

Build a release version:

```bash
cargo build --release
```

---

## Roadmap

### Already implemented

- [x] Telegram Bot
- [x] Openfort Backend Wallet creation
- [x] SOL and SPL token balances (including Token-2022)
- [x] Jupiter swaps
- [x] SOL withdrawals
- [x] Kora gasless transactions
- [x] SQLite persistence
- [x] Jupiter referral support (confirmed working on mainnet)

### Short-term (3 months)

- [ ] Withdrawal confirmation step
- [ ] Transaction history
- [ ] Account and wallet recovery after Telegram ID change
- [ ] Stronger account authentication and recovery protection
- [ ] Security hardening and protection against common attacks
- [ ] More SPL tokens, with independently verified decimals
- [ ] Token metadata & logos
- [ ] Inline keyboard UI
- [ ] Security audit
- [ ] Full Georgian translation of documentation

### Medium-term (3-6 months)

- [ ] Limit orders
- [ ] DCA (Dollar-Cost Averaging)
- [ ] Advanced withdrawal protection

### Long-term (6+ months)

- [ ] Token sniping
- [ ] Copy trading
- [ ] Production deployment guide
- [ ] Integration tests

The roadmap focuses on expanding the current implementation, improving usability, and strengthening security for production-oriented Telegram-native Solana applications.

## Reusability

The project is released as open source so other developers can inspect the implementation, reuse it, and build on top of it.

The same infrastructure can support other kinds of Telegram-native Solana applications, not just this specific bot — for example:

- a trading assistant;
- a community bot;
- a DeFi interface;
- a payment bot;
- a portfolio assistant;
- a game economy;
- or another Telegram-native Solana application.

The wallet integration, transaction signing, swap, and withdrawal logic is the same across all of these use cases.

---

## Contributing

Contributions are welcome.

If you find a bug, have an improvement, or want to add another Solana integration:

1. Fork the repository.
2. Create a feature branch.
3. Make your changes.
4. Run formatting and checks.
5. Open a pull request.

Example:

```bash
git checkout -b feature/my-feature
cargo fmt
cargo check
git commit -m "Add my feature"
git push origin feature/my-feature
```

---

## Author

Built by [your name / GitHub handle here] as part of a submission for the Solana Foundation Georgia Grant.

Questions, feedback, or collaboration: [add your contact — GitHub issues, Telegram, or email]

---

## License

MIT License.

Copyright © 2026.

---

# Русская версия

## Зачем нужен этот проект?

Создание Telegram-приложения, которое взаимодействует с Solana, требует интеграции нескольких компонентов:

- создание и управление кошельками;
- безопасная подпись транзакций;
- взаимодействие с Solana RPC;
- обмен токенов и маршрутизация ликвидности;
- спонсирование транзакций;
- хранение состояния пользователей;
- инфраструктура Telegram-бота.

Для отдельного разработчика или небольшой команды эта инфраструктурная работа может стать серьёзным барьером ещё до того, как будет проверена сама идея приложения.

**Solana Starter Kit Bot создан для снижения этого барьера.**

Репозиторий представляет собой рабочую open-source реализацию, объединяющую:

- **Openfort Backend Wallets** — инфраструктуру кошельков и подпись транзакций;
- **Jupiter** — обмен токенов и агрегацию ликвидности;
- **Kora** — gasless-транзакции;
- **Solana RPC** — взаимодействие с блокчейном;
- **Rust + Teloxide** — уровень Telegram-приложения;
- **SQLite** — хранение состояния приложения.

Цель проекта — предоставить не закрытого торгового бота, а **переиспользуемую основу**, которую другие разработчики могут форкнуть, изучить, изменить и использовать для собственных Solana-приложений.

### Проблема

> **Разработчик должен иметь возможность быстро экспериментировать с Telegram-native приложениями на Solana, не создавая с нуля всю инфраструктуру кошельков и транзакций.**

---

## Живая демонстрация — доказательство работоспособности

Ниже — транзакции, выполненные в мейннете Solana этой реализацией:

| Операция | Подпись транзакции | Explorer |
|---|---|---|
| Покупка (SOL → USDC, с реферальной комиссией Jupiter) | `2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1` | [Открыть в Solana Explorer](https://explorer.solana.com/tx/2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1) |
| Продажа (USDC → SOL) | `465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz` | [Открыть в Solana Explorer](https://explorer.solana.com/tx/465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz) |
| Вывод (gasless, через Kora) | `27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87` | [Открыть в Solana Explorer](https://explorer.solana.com/tx/27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87) |

---

## Почему Solana?

Главная идея проекта — сделать взаимодействие с блокчейном таким же простым для пользователя, как взаимодействие с обычным Telegram-ботом.

Для этого нужны:

- быстрые транзакции;
- низкая стоимость операций;
- развитая экосистема токенов и DeFi;
- современная инфраструктура для спонсирования транзакций.

Solana особенно хорошо подходит под такой сценарий.

### Быстрые и недорогие операции

Telegram-приложение может выполнять много небольших и частых blockchain-операций.

Пользователь может проверять баланс, обменивать токены, выводить средства и выполнять другие действия прямо из интерфейса Telegram.

Поэтому высокая производительность и низкая стоимость транзакций особенно важны для такого UX.

### Экосистема DeFi

Проект использует **Jupiter** для обмена токенов.

Это позволяет использовать существующую инфраструктуру ликвидности Solana вместо того, чтобы каждому разработчику самостоятельно реализовывать маршрутизацию и поиск ликвидности.

### Gasless-транзакции

Одна из проблем blockchain UX — необходимость держать нативный токен только для оплаты комиссии.

Проект интегрирует **Kora** для поддержки gasless-транзакций.

Вместе с Openfort Backend Wallet это позволяет создавать сценарии, в которых управление комиссией может быть скрыто от пользователя.

### Экосистема разработчика

Для такого приложения уже существует необходимая инфраструктура:

- Solana RPC;
- Jupiter;
- Kora;
- Openfort;
- Rust-инструменты;
- большая экосистема токенов и DeFi.

Этот проект объединяет эти компоненты в одну понятную рабочую реализацию.

### Почему именно Solana?

Проект намеренно является **Solana-first**.

Цель — не создать абстрактный multi-chain wallet framework, а упростить создание именно **Telegram-native приложений для Solana**.

Сочетание низкой стоимости транзакций, производительности, развитой DeFi-экосистемы и инфраструктуры gasless-транзакций делает Solana сильной платформой для такого сценария.

---

## Что даёт этот репозиторий

- **Рабочая реализация**, а не только описание интеграции — в разделе «Живая демонстрация» выше собраны реальные транзакции в мейннете, полученные этим же кодом.
- **Переиспользуемые интеграции.** Логика кошелька, свопов, транзакций и бота разделена на независимые модули, которые можно изучать, менять или заменять по отдельности.
- **Задокументированные ловушки интеграции.** Проблемы, с которыми столкнулись при разработке — расхождения между версиями API, формат payload для подписи, mainnet-only зависимости — зафиксированы в разделе «Известные ограничения» ниже, а не оставлены на самостоятельное обнаружение следующим разработчиком.
- **Открыт по умолчанию.** Код приложения публичен под лицензией MIT — можно форкнуть, изучить и построить поверх него.

Та же инфраструктура может поддержать другие Telegram-native приложения на Solana, не только этого бота — например:

- trading assistants;
- community bots;
- DeFi-интерфейсов;
- payment bots;
- portfolio assistants;
- игровых экономик;
- других Telegram-приложений на Solana.

Логика кошелька, подписи транзакций, свопов и вывода средств одна и та же для всех этих сценариев.

---

## Текущий функционал

На данный момент реализованы:

| Возможность | Описание |
|---|---|
| Backend Wallets | Solana-кошельки через Openfort Backend Wallet |
| Удалённая подпись | Бот не хранит приватные ключи пользователей |
| Jupiter Swap | Покупка и продажа поддерживаемых токенов (SOL ↔ USDC/USDT подтверждено вживую в мейннете) |
| Gasless transactions | Gasless flow через Kora |
| Вывод SOL | Вывод SOL на внешний Solana-адрес |
| Балансы | Просмотр SOL и SPL-токенов (включая Token-2022) |
| Jupiter Referral | Реферальная монетизация обменов — подтверждена работающей в мейннете |
| Разделение секретов | Чувствительные credentials не хранятся в базе |

> **О поддержке токенов:** USDC и USDT полностью протестированы вживую на мейннете. Mint-адреса wBTC и wETH подключены и резолвятся по символу, но сам своп и decimals для них отдельно не верифицированы реальной сделкой — считайте это «должно работать», а не «подтверждено», пока не протестировано лично. Любой другой SPL-токен можно использовать, указав его mint-адрес напрямую вместо символа.

---

## Известные ограничения

Этот раздел специально для тех, кто использует репозиторий как основу для своего проекта. Это реальные ловушки, с которыми столкнулись при разработке — фиксация их здесь должна сэкономить время на отладку, которое ушло на их обнаружение.

- **Jupiter API работает только на мейннете.** На devnet нет реальной ликвидности Jupiter. Создание кошелька, проверка баланса и вывод средств работают на devnet нормально, а `/buy` и `/sell` маршрут не найдут — свопы нужно тестировать на мейннете с небольшими суммами.
- **Поддерживается только режим `ExactIn`.** Эндпоинт `/swap/v2/order` не поддерживает `ExactOut`. Можно указать «потрать ровно столько», но не «получи ровно столько» — стоит закладывать это в UX сразу, а не бороться с ограничением.
- **На подпись нужно отправлять именно message транзакции, а не всю сериализованную транзакцию.** При делегировании подписи в Openfort backend wallet через `/sign` нужно хешировать и отправлять `transaction.message.serialize()`, а не `bincode::serialize(&transaction)`. Подпись не того payload'а даёт подпись, которая молча не проходит верификацию в сети.
- **Структура REST API Openfort не всегда совпадает с тем, что предполагает публичная документация, а v1 и v2 эндпоинты отличаются.** Имена полей для одной и той же концептуальной операции (`player` vs `user`, snake_case vs camelCase в claims, hex vs base64 кодирование payload) менялись между версиями API. При интеграции стоит сверяться с реальными исходниками SDK (`openapi-client/generated/`), а не полагаться только на публичную документацию или догадки ИИ-агента.
- **Вывод средств выполняется мгновенно, без подтверждения.** Сейчас нет шага «вы уверены?» перед отправкой `/withdraw`. Это осознанное упрощение MVP, а не недосмотр — перед использованием с реальными пользователями стоит добавить подтверждение.
- **Реферальный аккаунт Jupiter должен быть инициализирован под правильным ончейн-«проектом».** Создание аккаунта через веб-дашборд может зарегистрировать его не под тем проектом, который требует Meta-Aggregator API (`/order` + `/execute`). Используй `@jup-ag/referral-sdk` с `projectPubKey = DkiqsTrw1u1bYFumumC7sCG2S8K25qc2vemJFHyW2wJc` (Jupiter Ultra Referral Project), если созданный через дашборд аккаунт отклоняется с ошибкой «Invalid referralAccount» или несовпадением проекта.

---

## Архитектура кошелька

Проект использует **Openfort Backend Wallets**.

Ключевое свойство архитектуры: Telegram-бот сам **не хранит и напрямую не управляет приватными ключами пользователей**.

Приложение хранит информацию, необходимую для связи Telegram-пользователя с его Openfort account/wallet, а операции подписи делегируются инфраструктуре Openfort.

Концептуально:

```text
Пользователь Telegram
        │
        ▼
Telegram Bot
        │
        │ ссылка на wallet/account
        ▼
Openfort Backend Wallet
        │
        │ подпись транзакции
        ▼
Solana
```

> **Важно:** этот репозиторий является примером интеграции. Перед production-развёртыванием необходимо отдельно оценить модель безопасности, контроль доступа, аутентификацию, rate limiting, политики транзакций и управление ключевой инфраструктурой.

---

## Технологический стек

| Технология | Роль |
|---|---|
| **Rust** | Логика приложения и бэкенда |
| **Teloxide** | Фреймворк для Telegram-бота |
| **Solana SDK** | Взаимодействие с блокчейном Solana |
| **Openfort** | Инфраструктура Backend Wallet и подпись транзакций |
| **Jupiter** | Обмен токенов и агрегация ликвидности |
| **Kora** | Инфраструктура gasless-транзакций |
| **SQLite** | Хранение состояния приложения |
| **Reqwest** | HTTP-коммуникация |
| **Serde** | Сериализация и модели API |

---

## Команды

| Команда | Описание |
|---|---|
| `/start` | Приветствие и доступные команды |
| `/create_wallet` | Создание нового Solana-кошелька |
| `/balance` | Проверка баланса SOL |
| `/tokens` | Балансы всех SPL-токенов (SOL + Token / Token-2022) |
| `/buy <token> <amount>` | Потратить SOL на покупку токена |
| `/sell <token> <amount>` | Продажа поддерживаемого токена за SOL |
| `/withdraw <amount> <address>` | Вывод SOL на внешний Solana-адрес |

Пример:

```text
/buy USDC 0.1
```

```text
/sell USDC 5
```

```text
/withdraw 0.1 <SOLANA_ADDRESS>
```

---

## Быстрый старт

### 1. Клонирование репозитория

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
```

### 2. Настройка переменных окружения

```bash
cp .env.example .env
```

Заполните переменные — полный список см. в разделе [Конфигурация](#конфигурация) ниже.

Не добавляйте `.env` в Git.

### 3. Сборка

```bash
cargo build --release
```

### 4. Запуск

```bash
cargo run
```

---

## Конфигурация

| Переменная | Обязательна | Описание |
|---|---|---|
| `TELEGRAM_BOT_TOKEN` | Да | Токен от [@BotFather](https://t.me/BotFather) |
| `OPENFORT_SECRET_KEY` | Да | Секретный ключ проекта Openfort (`sk_...`) |
| `OPENFORT_WALLET_SECRET` | Да | Wallet secret Openfort для подписи `X-Wallet-Auth` JWT |
| `OPENFORT_PUBLISHABLE_KEY` | Да | Publishable key Openfort — нужен для gasless-вызовов через Kora |
| `OPENFORT_BASE_URL` | Нет (по умолчанию `https://api.openfort.io`) | Базовый URL API Openfort |
| `SOLANA_RPC_URL` | Да | RPC-эндпоинт Solana. Devnet — для тестов кошелька/баланса, mainnet — для свопов |
| `SOLANA_NETWORK` | Да | `devnet` или `mainnet` |
| `DATABASE_URL` | Нет (по умолчанию `sqlite:./data/bot.db`) | Строка подключения SQLite |
| `JUPITER_API_KEY` | Нет | API-ключ Jupiter, если есть |
| `REFERRAL_FEE_BPS` | Нет (по умолчанию `50`) | Комиссия свопа в bps на твой реферальный аккаунт |
| `REFERRAL_ACCOUNT` | Нет | Твой реферальный аккаунт Jupiter — должен быть инициализирован под Ultra-проектом (см. «Известные ограничения») |

> **Совет:** создание кошелька, `/balance` и `/withdraw` работают на devnet. `/buy` и `/sell` требуют `SOLANA_RPC_URL`/`SOLANA_NETWORK`, указывающие на mainnet — у Jupiter нет ликвидности на devnet.

---

## Безопасность

Telegram-бот не хранит приватные ключи пользователей в SQLite или исходном коде.

Подпись транзакций выполняется через инфраструктуру Openfort Backend Wallet.

Секретные данные передаются через environment variables.

**Важнее всего:** вывод средств сейчас выполняется мгновенно, без подтверждения — это главный пробел, который стоит закрыть перед любым production-использованием.

### Security roadmap

Работа над безопасностью — постоянная часть проекта.

В планах:

- подтверждение перед выводом средств;
- лимиты на вывод;
- валидация адресов;
- rate limiting;
- защита от replay-атак;
- симуляция транзакций перед отправкой;
- усиленная аутентификация и авторизация;
- защита от распространённых атак на уровне приложения;
- security audit.

### Восстановление аккаунта

Планируемая функция — восстановление доступа к аккаунту после смены Telegram ID.

Предполагаемый сценарий: связать новую Telegram-идентичность с существующим Openfort-аккаунтом через проверенный механизм восстановления — например, Openfort account ID или подтверждённый номер телефона.

Эта функция будет реализована с дополнительной аутентификацией и защитой от захвата аккаунта (anti-takeover).

### Прочие эксплуатационные аспекты

Помимо безопасности, для production-развёртывания стоит также предусмотреть:

- мониторинг и алертинг;
- аудит операций;
- обработку ошибок;
- надёжность RPC;
- резервное копирование;
- ротацию секретов;
- политики доступа Openfort.

---

## Разработка

Проверка проекта:

```bash
cargo check
```

Форматирование кода:

```bash
cargo fmt
```

Запуск Clippy:

```bash
cargo clippy
```

---

## Roadmap

### Уже реализовано

- [x] Telegram Bot
- [x] Создание Openfort Backend Wallet
- [x] Балансы SOL и SPL-токенов (включая Token-2022)
- [x] Jupiter swaps
- [x] Вывод SOL
- [x] Kora gasless transactions
- [x] SQLite persistence
- [x] Jupiter referral (подтверждено работающим в мейннете)

### Краткосрочные цели (3 месяца)

- [ ] Подтверждение перед выводом средств
- [ ] История транзакций
- [ ] Восстановление аккаунта и кошелька после смены Telegram ID
- [ ] Усиленная аутентификация и защита восстановления аккаунта
- [ ] Усиление безопасности и защита от распространённых атак
- [ ] Больше SPL-токенов с независимо проверенными decimals
- [ ] Метаданные токенов и логотипы
- [ ] Inline keyboard UI
- [ ] Security audit
- [ ] Полный перевод документации на грузинский

### Среднесрочные цели (3–6 месяцев)

- [ ] Limit orders
- [ ] DCA (Dollar-Cost Averaging)
- [ ] Расширенная защита вывода средств

### Долгосрочные цели (6+ месяцев)

- [ ] Token sniping
- [ ] Copy trading
- [ ] Production deployment guide
- [ ] Integration tests

---

## Вклад в проект

Pull requests и предложения по улучшению приветствуются.

```bash
git checkout -b feature/my-feature
cargo fmt
cargo check
git commit -m "Add my feature"
git push origin feature/my-feature
```

После этого создайте Pull Request.

---

## Автор

Разработано [твоё имя / GitHub] в рамках заявки на Solana Foundation Georgia Grant.

Вопросы, отзывы, сотрудничество: [добавь контакт — GitHub issues, Telegram или email]

---

## Лицензия

MIT License.

Copyright © 2026.
