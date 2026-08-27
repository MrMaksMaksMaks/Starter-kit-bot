# Solana Starter Kit Bot 🤖

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.76+-blue.svg)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/Solana-1.18-purple.svg)](https://solana.com/)
[![Telegram](https://img.shields.io/badge/Telegram-Bot-blue.svg)](https://core.telegram.org/bots)

---

## 🇬🇧 English

### 📖 Description

**Starter-kit-bot** is an open-source, non-custodial Telegram bot for Solana. It provides a ready-to-use foundation for developers to build their own Telegram-based trading bots with minimal setup.

This project is designed as a **Public Good** for the Solana ecosystem, lowering the entry barrier for developers and accelerating the creation of DeFi applications on Telegram.

### ✨ Features

| Feature | Description |
|---------|-------------|
| 🔐 **Non-custodial wallets** | Full user control over funds via Openfort |
| 💱 **Jupiter Swap integration** | Buy and sell tokens with best prices |
| ⛽ **Gasless transactions** | Withdraw SOL without network fees via Kora |
| 📊 **Balance checking** | View SOL and SPL token balances |
| 💰 **Referral program** | Monetize with Jupiter Referral (50 bps) |
| 🛡️ **Secure** | Private keys never stored in the bot |

### 📋 Commands

| Command | Description |
|---------|-------------|
| `/start` | Welcome message and help |
| `/create_wallet` | Create a new non-custodial wallet |
| `/balance` | Check SOL balance |
| `/buy <token> <amount>` | Buy tokens (USDC, USDT, wBTC, wETH) |
| `/sell <token> <amount>` | Sell tokens for SOL |
| `/withdraw <amount> <address>` | Withdraw SOL to any address |

### 🛠️ Tech Stack

- **Rust** + **Teloxide** — Telegram Bot Framework
- **Solana SDK** — Blockchain interaction
- **Jupiter API** — Liquidity aggregator
- **Openfort** — Wallet management & transaction signing
- **Kora** — Gasless transactions
- **SQLite** — Local database

### 🏗️ Architecture
```bash
┌─────────────────────────────────────────────────────────────────┐
│ Telegram User │
└─────────────────────────────┬───────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────────────────────┐
│ Telegram Bot API │
└─────────────────────────────┬───────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────────────────────┐
│ main.rs (Command Handler) │
│ ┌──────────────────────────────────────────────────────────┐ │
│ │ /create_wallet /balance /buy /sell /withdraw │ │
│ └──────────────────────────────────────────────────────────┘ │
└───────────┬─────────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────────────────────┐
│ Modules │
├─────────┬──────────┬─────────┬──────────┬──────────┬──────────┤
│balance/ │ config/ │ db/ │ jupiter/ │ openfort/│ solana/ │
├─────────┴──────────┴─────────┴──────────┴──────────┴──────────┤
│ withdraw/ │
└───────────┬─────────────────────────────────────────────────────┘
│
▼
┌─────────────────────────────────────────────────────────────────┐
│ External APIs │
├─────────────┬─────────────┬─────────────┬─────────────────────┤
│ Jupiter API │ Openfort API│ Solana RPC │ Kora API │
├─────────────┴─────────────┴─────────────┴─────────────────────┤
│ SQLite Database │
└─────────────────────────────────────────────────────────────────┘
```
### 🚀 Quick Start

#### 1. Clone the repository

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
```

#### 2. Set up environment variables

```bash
cp .env.example .env
# Fill in your keys (Telegram Bot Token, Openfort keys, etc.)
```

#### 3. Build and run

```bash
cargo build --release
cargo run
```

💰 Monetization
The bot includes a Jupiter Referral Program with 50 bps (0.5%) commission on swaps. This provides a sustainable revenue model for developers while maintaining full transparency and non-custodial nature.

🔒 Security
Private keys never stored in the database or bot

All transaction signing happens via Openfort's secure infrastructure

.env file protected by .gitignore

User funds always remain in their control

📄 License
MIT © 2026

🤝 Contributing
Contributions are welcome! Feel free to submit issues, fork the repository, and send pull requests.

🇷🇺 Русский
📖 Описание
Starter-kit-bot — это open-source, некастодиальный Telegram-бот для Solana. Он предоставляет готовую основу для разработчиков, позволяющую быстро создавать торговых ботов на Solana с минимальной настройкой.

Проект создан как Public Good для экосистемы Solana, снижая порог входа для разработчиков и ускоряя создание DeFi-приложений в Telegram.

✨ Возможности
Возможность	Описание
🔐 Некастодиальные кошельки	Полный контроль пользователя над средствами через Openfort
💱 Интеграция Jupiter Swap	Покупка и продажа токенов по лучшим ценам
⛽ Gasless транзакции	Вывод SOL без комиссии сети через Kora
📊 Проверка баланса	Просмотр баланса SOL и SPL-токенов
💰 Реферальная программа	Монетизация через Jupiter Referral (50 bps)
🛡️ Безопасность	Приватные ключи не хранятся в боте
📋 Команды
Команда	Описание
/start	Приветствие и помощь
/create_wallet	Создание нового некастодиального кошелька
/balance	Проверка баланса SOL
/buy <token> <amount>	Покупка токенов (USDC, USDT, wBTC, wETH)
/sell <token> <amount>	Продажа токенов за SOL
/withdraw <amount> <address>	Вывод SOL на любой адрес
🛠️ Технологии
Rust + Teloxide — Telegram Bot Framework

Solana SDK — работа с блокчейном

Jupiter API — агрегатор ликвидности

Openfort — управление кошельками и подпись транзакций

Kora — gasless транзакции

SQLite — локальная база данных

🏗️ Архитектура
```bash
┌─────────────────────────────────────────────────────────────────┐
│                        Пользователь Telegram                   │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Telegram Bot API                           │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      main.rs (Обработчик команд)               │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ /create_wallet  /balance  /buy  /sell  /withdraw        │   │
│  └──────────────────────────────────────────────────────────┘   │
└───────────┬─────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────────┐
│                          Модули                                 │
├─────────┬──────────┬─────────┬──────────┬──────────┬──────────┤
│balance/ │ config/  │   db/   │ jupiter/ │ openfort/│ solana/  │
├─────────┴──────────┴─────────┴──────────┴──────────┴──────────┤
│                         withdraw/                              │
└───────────┬─────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Внешние API                                │
├─────────────┬─────────────┬─────────────┬─────────────────────┤
│ Jupiter API │ Openfort API│ Solana RPC  │     Kora API        │
├─────────────┴─────────────┴─────────────┴─────────────────────┤
│                         SQLite Database                        │
└─────────────────────────────────────────────────────────────────┘
```

#### 🚀 Быстрый старт
#### 1. Клонирование репозитория

```bash
git clone https://github.com/MrMaksMaksMaks/Starter-kit-bot.git
cd Starter-kit-bot
```

#### 2. Настройка переменных окружения

```bash
cp .env.example .env
# Fill in your keys (Telegram Bot Token, Openfort keys, etc.)
```

#### 3. Сборка и запуск

```bash
cargo build --release
cargo run
```

💰 Монетизация
Бот включает реферальную программу Jupiter с комиссией 50 bps (0.5%) от свопов. Это обеспечивает устойчивую модель дохода для разработчиков, сохраняя полную прозрачность и некастодиальность.

🔒 Безопасность
Приватные ключи никогда не хранятся в базе данных или боте

Подпись транзакций происходит через защищенную инфраструктуру Openfort

Файл .env защищен .gitignore

Средства пользователя всегда остаются под их контролем

📄 Лицензия
MIT © 2026

🤝 Вклад в проект
Приветствуются любые вклады! Создавайте форки, отправляйте pull request и предлагайте улучшения.

⭐ Поставьте звезду, если проект вам полезен!
⭐ Star this repository if you find it useful!
