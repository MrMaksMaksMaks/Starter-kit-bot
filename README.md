# Solana Starter Kit Bot 🤖

[![Rust](https://img.shields.io/badge/Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-3F3F3F?logo=solana)](https://solana.com/)
[![Telegram](https://img.shields.io/badge/Telegram-2CA5E0?logo=telegram)](https://telegram.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Open-source Telegram bot starter kit for building Solana wallets, token swaps and DeFi applications with Rust.**

The project combines Telegram, Solana, Openfort Backend Wallets, Jupiter swaps and Kora gasless transactions into a single application that can be used as a starting point for building your own Solana Telegram bot.

---

## ✨ Features

| Feature | Description |
|---|---|
| 🔐 **Openfort Backend Wallets** | Create and manage Solana wallets through Openfort |
| 🛡️ **Non-custodial user experience** | The bot does not receive or store users' Solana private keys |
| 💱 **Jupiter Swaps** | Buy and sell supported tokens using Jupiter |
| 💸 **SOL Withdrawals** | Send SOL to any valid Solana address |
| ⛽ **Gasless Transactions** | Use Kora infrastructure for sponsored Solana transactions |
| 📊 **Balance Checking** | View SOL and SPL token balances |
| 💰 **Jupiter Referral** | Optional referral fee integration |
| 🗄️ **SQLite** | Lightweight local persistence |
| 🦀 **Rust** | Fast, safe and memory-efficient backend |

---

# 🤖 Telegram Commands

## Wallet

### `/start`

Show the welcome message and available commands.

### `/create_wallet`

Create a new Solana Backend Wallet through Openfort.

---

## Balance

### `/balance`

Show the user's SOL and supported SPL token balances.

---

## Swaps

### `/buy <token> <amount>`

Buy a supported token using SOL.

Example:

```text
/buy USDC 1
```

### `/sell <token> <amount>`

Sell a supported token for SOL.

Example:

```text
/sell USDC 1
```

Currently supported tokens include:

- USDC
- USDT
- wBTC
- wETH

Swap routing and liquidity are provided by Jupiter.

---

## Withdrawals

### `/withdraw <amount> <address>`

Withdraw SOL from the user's Openfort wallet to a Solana address.

Example:

```text
/withdraw 0.1 7xKXtg2CW87d97TXJSDpbD5jBkheT...
```

The transaction is signed through the Openfort Backend Wallet infrastructure.

When Kora sponsorship is configured, the transaction can be processed through a gasless flow where the configured fee payer covers the Solana network fee.

---

# 🏗️ Architecture

The bot is built around several independent components:

```text
┌──────────────────────────────────────────────┐
│                  Telegram User               │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────┐
│               Telegram Bot API               │
│                  Teloxide                    │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
┌──────────────────────────────────────────────┐
│                Rust Application              │
│                                              │
│  /start /create_wallet /balance              │
│  /buy /sell /withdraw                        │
└──────────────┬──────────────┬────────────────┘
               │              │
               ▼              ▼
        ┌────────────┐   ┌────────────────────┐
        │   SQLite   │   │   External APIs    │
        │            │   │                    │
        │ User state │   │ Openfort           │
        │ Wallet IDs │   │ Jupiter            │
        │ Addresses  │   │ Kora               │
        └────────────┘   │ Solana RPC         │
                         └────────────────────┘
```

---

# 🔐 Wallet Architecture

This project uses **Openfort Backend Wallets** to provide wallet infrastructure without requiring the Telegram bot to handle users' Solana private keys.

The basic architecture is:

```text
                    User
                     │
                     ▼
              Telegram Bot
                     │
                     │ wallet operations
                     ▼
              ┌─────────────┐
              │  Openfort   │
              │   Backend   │
              │   Wallet    │
              └──────┬──────┘
                     │
                     │ signing
                     ▼
                  Solana
```

The application works with wallet/account identifiers and public addresses.

The bot does not need to store:

```text
Solana private keys
Seed phrases
Recovery phrases
```

> **Important:** "Non-custodial" here refers to the bot not holding users' Solana private keys. Openfort Backend Wallets are managed wallet infrastructure, so developers should review Openfort's custody, security and policy model before using the system in production.

---

# ⛽ Gasless Transactions with Kora

The project supports Kora for gasless Solana transaction flows.

A simplified flow looks like this:

```text
User
 │
 ▼
Telegram Bot
 │
 ▼
Openfort Backend Wallet
 │
 │ user transaction signature
 ▼
Kora
 │
 │ fee payer / sponsorship
 ▼
Solana RPC
 │
 ▼
Solana Network
```

Kora can act as the transaction fee payer, allowing applications to sponsor Solana transaction fees for users.

This can provide a much simpler user experience because users do not necessarily need to maintain SOL solely to pay transaction fees.

> Gasless transaction availability depends on the configured Kora infrastructure, sponsorship policies and fee-payer configuration.

---

# 💱 Jupiter Integration

Token swaps are powered by Jupiter.

The bot uses Jupiter to find swap routes and execute token swaps on Solana.

Examples:

```text
SOL → USDC
USDC → SOL

SOL → USDT
USDT → SOL

SOL → wBTC
wBTC → SOL

SOL → wETH
wETH → SOL
```

Jupiter provides swap routing and liquidity aggregation, while Openfort provides wallet signing infrastructure.

---

# 💰 Jupiter Referral

The project includes support for Jupiter referral fees.

The current configuration uses:

```text
50 bps = 0.5%
```

Referral fees provide an optional monetization mechanism for applications built on top of Jupiter.

Before deploying your own instance, configure your own referral account and review the current Jupiter referral requirements.

---

# 🦀 Tech Stack

| Technology | Purpose |
|---|---|
| **Rust** | Application backend |
| **Teloxide** | Telegram Bot framework |
| **Solana** | Blockchain |
| **Openfort** | Backend Wallet infrastructure and transaction signing |
| **Kora** | Gasless transaction infrastructure |
| **Jupiter** | Swap routing and liquidity |
| **SQLite** | Local persistence |
| **Reqwest** | HTTP API communication |
| **Serde** | Serialization and deserialization |

---

# 📁 Project Structure

```text
src/
├── balance/
│   └── ...
├── config/
│   └── ...
├── crypto/
│   └── ...
├── db/
│   └── ...
├── jupiter/
│   └── ...
├── openfort/
│   └── ...
├── solana/
│   └── ...
├── withdraw/
│   └── ...
└── main.rs
```

The project separates Telegram command handling, blockchain logic, wallet infrastructure and persistence so individual components can be extended or replaced independently.

---

# 🚀 Quick Start

## Requirements

Before running the bot, make sure you have:

- Rust
- Cargo
- A Telegram Bot Token
- An Openfort account
- Openfort API credentials
- A Solana RPC endpoint
- Jupiter configuration
- Kora configuration if you want to use gasless transactions

---

## 1. Clone the repository

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
```

---

## 2. Configure environment variables

Copy the example environment file:

```bash
cp .env.example .env
```

Open `.env` and configure your credentials.

Example:

```dotenv
TELEGRAM_BOT_TOKEN=...

OPENFORT_SECRET_KEY=...
OPENFORT_WALLET_SECRET=...

SOLANA_RPC_URL=...

JUPITER_API_KEY=...

DATABASE_URL=...
```

Use the `.env.example` file in the repository as the source of truth for the complete list of supported environment variables.

### Never commit secrets

Do not commit:

```text
.env
private keys
API keys
Openfort secrets
Telegram bot tokens
database files containing sensitive application data
```

Make sure `.env` is included in `.gitignore`.

---

## 3. Build the project

For a release build:

```bash
cargo build --release
```

---

## 4. Run the bot

```bash
cargo run --release
```

For development:

```bash
cargo run
```

---

# 🧪 Development

## Format the code

```bash
cargo fmt
```

Check formatting:

```bash
cargo fmt --check
```

---

## Run Clippy

```bash
cargo clippy
```

For stricter CI-style checking:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

---

## Run tests

```bash
cargo test
```

---

# 🌐 Solana Networks

The project is intended to support Solana development and production deployments.

For development and testing, use:

```text
Solana Devnet
```

For production:

```text
Solana Mainnet-beta
```

Make sure that your Openfort, Kora, Jupiter and RPC configurations are targeting the same intended network.

---

# 🔒 Security

Security is especially important because the bot can initiate blockchain transactions.

The intended architecture avoids storing users' Solana private keys inside the bot database.

The application can store information such as:

```text
Telegram user ID
Openfort account ID
Solana public address
Application state
```

It should not store:

```text
Solana private keys
Seed phrases
Recovery phrases
```

## Production recommendations

Before deploying the bot with real funds, consider implementing:

- withdrawal limits;
- per-user rate limiting;
- destination address validation;
- transaction idempotency;
- transaction audit logs;
- Openfort wallet policies;
- Kora spending and sponsorship limits;
- secure secret management;
- separate devnet and mainnet credentials;
- RPC failover;
- monitoring and alerting;
- database backups;
- administrator controls;
- emergency transaction controls.

---

# ⚠️ Production Warning

This repository is a **starter kit** and should not be considered a complete production-ready financial infrastructure platform.

Before handling significant real-world funds:

1. Test all transaction flows on Solana Devnet.
2. Review the Openfort Backend Wallet configuration.
3. Configure appropriate Openfort policies.
4. Configure Kora sponsorship and spending limits.
5. Add application-level withdrawal limits.
6. Validate destination addresses.
7. Implement proper error handling and transaction retry logic.
8. Secure all API credentials.
9. Add monitoring and alerting.
10. Perform an independent security review.

Do not assume that using an external wallet infrastructure provider eliminates the need for application-level security controls.

---

# 🗺️ Roadmap

## ✅The core functionality is already implemented:

- [x] Telegram Bot
- [x] Openfort Backend Wallet creation
- [x] Solana wallet management
- [x] SOL balance
- [x] SPL token balance
- [x] Jupiter swaps
- [x] SOL withdrawals
- [x] Kora transaction flow
- [x] SQLite persistence
- [x] Jupiter referral support

## 🚀 Short-term (next 3 months)

- [ ] **Transaction history** — view all user swaps and withdrawals
- [ ] **Inline keyboard UI** — better UX with buttons instead of text commands
- [ ] **More SPL tokens** — support for popular tokens (JUP, RAY, etc.)
- [ ] **Token metadata & logos** — display token icons and names
- [ ] **Security audit** — external security review of the codebase

## 🎯 Medium-term (3-6 months)

- [ ] **Limit orders** — set buy/sell orders at specific prices
- [ ] **DCA (Dollar-Cost Averaging)** — automate periodic purchases
- [ ] **Advanced withdrawal protection** — whitelist addresses, daily limits

## 🏆 Long-term (6+ months)

- [ ] **Token sniping** — automatic purchase of new tokens at launch
- [ ] **Copy trading** — follow and replicate trades of successful wallets
- [ ] **Production deployment guide** — comprehensive guide for deploying the bot
- [ ] **Integration tests** — full test suite for all commands and flows

---

# 🤝 Contributing

Contributions are welcome.

You can contribute by:

- reporting bugs;
- opening issues;
- improving documentation;
- submitting pull requests;
- improving security;
- adding tests;
- adding new Solana integrations;
- improving the Telegram UX.

Before submitting a pull request, run:

```bash
cargo fmt
cargo clippy
cargo test
```

---

# 📜 License

MIT License

Copyright © 2026

---

# ⭐ Why this project?

Building a Telegram-based Solana application usually requires combining several independent pieces of infrastructure:

```text
Telegram
   +
Wallet infrastructure
   +
Transaction signing
   +
Solana RPC
   +
Swap aggregation
   +
Gas sponsorship
   +
Database
```

This project brings those components together into one Rust starter kit.

The goal is simple:

> **Clone it, configure your infrastructure, run it, and start building your own Solana Telegram application.**

---

# 🔗 Ecosystem

- [Solana](https://solana.com/)
- [Openfort](https://www.openfort.io/)
- [Jupiter](https://jup.ag/)
- [Kora](https://github.com/solana-foundation/kora)
- [Teloxide](https://github.com/teloxide/teloxide)

---

# 🇷🇺 Русский

# Solana Starter Kit Bot 🤖

**Open-source starter kit для создания Telegram-ботов на Solana с использованием Rust.**

Проект объединяет:

- 🔐 Openfort Backend Wallet
- 💱 Jupiter для обмена токенов
- ⛽ Kora для gasless-транзакций
- 💸 вывод SOL
- 📊 баланс SOL и SPL-токенов
- 💰 Jupiter Referral
- 🗄️ SQLite
- 🦀 Rust + Teloxide

Цель проекта — предоставить готовую основу для разработчиков, которые хотят создавать Telegram-приложения с Solana-кошельками, обменом токенов и DeFi-функциональностью.

---

## ✨ Возможности

| Возможность | Описание |
|---|---|
| 🔐 **Openfort Backend Wallet** | Создание и управление Solana-кошельками через Openfort |
| 🛡️ **Non-custodial user experience** | Telegram-бот не получает и не хранит private keys пользователей |
| 💱 **Jupiter Swap** | Покупка и продажа поддерживаемых токенов |
| 💸 **Вывод SOL** | Отправка SOL на указанный Solana-адрес |
| ⛽ **Gasless Transactions** | Использование Kora для спонсируемых транзакций |
| 📊 **Баланс** | Просмотр SOL и SPL-токенов |
| 💰 **Jupiter Referral** | Возможность использовать referral fees |
| 🗄️ **SQLite** | Локальное хранение состояния приложения |
| 🦀 **Rust** | Производительный и безопасный backend |

---

# 🤖 Telegram-команды

## Кошелёк

### `/start`

Показывает приветственное сообщение и доступные команды.

### `/create_wallet`

Создаёт новый Solana Backend Wallet через Openfort.

---

## Баланс

### `/balance`

Показывает баланс SOL и поддерживаемых SPL-токенов.

---

## Обмен токенов

### `/buy <token> <amount>`

Покупка токена за SOL.

Пример:

```text
/buy USDC 1
```

### `/sell <token> <amount>`

Продажа токена за SOL.

Пример:

```text
/sell USDC 1
```

Поддерживаемые токены:

- USDC
- USDT
- wBTC
- wETH

Маршрутизация обмена выполняется через Jupiter.

---

## Вывод SOL

### `/withdraw <amount> <address>`

Вывод SOL на указанный Solana-адрес.

Пример:

```text
/withdraw 0.1 7xKXtg2CW87d97TXJSDpbD5jBkheT...
```

Транзакция подписывается через Openfort Backend Wallet.

При настроенной Kora sponsorship-инфраструктуре транзакция может выполняться в gasless-режиме.

---

# 🔐 Архитектура кошелька

Проект использует **Openfort Backend Wallet**.

Главный принцип:

> **Telegram-бот не получает и не хранит приватные ключи Solana пользователей.**

Упрощённая архитектура:

```text
                    Пользователь
                         │
                         ▼
                  Telegram Bot
                         │
                         ▼
                   Openfort API
                         │
                         ▼
               ┌──────────────────┐
               │ Openfort Backend │
               │      Wallet      │
               └────────┬─────────┘
                        │
                     signing
                        │
                        ▼
                     Solana
```

В приложении используются:

```text
Telegram user ID
Openfort account ID
Solana public address
Application state
```

В базе не должны храниться:

```text
Solana private key
Seed phrase
Recovery phrase
```

> **Важно:** термин "non-custodial" в данном README означает, что Telegram-бот не владеет и не хранит private keys пользователей. Backend Wallet является управляемой wallet-инфраструктурой, поэтому перед production deployment необходимо самостоятельно изучить модель custody, security и policies Openfort.

---

# ⛽ Kora

Kora используется для реализации gasless Solana transaction flows.

Упрощённый процесс:

```text
Telegram Bot
     │
     ▼
Openfort Backend Wallet
     │
     │ signature
     ▼
    Kora
     │
     │ fee payer
     ▼
 Solana RPC
     │
     ▼
  Solana
```

Kora может выступать в качестве fee payer и оплачивать комиссию Solana за пользователя.

Это позволяет создавать UX, в котором пользователю не обязательно заранее иметь SOL исключительно для оплаты network fees.

> Возможность gasless-транзакций зависит от настроек Kora, sponsorship policy и fee payer configuration.

---

# 💱 Jupiter

Jupiter используется для маршрутизации swap-транзакций и доступа к ликвидности Solana.

Примеры:

```text
SOL → USDC
USDC → SOL

SOL → USDT
USDT → SOL

SOL → wBTC
wBTC → SOL

SOL → wETH
wETH → SOL
```

Jupiter отвечает за routing и liquidity aggregation, а Openfort — за wallet signing.

---

# 💰 Jupiter Referral

В проекте предусмотрена интеграция Jupiter Referral.

Текущая конфигурация:

```text
50 bps
=
0.5%
```

Referral fees могут использоваться как механизм монетизации приложения.

Перед запуском собственной production-инсталляции необходимо настроить собственный referral account и проверить актуальные требования Jupiter.

---

# 🦀 Технологии

- **Rust** — backend
- **Teloxide** — Telegram Bot framework
- **Solana** — blockchain
- **Openfort** — Backend Wallet и transaction signing
- **Kora** — gasless transaction infrastructure
- **Jupiter** — swap routing и liquidity
- **SQLite** — database
- **Reqwest** — HTTP
- **Serde** — serialization

---

# 📁 Структура проекта

```text
src/
├── balance/
│   └── ...
├── config/
│   └── ...
├── crypto/
│   └── ...
├── db/
│   └── ...
├── jupiter/
│   └── ...
├── openfort/
│   └── ...
├── solana/
│   └── ...
├── withdraw/
│   └── ...
└── main.rs
```

---

# 🚀 Быстрый старт

## Требования

Для запуска понадобятся:

- Rust
- Cargo
- Telegram Bot Token
- Openfort account
- Openfort API credentials
- Solana RPC
- Jupiter configuration
- Kora configuration — если необходимы gasless-транзакции

---

## 1. Клонирование репозитория

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
```

---

## 2. Настройка переменных окружения

Скопируй пример конфигурации:

```bash
cp .env.example .env
```

Открой `.env` и укажи необходимые значения.

Пример:

```dotenv
TELEGRAM_BOT_TOKEN=...

OPENFORT_SECRET_KEY=...
OPENFORT_WALLET_SECRET=...

SOLANA_RPC_URL=...

JUPITER_API_KEY=...

DATABASE_URL=...
```

Полный список переменных необходимо смотреть в актуальном `.env.example` репозитория.

### Никогда не публикуй секреты

Не добавляй в Git:

```text
.env
private keys
API keys
Openfort secrets
Telegram bot tokens
```

Убедись, что `.env` находится в `.gitignore`.

---

## 3. Сборка

Для production build:

```bash
cargo build --release
```

---

## 4. Запуск

```bash
cargo run --release
```

Для разработки:

```bash
cargo run
```

---

# 🧪 Разработка

Форматирование:

```bash
cargo fmt
```

Проверка форматирования:

```bash
cargo fmt --check
```

Clippy:

```bash
cargo clippy
```

Строгая проверка:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Тесты:

```bash
cargo test
```

---

# 🌐 Сети Solana

Для разработки рекомендуется использовать:

```text
Solana Devnet
```

Для production:

```text
Solana Mainnet-beta
```

Openfort, Kora, Jupiter и Solana RPC должны быть настроены на соответствующую сеть.

---

# 🔒 Безопасность

Проект построен таким образом, чтобы Telegram-бот не хранил private keys пользователей.

Приложение может хранить:

```text
Telegram user ID
Openfort account ID
Solana public address
Application state
```

Не должно хранить:

```text
Solana private key
Seed phrase
Recovery phrase
```

## Рекомендации для production

Перед использованием реальных средств рекомендуется реализовать:

- лимиты на вывод;
- rate limiting;
- проверку destination address;
- transaction idempotency;
- audit logs;
- Openfort policies;
- Kora spending limits;
- безопасное хранение secrets;
- отдельные devnet/mainnet credentials;
- RPC failover;
- monitoring;
- alerting;
- database backups;
- административные ограничения;
- emergency controls.

---

# ⚠️ Перед использованием реальных средств

Этот репозиторий является **starter kit**, а не полностью готовой финансовой инфраструктурой.

Перед production deployment:

1. Протестируй все операции в Solana Devnet.
2. Проверь Openfort Backend Wallet configuration.
3. Настрой Openfort policies.
4. Настрой Kora sponsorship и spending limits.
5. Добавь application-level withdrawal limits.
6. Добавь проверку адресов.
7. Проверь обработку ошибок.
8. Защити все API credentials.
9. Добавь monitoring и alerting.
10. Проведи security review.

Использование внешней wallet-инфраструктуры не отменяет необходимости защищать само приложение.

---

# 🗺️ Дорожная карта

## ✅ Уже реализовано

- [x] Telegram бот
- [x] Создание Openfort Backend Wallet
- [x] Управление Solana-кошельком
- [x] Баланс SOL
- [x] Баланс SPL-токенов
- [x] Jupiter свопы
- [x] Вывод SOL
- [x] Kora gasless транзакции
- [x] SQLite persistence
- [x] Jupiter referral support

## 🚀 Ближайшие планы (3 месяца)

- [ ] **История транзакций** — просмотр всех свопов и выводов пользователя
- [ ] **Инлайн-клавиатура** — улучшенный UX с кнопками вместо текстовых команд
- [ ] **Поддержка большего количества SPL-токенов** — популярные токены (JUP, RAY и др.)
- [ ] **Метаданные и логотипы токенов** — отображение иконок и названий токенов
- [ ] **Аудит безопасности** — внешняя проверка безопасности кодовой базы

## 🎯 Среднесрочные планы (3–6 месяцев)

- [ ] **Лимитные ордера** — установка ордеров на покупку/продажу по заданной цене
- [ ] **DCA (усреднение цены)** — автоматическая периодическая покупка
- [ ] **Расширенная защита вывода** — белый список адресов, дневные лимиты

## 🏆 Долгосрочные планы (6+ месяцев)

- [ ] **Снайпинг токенов** — автоматическая покупка новых токенов при запуске
- [ ] **Копитрейдинг** — отслеживание и повторение сделок успешных кошельков
- [ ] **Руководство по production-развертыванию** — полное руководство по деплою бота
- [ ] **Интеграционные тесты** — полный набор тестов для всех команд и сценариев

---

# 🤝 Contributing

Contributions are welcome.

Можно помочь проекту через:

- Bug reports
- Issues
- Pull Requests
- Улучшение документации
- Security improvements
- Новые Solana integrations
- Tests
- Улучшение Telegram UX

Перед созданием Pull Request:

```bash
cargo fmt
cargo clippy
cargo test
```

---

# 📜 License

MIT License

Copyright © 2026

---

# ⭐ Почему этот проект?

Для создания полноценного Solana DeFi-приложения внутри Telegram обычно необходимо объединить несколько компонентов:

```text
Telegram
   +
Wallet infrastructure
   +
Transaction signing
   +
Solana RPC
   +
Swap aggregation
   +
Gas sponsorship
   +
Database
```

Этот проект объединяет их в один Rust starter kit.

Цель проекта:

> **Склонировать → настроить инфраструктуру → запустить → начать создавать своё Solana-приложение в Telegram.**

---

# 🔗 Ecosystem

- [Solana](https://solana.com/)
- [Openfort](https://www.openfort.io/)
- [Jupiter](https://jup.ag/)
- [Kora](https://github.com/solana-foundation/kora)
- [Teloxide](https://github.com/teloxide/teloxide)
