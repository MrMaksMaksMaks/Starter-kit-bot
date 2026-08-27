# Solana Starter Kit Bot 🤖

**Open-source starter kit for building Telegram-native Solana applications.**

Solana Starter Kit Bot is an open-source Telegram bot built with Rust that combines Solana wallet infrastructure, token swaps, withdrawals, and gasless transaction flows into a single working application.

The project is intended to be more than a finished bot: it is a **reusable developer foundation** for anyone who wants to build a Telegram-based Solana application without implementing wallet, transaction, and swap infrastructure from scratch.

---

## 🎯 Why this project?

Building a Telegram application that interacts with Solana requires several pieces of infrastructure to work together:

- wallet creation and management;
- secure transaction signing;
- Solana RPC communication;
- token swaps and liquidity routing;
- transaction sponsorship;
- persistent user and wallet state;
- Telegram bot infrastructure.

For an individual developer or a small team, integrating all of these components can become a significant barrier before the actual application idea can even be tested.

**Solana Starter Kit Bot is designed to reduce that barrier.**

The repository provides a working reference implementation that brings together:

- **Openfort Backend Wallets** for wallet infrastructure and transaction signing;
- **Jupiter** for token swaps and liquidity aggregation;
- **Kora** for gasless transaction flows;
- **Solana RPC** for blockchain interaction;
- **Rust + Teloxide** for the Telegram application layer;
- **SQLite** for persistent application data.

The goal is to make the repository useful as a **public, reusable starting point** rather than as a closed application.

Developers can fork the project, inspect the implementation, replace individual components, add their own business logic, and use the existing integration patterns as a foundation for new Solana applications.

### The problem in one sentence

> **Developers should be able to experiment with Telegram-native Solana applications without first building the entire wallet and transaction infrastructure themselves.**

---

## ☀️ Why Solana?

The core idea of this project is to make on-chain interactions feel as simple as using a Telegram bot.

That experience requires a blockchain environment where transactions can be:

- fast enough for interactive applications;
- inexpensive enough for frequent user actions;
- supported by a mature token and DeFi ecosystem;
- compatible with modern transaction sponsorship infrastructure.

Solana is a particularly strong fit for this use case.

### ⚡ Fast and inexpensive interactions

A Telegram application can generate many small and frequent blockchain interactions.

Users may check balances, swap tokens, send funds, or perform other on-chain actions directly from a conversational interface. Keeping those interactions practical requires low transaction costs and fast confirmation.

Solana's performance and transaction economics make this type of user experience practical.

### 💱 A mature DeFi ecosystem

The project uses **Jupiter** as its swap infrastructure.

This means the starter kit can provide token swaps while connecting the application to Solana's broader liquidity ecosystem instead of requiring developers to implement their own routing and liquidity logic.

### ⛽ Gasless transaction infrastructure

A major usability problem for blockchain applications is requiring users to maintain a separate balance of the native token just to pay transaction fees.

This project integrates **Kora** to support gasless transaction flows.

Combined with Openfort Backend Wallets, this allows developers to build Telegram experiences where transaction-fee management can be abstracted away from the user for supported flows.

### 🧩 A strong developer ecosystem

Solana already provides the building blocks needed for this type of application:

- Solana RPC infrastructure;
- a mature token ecosystem;
- Jupiter liquidity infrastructure;
- Kora transaction sponsorship;
- Openfort wallet infrastructure;
- Rust tooling and libraries;
- a large ecosystem of DeFi applications and protocols.

This starter kit connects those components into one understandable reference implementation.

### Why build specifically for Solana?

This project is intentionally **Solana-first**, not chain-agnostic.

The objective is not to create a generic multi-chain wallet abstraction. The objective is to make it easier to build useful, Telegram-native applications specifically on Solana.

The combination of low-cost transactions, high performance, mature DeFi infrastructure, and gasless transaction primitives creates a strong foundation for this type of application.

---

## 🌐 Solana ecosystem contribution

This project is designed as an **open-source public good for Solana developers**.

