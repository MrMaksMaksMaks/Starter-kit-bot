# Solana Starter Kit Bot

A reusable, open-source reference implementation for building Telegram-native Solana applications — without building wallet, signing, swap, and gasless infrastructure from scratch.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)
![Status](https://img.shields.io/badge/status-working-brightgreen.svg)
[![GitHub](https://img.shields.io/badge/GitHub-repository-181717?logo=github)](https://github.com/MrMaksMaksMaks/Starter-kit-bot)

---

## What is it?

Solana Starter Kit Bot is an open-source, working Telegram bot that gives Solana developers a reusable starting point for wallet creation, transaction signing, token swaps, and sponsored (gasless) withdrawals — instead of requiring every new project to build that infrastructure from zero.

Building a Telegram application that talks to Solana normally means integrating wallet infrastructure, secure signing, RPC communication, swap routing, transaction sponsorship, and persistent state before a single feature idea can even be tested. This repository provides a working reference for all of that, built with Rust and Teloxide, using Openfort Backend Wallets for signing, Jupiter for swaps, and Kora for sponsored transactions.

Solana fits this use case specifically because interactions are cheap and fast enough for a conversational, frequent-action UX, Jupiter's liquidity is deep enough that swaps don't need custom routing, and Kora's sponsorship infrastructure lets transaction fees be abstracted away from the end user for supported flows.

---

## What works today

| Feature | Status |
|---|---|
| Telegram bot | ✅ |
| Openfort Backend Wallet creation | ✅ |
| SOL balance | ✅ |
| SPL / Token-2022 balances | ✅ |
| Jupiter swaps (buy/sell) | ✅ Mainnet |
| Kora sponsored (gasless) withdrawals | ✅ |
| Jupiter Referral (optional) | ✅ Mainnet |

> Transaction fees for supported flows (like withdrawals) are sponsored by Kora, so the user does not need to hold SOL to pay the network fee.

---

## Live mainnet proof

| Flow | Signature | Explorer |
|---|---|---|
| Buy (SOL → USDC, with referral fee) | `2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1` | [View](https://explorer.solana.com/tx/2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1) |
| Sell (USDC → SOL) | `465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz` | [View](https://explorer.solana.com/tx/465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz) |
| Sponsored withdrawal (via Kora) | `27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87` | [View](https://explorer.solana.com/tx/27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87) |

---

## Demo

| Feature | Screenshot |
|---------|------------|
| **Main Menu** | <img src="./images/Screenshot_Start.jpg" width="300"/> |
| **Create Wallet** | <img src="./images/Screenshot_Create_wallet.jpg" width="300"/> |
| **Buy Tokens** | <img src="./images/Screenshot_Buy_USDC.jpg" width="300"/> |
| **SOL Balance** | <img src="./images/Screenshot_Balance.jpg" width="300"/> |
| **Token Portfolio** | <img src="./images/Screenshot_Tokens.jpg" width="300"/> |

---

## Wallet architecture

Before the diagrams, a quick note on terminology — the same word ("wallet," "account") gets used loosely across this space, and it's worth being precise once:

| Term | Meaning |
|---|---|
| Telegram user | The application-level identity — a Telegram account interacting with the bot |
| Openfort account | The wallet-infrastructure identity that owns and signs for a Solana wallet |
| Solana wallet | The on-chain address (public key) that holds SOL / SPL tokens |
| SQLite record | The mapping this application stores between a Telegram user and their Openfort account / Solana wallet |

The important security property: the Telegram bot itself does **not** store or directly manage users' private keys. The application stores the mapping above; signing operations are delegated to Openfort's wallet infrastructure.

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

**Create wallet flow:**

```text
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

## Transaction flows

### Buy / Sell

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

### Withdraw

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

| Technology | Role |
|---|---|
| **Rust** | Application and backend logic |
| **Teloxide** | Telegram Bot framework |
| **Solana SDK** | Solana blockchain interaction |
| **Openfort** | Backend Wallet infrastructure and transaction signing |
| **Jupiter** | Token swap and liquidity aggregation |
| **Kora** | Sponsored (gasless) transaction infrastructure |
| **SQLite** | Persistent local application state |
| **Reqwest** | HTTP communication |
| **Serde** | Serialization and API models |

---

## Quick Start

### Prerequisites

- Rust (stable toolchain)
- A Telegram bot token from [@BotFather](https://t.me/BotFather)
- An Openfort project — secret key, wallet secret, and publishable key
- For swaps: a Solana mainnet RPC endpoint and a small amount of real SOL to test with (Jupiter has no devnet liquidity — see [Known Integration Gotchas](#known-integration-gotchas))

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

Fill in the required variables — see [Configuration](#configuration) below for the full list.

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
| `REFERRAL_ACCOUNT` | No | Your Jupiter Referral account address — must be initialized under the Ultra project (see [Known Integration Gotchas](#known-integration-gotchas)) |

---

## Jupiter Referral / Monetization

This starter kit includes an optional integration with Jupiter's Referral mechanism, confirmed working on mainnet (see [Live mainnet proof](#live-mainnet-proof)).

It is disabled by simply omitting `REFERRAL_ACCOUNT` from `.env` — the core wallet, swap, and withdrawal infrastructure works identically with or without it.

The integration exists to demonstrate one way a developer could add a transparent revenue mechanism on top of the reusable infrastructure, without taking custody of user funds — not as the primary purpose of the repository. The default configuration routes 50 bps (0.5%) of each swap to a referral account via Jupiter's `referralAccount` / `referralFee` parameters.

Developers using this repository as a foundation can remove or replace this configuration entirely.

---

## Security

Security is a core design consideration of the project, and an area of active, ongoing work.

### Current model

The Telegram bot does not store users' private keys in SQLite or in the application source code. Signing is delegated to Openfort Backend Wallet infrastructure; sensitive credentials are provided through environment variables and must never be committed.

Given this architecture, the realistic attack surfaces to design around include:

- Telegram account takeover;
- unauthorized withdrawal requests;
- forged or replayed signing requests;
- malicious destination addresses;
- compromised application credentials (`OPENFORT_SECRET_KEY`, `OPENFORT_WALLET_SECRET`);
- RPC / API abuse;
- transaction manipulation between construction and signing.

### Known limitations

This repository is a starter kit, not a hardened production system. Specifically, today:

- **withdrawals execute immediately, with no confirmation step;**
- there is no rate limiting on Telegram commands;
- there is no replay protection beyond what Solana's own blockhash expiry provides;
- there is no transaction simulation before signing.

These are deliberate MVP simplifications, not oversights — treat them as required work, not optional polish, before exposing this to real users with real funds.

### Production hardening

Planned work:

- withdrawal confirmation and configurable limits;
- address validation;
- rate limiting;
- replay protection;
- transaction simulation before signing;
- improved authentication and authorization;
- protection against common application-layer attacks;
- monitoring, alerting, and audit logging;
- database backups and secret rotation;
- independent security audit.

**Account recovery** — a planned feature is account recovery after a Telegram ID change. This is a sensitive operation: the design must independently verify ownership of the original account before granting access to a new Telegram identity, plus include anti-takeover protections. The specific verification mechanism is intentionally not finalized here — it will be detailed in a dedicated design document before implementation, rather than committed to prematurely in this README.

---

## Known Integration Gotchas

Real integration pitfalls discovered while building this project. Documenting them here is meant to save the next developer the debugging time it took to find them.

- **Jupiter API is mainnet-only.** There is no Jupiter liquidity on devnet. Wallet creation, balance checks, and withdrawals work fine on devnet, but `/buy` and `/sell` will not find a route — test swaps on mainnet with small amounts.
- **Only `ExactIn` swap mode is supported.** The `/swap/v2/order` endpoint used here does not support `ExactOut`. You can specify "spend exactly this much," not "receive exactly this much."
- **Sign the transaction's *message*, not the full serialized transaction.** When delegating signing to Openfort's backend wallet `/sign` endpoint, hash and send `transaction.message.serialize()` — not `bincode::serialize(&transaction)`. Signing the wrong payload produces a signature that silently fails on-chain verification.
- **Openfort's REST API structure is not always what the public docs suggest, and v1/v2 endpoints differ.** Field names for the same conceptual operation (e.g. `player` vs `user`, snake_case vs camelCase claims, hex vs base64 payload encoding) have changed between API versions. Cross-check against the actual SDK source (`openapi-client/generated/`) rather than relying solely on public documentation.
- **The Jupiter Referral account must be initialized under the correct on-chain "project."** Creating a referral account through the web dashboard may register it under the wrong project for the Meta-Aggregator (`/order` + `/execute`) API. Use `@jup-ag/referral-sdk` with `projectPubKey = DkiqsTrw1u1bYFumumC7sCG2S8K25qc2vemJFHyW2wJc` (Jupiter Ultra Referral Project) if a dashboard-created account is rejected with "Invalid referralAccount" or a project mismatch error.

---

## Roadmap

Scope is intentionally focused: this open-source repository is meant to be a secure, reusable **foundation** — not a full-featured trading bot. Advanced trading features are being developed separately as a commercial product built on top of this same open-source base; they are not part of this repository's roadmap, and are called out explicitly below rather than left ambiguous.

### Core

- [ ] Withdrawal confirmation step
- [ ] Withdrawal limits
- [ ] Transaction history
- [ ] Account recovery after Telegram ID change (independently verified ownership + anti-takeover protections)
- [ ] More SPL tokens, with independently verified decimals
- [ ] Token metadata & logos

### DX (developer experience)

- [ ] Integration tests
- [ ] Inline keyboard UI
- [ ] Georgian translation of documentation

### Trading

Advanced trading features — limit orders, DCA, token sniping, copy trading — are intentionally **out of scope** for this open-source starter kit. They carry meaningfully higher security, execution-reliability, and abuse-prevention requirements than the core wallet / swap / withdraw flows this repository demonstrates. They are being explored separately, as part of a commercial product built on top of this same open-source foundation.

### Production

- [ ] Rate limiting reference implementation
- [ ] Replay protection
- [ ] Transaction simulation before send
- [ ] Security audit
- [ ] Production deployment guide

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

## License

MIT License.

Copyright © 2026.

---

## Author

Built by MAKSIM GORBUNOV

Questions, feedback, or collaboration: jobpostgm@gmail.com
---

# RU

## Что это за проект?

Solana Starter Kit Bot — открытый, рабочий Telegram-бот, который даёт Solana-разработчикам переиспользуемую основу для создания кошельков, подписи транзакций, обмена токенов и спонсируемого (gasless) вывода средств — вместо того чтобы каждый новый проект строил эту инфраструктуру с нуля.

Создание Telegram-приложения, взаимодействующего с Solana, обычно означает интеграцию инфраструктуры кошельков, безопасной подписи, RPC-коммуникации, маршрутизации свопов, спонсирования транзакций и хранения состояния — ещё до того, как можно проверить саму идею продукта. Этот репозиторий даёт рабочую реализацию всего этого: на Rust и Teloxide, с Openfort Backend Wallets для подписи, Jupiter для свопов и Kora для спонсируемых транзакций.

Solana хорошо подходит именно для такого сценария: операции достаточно дешёвые и быстрые для разговорного, частого UX, ликвидность через Jupiter достаточно глубокая, чтобы не писать собственную маршрутизацию, а инфраструктура спонсирования Kora позволяет скрыть управление комиссией от конечного пользователя для поддерживаемых сценариев.

---

## Что работает уже сейчас

| Возможность | Статус |
|---|---|
| Telegram-бот | ✅ |
| Создание Openfort Backend Wallet | ✅ |
| Баланс SOL | ✅ |
| Балансы SPL / Token-2022 | ✅ |
| Jupiter swaps (buy/sell) | ✅ Mainnet |
| Kora — спонсируемый (gasless) вывод | ✅ |
| Jupiter Referral (опционально) | ✅ Mainnet |

> Комиссия сети для поддерживаемых сценариев (например, вывода средств) оплачивается Kora — пользователю не нужно держать SOL для оплаты газа.

---

## Подтверждение в мейннете

| Операция | Подпись транзакции | Explorer |
|---|---|---|
| Покупка (SOL → USDC, с реферальной комиссией) | `2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1` | [Открыть](https://explorer.solana.com/tx/2Sk3FCmNbokLewrauVDoBohUhAMkbcfDqA6iMbsVv9DSnbtPXqAsMkV3zPfziwWL74c5QmW2SvDJKxzegKFXczE1) |
| Продажа (USDC → SOL) | `465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz` | [Открыть](https://explorer.solana.com/tx/465yUkJD5FSiQSsDmcan7Qq1VDkkqTcfyATV1mRpezeA5ZXVQM2VCF2qRSatVbBUXRnQEb1yYQ5KH42KN66scWBz) |
| Спонсируемый вывод (через Kora) | `27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87` | [Открыть](https://explorer.solana.com/tx/27PAXPkFoD97ZBcVemXN3o3B1eMAsdJkmmgFUe9dKGYjE8PkwgNK4JQJNt2HCGMmmMTsoQiHSETUwuJD98yC9x87) |

---

## Демо

| Функция | Скриншот |
|---------|----------|
| **Главное меню** | ![Главное меню](./images/Screenshot_Start.jpg) |
| **Создание кошелька** | ![Кошелек](./images/Screenshot_Create_wallet.jpg) |
| **Покупка токена** | ![Покупка](./images/Screenshot_Buy_USDC.jpg) |
| **Баланс кошелька SOL** | ![Баланс SOL](./images/Screenshot_Balance.jpg) |
| **Баланс кошелька Tokens** | ![Баланс Tokens](./images/Screenshot_Tokens.jpg) |

---

## Архитектура кошелька

Прежде чем перейти к диаграммам — короткая сверка терминов, потому что слова «кошелёк»/«аккаунт» в этой области используются вольно, а здесь стоит быть точным:

| Термин | Значение |
|---|---|
| Telegram-пользователь | Идентичность на уровне приложения — Telegram-аккаунт, взаимодействующий с ботом |
| Openfort account | Идентичность на уровне инфраструктуры кошелька, которая владеет и подписывает за Solana-кошелёк |
| Solana wallet | Ончейн-адрес (публичный ключ), на котором хранятся SOL / SPL-токены |
| Запись в SQLite | Связка, которую хранит приложение, между Telegram-пользователем и его Openfort account / Solana wallet |

Ключевое свойство безопасности: Telegram-бот сам **не хранит и напрямую не управляет** приватными ключами пользователей. Приложение хранит только связку выше; операции подписи делегируются инфраструктуре Openfort.

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

**Создание кошелька:**

```text
/create_wallet
      │
      ▼
Rust-приложение
      │
      ▼
Openfort Backend Wallet API
      │
      ▼
Solana wallet/account создан
      │
      ▼
Адрес возвращён пользователю, связка сохранена в SQLite
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
| **Kora** | Инфраструктура спонсируемых (gasless) транзакций |
| **SQLite** | Хранение состояния приложения |
| **Reqwest** | HTTP-коммуникация |
| **Serde** | Сериализация и модели API |

---

## Быстрый старт

### Требования

- Rust (stable toolchain)
- Токен Telegram-бота от [@BotFather](https://t.me/BotFather)
- Проект Openfort — secret key, wallet secret и publishable key
- Для свопов: mainnet RPC-эндпоинт Solana и немного реального SOL для теста (у Jupiter нет ликвидности на devnet — см. «Технические уроки интеграции»)

### Установка

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
cargo build --release
```

### Настройка

```bash
cp .env.example .env
```

Заполните переменные — полный список см. в разделе «Конфигурация» ниже. Не добавляйте `.env` в Git.

### Первые команды

```bash
cargo run
```

Дальше в Telegram:

1. `/start` — список доступных команд
2. `/create_wallet` — создаёт Solana-кошелёк через Openfort
3. `/balance` — подтверждает, что кошелёк создан и читается
4. `/buy USDC 0.01` — небольшой реальный своп в мейннете, после пополнения кошелька

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
| `REFERRAL_ACCOUNT` | Нет | Твой реферальный аккаунт Jupiter — должен быть инициализирован под Ultra-проектом (см. «Технические уроки интеграции») |

---

## Jupiter Referral / Монетизация

Стартер-кит включает опциональную интеграцию с реферальным механизмом Jupiter, подтверждённую работающей в мейннете (см. «Подтверждение в мейннете»).

Отключается простым отсутствием `REFERRAL_ACCOUNT` в `.env` — базовая инфраструктура кошелька, свопов и вывода работает идентично с ней и без неё.

Интеграция существует, чтобы показать один из способов добавить прозрачный источник дохода поверх переиспользуемой инфраструктуры, не беря на себя кастодию средств пользователя — а не как основную цель репозитория. Дефолтная конфигурация направляет 50 bps (0,5%) с каждого свопа на реферальный аккаунт через параметры Jupiter `referralAccount`/`referralFee`.

Разработчики, использующие репозиторий как основу, могут убрать или заменить эту конфигурацию полностью.

---

## Безопасность

Безопасность — ключевая часть проекта и область постоянной, активной работы.

### Текущая модель

Telegram-бот не хранит приватные ключи пользователей ни в SQLite, ни в исходном коде. Подпись делегируется инфраструктуре Openfort Backend Wallet; чувствительные данные передаются через переменные окружения и никогда не должны попадать в git.

С учётом такой архитектуры, реалистичные поверхности атаки, под которые стоит проектировать защиту:

- захват Telegram-аккаунта;
- неавторизованные запросы на вывод средств;
- подделанные или replay-запросы на подпись;
- вредоносные адреса получателя;
- компрометация credentials приложения (`OPENFORT_SECRET_KEY`, `OPENFORT_WALLET_SECRET`);
- злоупотребление RPC/API;
- манипуляция транзакцией между сборкой и подписью.

### Известные ограничения

Этот репозиторий — стартер-кит, а не защищённая production-система. Конкретно сейчас:

- **вывод средств выполняется мгновенно, без подтверждения;**
- нет rate limiting на команды Telegram;
- нет защиты от replay сверх того, что даёт естественное истечение blockhash в Solana;
- нет симуляции транзакции перед подписью.

Это осознанные упрощения MVP, а не недосмотр — стоит воспринимать их как обязательную работу, а не опциональную полировку, прежде чем показывать бота реальным пользователям с реальными деньгами.

### Усиление для production

Планируемая работа:

- подтверждение вывода средств и настраиваемые лимиты;
- валидация адресов;
- rate limiting;
- защита от replay-атак;
- симуляция транзакции перед подписью;
- усиленная аутентификация и авторизация;
- защита от распространённых атак на уровне приложения;
- мониторинг, алертинг и аудит операций;
- резервное копирование БД и ротация секретов;
- независимый security audit.

**Восстановление аккаунта** — планируемая функция: восстановление доступа после смены Telegram ID. Это чувствительная операция: механизм должен независимо подтверждать владение исходным аккаунтом перед выдачей доступа новой Telegram-идентичности, плюс включать защиту от захвата (anti-takeover). Конкретный механизм верификации намеренно не зафиксирован здесь — он будет описан в отдельном design-документе перед реализацией, а не преждевременно закреплён в этом README.

---

## Технические уроки интеграции

Реальные ловушки, обнаруженные при разработке этого проекта. Фиксация их здесь должна сэкономить следующему разработчику время на отладку, которое ушло на их обнаружение.

- **Jupiter API работает только на мейннете.** На devnet нет реальной ликвидности Jupiter. Создание кошелька, проверка баланса и вывод средств работают на devnet нормально, а `/buy` и `/sell` маршрут не найдут — свопы нужно тестировать на мейннете с небольшими суммами.
- **Поддерживается только режим `ExactIn`.** Эндпоинт `/swap/v2/order` не поддерживает `ExactOut`. Можно указать «потрать ровно столько», но не «получи ровно столько».
- **На подпись нужно отправлять именно message транзакции, а не всю сериализованную транзакцию.** При делегировании подписи в Openfort backend wallet через `/sign` нужно хешировать и отправлять `transaction.message.serialize()`, а не `bincode::serialize(&transaction)`. Подпись не того payload'а даёт подпись, которая молча не проходит верификацию в сети.
- **Структура REST API Openfort не всегда совпадает с тем, что предполагает публичная документация, а v1 и v2 эндпоинты отличаются.** Имена полей для одной и той же концептуальной операции (`player` vs `user`, snake_case vs camelCase в claims, hex vs base64 кодирование payload) менялись между версиями API. Стоит сверяться с реальными исходниками SDK (`openapi-client/generated/`), а не полагаться только на публичную документацию.
- **Реферальный аккаунт Jupiter должен быть инициализирован под правильным ончейн-«проектом».** Создание аккаунта через веб-дашборд может зарегистрировать его не под тем проектом, который требует Meta-Aggregator API (`/order` + `/execute`). Используй `@jup-ag/referral-sdk` с `projectPubKey = DkiqsTrw1u1bYFumumC7sCG2S8K25qc2vemJFHyW2wJc` (Jupiter Ultra Referral Project), если созданный через дашборд аккаунт отклоняется с ошибкой «Invalid referralAccount» или несовпадением проекта.

---

## Roadmap

Скоуп намеренно ограничен: этот открытый репозиторий задуман как защищённая, переиспользуемая **основа**, а не полнофункциональный трейдинг-бот. Продвинутые торговые функции разрабатываются отдельно как коммерческий продукт поверх этой же открытой основы — они не входят в roadmap этого репозитория, и это явно указано ниже, а не оставлено недосказанным.

### Core

- [ ] Подтверждение перед выводом средств
- [ ] Лимиты на вывод
- [ ] История транзакций
- [ ] Восстановление аккаунта после смены Telegram ID (независимо проверенное владение + anti-takeover защита)
- [ ] Больше SPL-токенов с независимо проверенными decimals
- [ ] Метаданные токенов и логотипы

### DX (опыт разработчика)

- [ ] Integration tests
- [ ] Inline keyboard UI
- [ ] Перевод документации на грузинский

### Trading

Продвинутые торговые функции — limit orders, DCA, token sniping, copy trading — намеренно **вне скоупа** этого открытого стартер-кита. Они требуют заметно более высоких требований к безопасности, надёжности исполнения и защите от злоупотреблений, чем базовые потоки wallet/swap/withdraw, которые демонстрирует этот репозиторий. Они прорабатываются отдельно, как часть коммерческого продукта поверх этой же открытой основы.

### Production

- [ ] Референсная реализация rate limiting
- [ ] Защита от replay-атак
- [ ] Симуляция транзакции перед отправкой
- [ ] Security audit
- [ ] Production deployment guide

---

## Переиспользуемость

Проект выпущен как open source, чтобы другие разработчики могли изучить реализацию, переиспользовать её и построить поверх неё.

Та же инфраструктура может поддержать другие Telegram-native приложения на Solana, не только этого бота — например:

- community bot;
- DeFi-интерфейс;
- payment bot;
- portfolio assistant;
- игровую экономику;
- или другое Telegram-native приложение на Solana.

Логика кошелька, подписи транзакций, свопов и вывода средств одна и та же для всех этих сценариев.

---

## Вклад в проект

Pull requests, баг-репорты и предложения по улучшению приветствуются.

```bash
git checkout -b feature/my-feature
cargo fmt
cargo check
git commit -m "Add my feature"
git push origin feature/my-feature
```

После этого создайте Pull Request.

---

## Лицензия

MIT License.

Copyright © 2026.

---

## Автор

Разработчик: МАКСИМ ГОРБУНОВ

Вопросы, отзывы, сотрудничество: jobpostgm@gmail.com