The Solana Foundation describes public goods as projects that make significant open-source contributions to the Solana ecosystem or provide meaningful free community offerings. It also emphasizes open-source learnings and asks applicants to clearly explain why their project is building within Solana.

This repository is structured around those principles:

### 📚 Learn from a working implementation

Instead of documenting integrations only at a conceptual level, the repository contains a working application that demonstrates how the components interact.

Developers can inspect the source code and use the implementation as a reference for their own projects.

### 🧱 Reuse existing integrations

The wallet, swap, transaction, and bot modules are separated so that developers can modify or replace individual parts of the stack.

### 🔓 Open source by default

The usable application code is publicly available so that other developers can fork it, learn from it, and build on top of it.

### 🚀 Lower the entry barrier

The project aims to reduce the amount of infrastructure work required before a developer can start experimenting with a Solana application inside Telegram.

---

## ✨ Current functionality

The current implementation includes:

| Feature | Description |
|---|---|
| 🔐 Backend Wallets | Solana wallets implemented through Openfort Backend Wallet infrastructure |
| 🔑 Remote transaction signing | The Telegram bot does not store users' private keys |
| 💱 Jupiter swaps | Buy and sell supported Solana tokens through Jupiter |
| ⛽ Gasless transactions | Gasless transaction flow using Kora |
| 💸 SOL withdrawals | Withdraw SOL to an external Solana address |
| 📊 Balance checking | View SOL and SPL token balances |
| 💰 Jupiter referral integration | Optional swap monetization through Jupiter Referral |
| 🛡️ Secret isolation | Sensitive wallet credentials are kept outside the application database |

---

## 🔐 Wallet architecture

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

## 🏗️ Architecture

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
│  /start   /create_wallet   /balance   /buy   /sell   /withdraw │
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

## 🔄 Main transaction flows

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
Jupiter
      │
      ▼
Swap transaction
      │
      ▼
Openfort transaction signing
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
Openfort Backend Wallet
      │
      ▼
Kora gasless transaction flow
      │
      ▼
Solana
      │
      ▼
Transaction signature
```

---

## 📋 Telegram commands

| Command | Description |
|---|---|
| `/start` | Welcome message and available commands |
| `/create_wallet` | Create a new Solana wallet |
| `/balance` | Check SOL and token balances |
| `/buy <token> <amount>` | Buy a supported token |
| `/sell <token> <amount>` | Sell a supported token for SOL |
| `/withdraw <amount> <address>` | Withdraw SOL to an external Solana address |

### Buy

```text
/buy <token> <amount>
```

Example:

```text
/buy USDC 1
```

### Sell

```text
/sell <token> <amount>
```

Example:

```text
/sell USDC 1
```

Currently supported tokens include:

- USDC
- USDT
- wBTC
- wETH

### Withdraw

```text
/withdraw <amount> <address>
```

Example:

```text
/withdraw 0.1 7xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

---

## 🛠️ Tech stack

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

## 🚀 Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
```

### 2. Create your environment file

```bash
cp .env.example .env
```

Fill in the required environment variables in `.env`.

Depending on the enabled functionality, this includes credentials for:

- Telegram Bot API;
- Openfort;
- Jupiter;
- Kora;
- Solana RPC.

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

## 🔧 Configuration

Configuration is provided through environment variables.

See:

```text
.env.example
```

for the variables required by the current implementation.

Keep all secrets outside the repository and make sure `.env` is excluded by `.gitignore`.

---

## 💰 Jupiter Referral / Monetization

The bot includes integration with the **Jupiter Referral** mechanism.

The current configuration uses a **50 bps (0.5%)** swap commission.

The purpose of this integration is to demonstrate how developers building on top of this starter kit can add an optional and transparent revenue mechanism while keeping the core project open source.

For developers using the repository as a foundation, the referral configuration can be replaced or removed according to their own application model.

---

## 🔒 Security

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

### Production considerations

This repository is a starter kit and should not be considered a complete production security framework.

Before deploying a production application, developers should consider:

- Telegram user authentication and authorization;
- withdrawal limits;
- address validation;
- rate limiting;
- replay protection;
- transaction simulation;
- audit logging;
- error handling;
- monitoring and alerting;
- Openfort access policies;
- RPC reliability;
- database backups;
- secret rotation.

---

## 🧪 Development

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

## 🗺️ Roadmap

### ✅ Already implemented

- [x] Telegram Bot
- [x] Openfort Backend Wallet creation
- [x] SOL and SPL token balances
- [x] Jupiter swaps
- [x] SOL withdrawals
- [x] Kora gasless transactions
- [x] SQLite persistence
- [x] Jupiter referral support

### 🚀 Short-term (3 months)

- [ ] Transaction history
- [ ] Inline keyboard UI
- [ ] More SPL tokens
- [ ] Token metadata & logos
- [ ] Security audit

### 🎯 Medium-term (3-6 months)

- [ ] Limit orders
- [ ] DCA (Dollar-Cost Averaging)
- [ ] Advanced withdrawal protection

### 🏆 Long-term (6+ months)

- [ ] Token sniping
- [ ] Copy trading
- [ ] Production deployment guide
- [ ] Integration tests

The roadmap is focused on turning the current working reference implementation into a more complete and production-oriented foundation for Telegram-native Solana applications.

---

## 🌱 Open-source public good

The project is released as open source so that the implementation itself can become useful infrastructure for other builders.

The intended value to the Solana ecosystem is not limited to the bot described in this repository.

A developer should be able to take this project and turn it into:

- a trading assistant;
- a community bot;
- a DeFi interface;
- a payment bot;
- a portfolio assistant;
- a game economy;
- or another Telegram-native Solana application.

The common infrastructure — wallet integration, transaction signing, swaps, withdrawals, and Solana interaction — should not need to be rebuilt from zero every time.

---

## 🤝 Contributing

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

## 📄 License

MIT License.

Copyright © 2026.

---

## ⭐ Support the project

If this starter kit is useful to you:

- ⭐ Star the repository;
- fork it;
- build something on Solana;
- open an issue with feedback;
- contribute improvements;
- share what you build with the community.

**The goal is simple: make it easier for more developers to build useful applications on Solana.**

---

# 🇷🇺 Русская версия

## 🎯 Зачем нужен этот проект?

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

### Проблема в одном предложении

> **Разработчик должен иметь возможность быстро экспериментировать с Telegram-native приложениями на Solana, не создавая с нуля всю инфраструктуру кошельков и транзакций.**

---

## ☀️ Почему Solana?

Главная идея проекта — сделать взаимодействие с блокчейном таким же простым для пользователя, как взаимодействие с обычным Telegram-ботом.

Для этого нужны:

- быстрые транзакции;
- низкая стоимость операций;
- развитая экосистема токенов и DeFi;
- современная инфраструктура для спонсирования транзакций.

Solana особенно хорошо подходит под такой сценарий.

### ⚡ Быстрые и недорогие операции

Telegram-приложение может выполнять много небольших и частых blockchain-операций.

Пользователь может проверять баланс, обменивать токены, выводить средства и выполнять другие действия прямо из интерфейса Telegram.

Поэтому высокая производительность и низкая стоимость транзакций особенно важны для такого UX.

### 💱 Экосистема DeFi

Проект использует **Jupiter** для обмена токенов.

Это позволяет использовать существующую инфраструктуру ликвидности Solana вместо того, чтобы каждому разработчику самостоятельно реализовывать маршрутизацию и поиск ликвидности.

### ⛽ Gasless-транзакции

Одна из проблем blockchain UX — необходимость держать нативный токен только для оплаты комиссии.

Проект интегрирует **Kora** для поддержки gasless-транзакций.

Вместе с Openfort Backend Wallet это позволяет создавать сценарии, в которых управление комиссией может быть скрыто от пользователя.

### 🧩 Экосистема разработчика

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

## 🌐 Вклад в экосистему Solana

Проект задуман как **open-source public good для разработчиков Solana**.

Основная идея — сделать рабочую интеграцию доступной другим разработчикам, чтобы они могли:

- изучать реализацию;
- использовать её как reference implementation;
- делать fork;
- заменять отдельные компоненты;
- добавлять собственную бизнес-логику;
- создавать новые Telegram-native приложения.

Ценность проекта заключается не только в самом боте.

Одна и та же инфраструктура может стать основой для:

- trading assistants;
- community bots;
- DeFi-интерфейсов;
- payment bots;
- portfolio assistants;
- игровых экономик;
- других Telegram-приложений на Solana.

---

## ✨ Текущий функционал

На данный момент реализованы:

| Возможность | Описание |
|---|---|
| 🔐 Backend Wallets | Solana-кошельки через Openfort Backend Wallet |
| 🔑 Удалённая подпись | Бот не хранит приватные ключи пользователей |
| 💱 Jupiter Swap | Покупка и продажа поддерживаемых токенов |
| ⛽ Gasless transactions | Gasless flow через Kora |
| 💸 Вывод SOL | Вывод SOL на внешний Solana-адрес |
| 📊 Балансы | Просмотр SOL и SPL-токенов |
| 💰 Jupiter Referral | Реферальная монетизация обменов |
| 🛡️ Разделение секретов | Чувствительные credentials не хранятся в базе |

---

## 🔐 Архитектура кошелька

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

## 📋 Команды

| Команда | Описание |
|---|---|
| `/start` | Приветствие и доступные команды |
| `/create_wallet` | Создание нового Solana-кошелька |
| `/balance` | Проверка SOL и токенов |
| `/buy <token> <amount>` | Покупка поддерживаемого токена |
| `/sell <token> <amount>` | Продажа поддерживаемого токена за SOL |
| `/withdraw <amount> <address>` | Вывод SOL на внешний Solana-адрес |

Пример:

```text
/buy USDC 1
```

```text
/sell USDC 1
```

```text
/withdraw 0.1 <SOLANA_ADDRESS>
```

---

## 🚀 Быстрый старт

### 1. Клонирование репозитория

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
```

### 2. Настройка переменных окружения

```bash
cp .env.example .env
```

Заполните необходимые переменные в `.env`.

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

## 🔒 Безопасность

Telegram-бот не хранит приватные ключи пользователей в SQLite или исходном коде.

Подпись транзакций выполняется через инфраструктуру Openfort Backend Wallet.

Секретные данные передаются через environment variables.

Перед production-развёртыванием необходимо самостоятельно обеспечить:

- контроль доступа;
- лимиты на вывод;
- валидацию адресов;
- rate limiting;
- мониторинг;
- аудит операций;
- безопасное хранение секретов;
- резервное копирование;
- политики доступа Openfort.

---

## 🗺️ Roadmap

### ✅ Уже реализовано

- [x] Telegram Bot
- [x] Создание Openfort Backend Wallet
- [x] Балансы SOL и SPL-токенов
- [x] Jupiter swaps
- [x] Вывод SOL
- [x] Kora gasless transactions
- [x] SQLite persistence
- [x] Jupiter referral support

### 🚀 Краткосрочные цели (3 месяца)

- [ ] История транзакций
- [ ] Inline keyboard UI
- [ ] Поддержка большего количества SPL-токенов
- [ ] Метаданные токенов и логотипы
- [ ] Security audit

### 🎯 Среднесрочные цели (3–6 месяцев)

- [ ] Limit orders
- [ ] DCA (Dollar-Cost Averaging)
- [ ] Расширенная защита вывода средств

### 🏆 Долгосрочные цели (6+ месяцев)

- [ ] Token sniping
- [ ] Copy trading
- [ ] Production deployment guide
- [ ] Integration tests

Цель roadmap — превратить уже работающую reference implementation в более полноценную и production-oriented основу для Telegram-native приложений на Solana.

---

## 🤝 Вклад в проект

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

## 📄 Лицензия

MIT License.

Copyright © 2026.

---

## ⭐ Поддержать проект

Если проект оказался полезен:

- ⭐ поставьте Star;
- сделайте fork;
- создайте собственное приложение на Solana;
- предложите улучшения;
- отправьте Pull Request;
- расскажите сообществу о том, что построили.

**Цель проекта — сделать создание полезных Telegram-native приложений на Solana проще для большего числа разработчиков.**
