use anyhow::Result;
use dotenv::dotenv;
use starter_kit_bot::balance;
use starter_kit_bot::config::Config;
use starter_kit_bot::db::{self, models::NewUser, repository::UserRepository};
use starter_kit_bot::jupiter;
use starter_kit_bot::openfort::OpenfortClient;
use starter_kit_bot::solana;
use starter_kit_bot::withdraw;
use teloxide::prelude::*;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

/// Escapes special MarkdownV2 characters in dynamic data before inserting into a message.
fn escape_markdown_v2(text: &str) -> String {
    const SPECIAL: &[char] = &[
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=',
        '|', '{', '}', '.', '!',
    ];
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        if SPECIAL.contains(&c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

/// Formats a raw amount (in smallest units, as a string) into a human-readable format
/// with decimals — for example, "1500000" with decimals=6 becomes "1.5"
fn format_token_amount(raw_amount: &str, decimals: u8) -> String {
    match raw_amount.parse::<u64>() {
        Ok(raw) => {
            let divisor = 10u64.pow(decimals as u32);
            let whole = raw / divisor;
            let frac = raw % divisor;
            if frac == 0 {
                whole.to_string()
            } else {
                let frac_str = format!("{:0width$}", frac, width = decimals as usize);
                let trimmed = frac_str.trim_end_matches('0');
                if trimmed.is_empty() {
                    whole.to_string()
                } else {
                    format!("{}.{}", whole, trimmed)
                }
            }
        }
        Err(_) => raw_amount.to_string(),
    }
}

/// Resolves token decimals: first checks known ones (SOL/USDC/USDT/wBTC/wETH),
/// otherwise queries RPC via getTokenSupply
async fn resolve_decimals(rpc_url: &str, mint: &str) -> Result<u8> {
    if let Some(d) = jupiter::known_decimals(mint) {
        return Ok(d);
    }
    solana::get_token_decimals(rpc_url, mint).await
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    dotenv().ok();

    info!("🚀 Starting Starter-Kit-Bot...");

    let config = Config::from_env()?;
    info!("✅ Configuration loaded");
    info!("📊 Database URL: {}", config.database_url);
    info!("🔗 Solana RPC: {}", config.solana_rpc_url);
    info!("🔄 Jupiter API Key set: {}", !config.jupiter_api_key.is_empty());

    info!("📊 Initializing database...");
    let pool = db::init_pool(&config.database_url).await?;
    let repo = UserRepository::new(pool);
    info!("✅ Database initialized");

    let openfort = OpenfortClient::new(
        config.openfort_base_url.clone(),
        config.openfort_secret_key.clone(),
        config.openfort_wallet_secret.clone(),
        config.openfort_publishable_key.clone(),
    );
    info!("✅ Openfort client initialized");

    let bot = Bot::new(config.telegram_token.clone());

    info!("🤖 Bot is running! Press Ctrl+C to stop.");

    let handler = move |bot: Bot, msg: Message| {
        let repo = repo.clone();
        let openfort = openfort.clone();
        let config = config.clone();
        async move {
            let text = msg.text().unwrap_or("");
            let chat_id = msg.chat.id;
            let telegram_id = msg.chat.id.0 as i64;

            println!("📝 Received: {}", text);

            // ============================
            // /start
            // ============================
            if text == "/start" {
                bot.send_message(
                    chat_id,
                    "🚀 Welcome to Solana-kit-bot!\n\n\
                    Commands:\n\
                    /create_wallet - Create a wallet\n\
                    /balance - Check SOL balance\n\
                    /tokens - Show all SPL token balances\n\
                    /buy <token> <SOL> - Spend SOL to buy a token\n\
                    /sell <token> <amount> - Sell a token for SOL\n\
                    /withdraw <amount> <address> - Withdraw SOL\n\n\
                    Example: /buy USDC 0.1 — spends 0.1 SOL to buy USDC\n\
                    Example: /sell USDC 5 — sells 5 USDC for SOL"
                ).await?;
                return Ok(());
            }

            // ============================
            // /create_wallet
            // ============================
            if text == "/create_wallet" {
                println!("🔍 Checking if user {} has a wallet...", telegram_id);

                match repo.find_by_telegram_id(telegram_id).await {
                    Ok(Some(user)) => {
                        println!("✅ User already has a wallet");
                        bot.send_message(
                            chat_id,
                            format!(
                                "ℹ️ You already have a wallet!\n\n📍 Address: {}\n🆔 Account ID: {}\n\nUse /balance to check your funds.",
                                user.solana_address,
                                user.openfort_account_id
                            )
                        ).await?;
                        return Ok(());
                    }
                    Ok(None) => {
                        println!("🔐 Creating wallet for user: {}", telegram_id);
                        bot.send_message(chat_id, "🔐 Creating wallet...").await?;
                        let user_id = telegram_id.to_string();
                        match openfort.create_wallet(&user_id).await {
                            Ok(account) => {
                                println!("✅ Wallet created: {}", account.address);
                                let new_user = NewUser {
                                    telegram_id,
                                    openfort_account_id: account.id.clone(),
                                    solana_address: account.address.clone(),
                                    wallet_id: account.wallet_id.clone(),
                                };
                                match repo.create(new_user).await {
                                    Ok(saved) => {
                                        bot.send_message(
                                            chat_id,
                                            format!(
                                                "✅ Wallet created!\n\n📍 Address: {}",
                                                saved.solana_address
                                            )
                                        ).await?;
                                    }
                                    Err(e) => {
                                        bot.send_message(chat_id, format!("❌ DB error: {}", e)).await?;
                                    }
                                }
                            }
                            Err(e) => {
                                bot.send_message(chat_id, format!("❌ Error: {}", e)).await?;
                            }
                        }
                    }
                    Err(e) => {
                        println!("❌ DB error: {}", e);
                        bot.send_message(chat_id, format!("❌ DB error: {}", e)).await?;
                    }
                }
                return Ok(());
            }

            // ============================
            // /balance
            // ============================
            if text == "/balance" {
                match repo.find_by_telegram_id(telegram_id).await {
                    Ok(Some(user)) => {
                        println!("💰 Getting balance for: {}", user.solana_address);

                        bot.send_message(chat_id, "🔄 Checking balance...").await?;

                        match solana::get_balance(&config.solana_rpc_url, &user.solana_address).await {
                            Ok(balance) => {
                                bot.send_message(
                                    chat_id,
                                    format!(
                                        "💰 Balance\n\n📍 Address: {}\n💎 SOL: {:.6}\n\nNetwork: {}",
                                        user.solana_address,
                                        balance,
                                        config.solana_network
                                    )
                                ).await?;
                            }
                            Err(e) => {
                                println!("❌ RPC error: {}", e);
                                bot.send_message(
                                    chat_id,
                                    format!("❌ Failed to get balance\n\nError: {}", e)
                                ).await?;
                            }
                        }
                    }
                    Ok(None) => {
                        bot.send_message(chat_id, "⚠️ No wallet. Use /create_wallet").await?;
                    }
                    Err(e) => {
                        bot.send_message(chat_id, format!("❌ DB error: {}", e)).await?;
                    }
                }
                return Ok(());
            }

            // ============================
            // /tokens — SOL + all SPL tokens
            // ============================
            if text == "/tokens" {
                match repo.find_by_telegram_id(telegram_id).await {
                    Ok(Some(user)) => {
                        println!("💎 Getting all token balances for: {}", user.solana_address);

                        bot.send_message(chat_id, "🔄 Fetching balances...").await?;

                        match balance::get_formatted_balances(&config.solana_rpc_url, &user.solana_address).await {
                            Ok(formatted) => {
                                bot.send_message(chat_id, formatted)
                                    .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                                    .await?;
                            }
                            Err(e) => {
                                println!("❌ Failed to get balances: {}", e);
                                bot.send_message(chat_id, format!("❌ Error: {}", e)).await?;
                            }
                        }
                    }
                    Ok(None) => {
                        bot.send_message(chat_id, "⚠️ Please create a wallet first: /create_wallet").await?;
                    }
                    Err(e) => {
                        bot.send_message(chat_id, format!("❌ DB error: {}", e)).await?;
                    }
                }
                return Ok(());
            }

            // ============================
            // /buy — spend SOL to buy a token
            // ============================
            if text.starts_with("/buy") {
                println!("🟢 Processing buy for user: {}", telegram_id);

                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.len() < 3 {
                    bot.send_message(
                        chat_id,
                        "❌ Please specify token and amount in SOL to spend.\n\nExample: /buy USDC 0.1\n(spends 0.1 SOL to buy USDC)"
                    ).await?;
                    return Ok(());
                }

                let token_input = parts[1];
                let sol_amount: f64 = match parts[2].parse() {
                    Ok(a) if a > 0.0 => a,
                    _ => {
                        bot.send_message(chat_id, "❌ Invalid amount. Please enter a positive number.").await?;
                        return Ok(());
                    }
                };

                match repo.find_by_telegram_id(telegram_id).await {
                    Ok(Some(user)) => {
                        let output_mint = jupiter::resolve_token_mint(token_input);
                        let amount_lamports = (sol_amount * 1_000_000_000.0).round() as u64;

                        bot.send_message(
                            chat_id,
                            format!(
                                "🔄 Spending {} SOL, searching for the best route to buy {}\\.\\.\\.",
                                escape_markdown_v2(&sol_amount.to_string()),
                                escape_markdown_v2(token_input)
                            )
                        ).parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;

                        match jupiter::perform_swap(
                            &openfort,
                            &config.jupiter_api_key,
                            &user.openfort_account_id,
                            &user.solana_address,
                            jupiter::tokens::SOL,
                            &output_mint,
                            amount_lamports,
                            100,
                            config.referral_account.as_deref(),
                            Some(config.referral_fee_bps),
                        ).await {
                            Ok(result) => {
                                let out_decimals = resolve_decimals(&config.solana_rpc_url, &output_mint)
                                    .await
                                    .unwrap_or(6);
                                let received = format_token_amount(&result.output_amount_result, out_decimals);

                                bot.send_message(
                                    chat_id,
                                    format!(
                                        "✅ Purchased\\!\n\nSpent: {} SOL\nReceived: {} {}\nTXID: `{}`",
                                        escape_markdown_v2(&sol_amount.to_string()),
                                        escape_markdown_v2(&received),
                                        escape_markdown_v2(token_input),
                                        result.signature
                                    )
                                ).parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;
                            }
                            Err(e) => {
                                println!("❌ Buy failed: {}", e);
                                bot.send_message(chat_id, format!("❌ Purchase failed: {}", e)).await?;
                            }
                        }
                    }
                    Ok(None) => {
                        bot.send_message(chat_id, "⚠️ Please create a wallet first: /create_wallet").await?;
                    }
                    Err(e) => {
                        bot.send_message(chat_id, format!("❌ DB error: {}", e)).await?;
                    }
                }
                return Ok(());
            }

            // ============================
            // /sell — sell a token for SOL
            // ============================
            if text.starts_with("/sell") {
                println!("🔴 Processing sell for user: {}", telegram_id);

                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.len() < 3 {
                    bot.send_message(
                        chat_id,
                        "❌ Please specify token and amount to sell.\n\nExample: /sell USDC 5\n(sells 5 USDC for SOL)"
                    ).await?;
                    return Ok(());
                }

                let token_input = parts[1];
                let token_amount: f64 = match parts[2].parse() {
                    Ok(a) if a > 0.0 => a,
                    _ => {
                        bot.send_message(chat_id, "❌ Invalid amount. Please enter a positive number.").await?;
                        return Ok(());
                    }
                };

                match repo.find_by_telegram_id(telegram_id).await {
                    Ok(Some(user)) => {
                        let input_mint = jupiter::resolve_token_mint(token_input);

                        let decimals = match resolve_decimals(&config.solana_rpc_url, &input_mint).await {
                            Ok(d) => d,
                            Err(e) => {
                                bot.send_message(chat_id, format!("❌ Failed to resolve token: {}", e)).await?;
                                return Ok(());
                            }
                        };

                        let amount_raw = (token_amount * 10f64.powi(decimals as i32)).round() as u64;

                        bot.send_message(
                            chat_id,
                            format!(
                                "🔄 Selling {} {}, searching for the best route\\.\\.\\.",
                                escape_markdown_v2(&token_amount.to_string()),
                                escape_markdown_v2(token_input)
                            )
                        ).parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;

                        match jupiter::perform_swap(
                            &openfort,
                            &config.jupiter_api_key,
                            &user.openfort_account_id,
                            &user.solana_address,
                            &input_mint,
                            jupiter::tokens::SOL,
                            amount_raw,
                            100,
                            config.referral_account.as_deref(),
                            Some(config.referral_fee_bps),
                        ).await {
                            Ok(result) => {
                                let received_sol = format_token_amount(&result.output_amount_result, 9);

                                bot.send_message(
                                    chat_id,
                                    format!(
                                        "✅ Sold\\!\n\nSold: {} {}\nReceived: {} SOL\nTXID: `{}`",
                                        escape_markdown_v2(&token_amount.to_string()),
                                        escape_markdown_v2(token_input),
                                        escape_markdown_v2(&received_sol),
                                        result.signature
                                    )
                                ).parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;
                            }
                            Err(e) => {
                                println!("❌ Sell failed: {}", e);
                                bot.send_message(chat_id, format!("❌ Sale failed: {}", e)).await?;
                            }
                        }
                    }
                    Ok(None) => {
                        bot.send_message(chat_id, "⚠️ Please create a wallet first: /create_wallet").await?;
                    }
                    Err(e) => {
                        bot.send_message(chat_id, format!("❌ DB error: {}", e)).await?;
                    }
                }
                return Ok(());
            }

            // ============================
            // /withdraw
            // ============================
            if text.starts_with("/withdraw") {
                println!("💸 Processing withdraw for user: {}", telegram_id);

                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.len() < 3 {
                    bot.send_message(
                        chat_id,
                        "❌ Please specify amount and address.\n\nExample: /withdraw 0.1 SOL_ADDRESS"
                    ).await?;
                    return Ok(());
                }

                let amount_str = parts[1];
                let to_address = parts[2];

                let amount: f64 = match amount_str.parse() {
                    Ok(a) => a,
                    Err(_) => {
                        bot.send_message(chat_id, "❌ Invalid amount. Please use a number.").await?;
                        return Ok(());
                    }
                };

                match repo.find_by_telegram_id(telegram_id).await {
                    Ok(Some(user)) => {
                        println!("✅ Found user with wallet: {}", user.solana_address);
                        println!("✅ Openfort account ID: {}", user.openfort_account_id);

                        match solana::get_balance(&config.solana_rpc_url, &user.solana_address).await {
                            Ok(balance) => {
                                let amount_lamports = (amount * 1_000_000_000.0) as u64;
                                let balance_lamports = (balance * 1_000_000_000.0) as u64;

                                println!("💰 Balance: {} SOL, Amount: {} SOL", balance, amount);
                                println!("📊 Balance lamports: {}, Amount lamports: {}", balance_lamports, amount_lamports);

                                if balance_lamports < amount_lamports {
                                    bot.send_message(
                                        chat_id,
                                        format!(
                                            "❌ Insufficient balance!\n\nBalance: {:.6} SOL\nRequired: {} SOL",
                                            balance, amount
                                        )
                                    ).await?;
                                    return Ok(());
                                }

                                bot.send_message(
                                    chat_id,
                                    format!(
                                        "🔄 Withdrawing {} SOL to `{}`\\.\\.\\.\n\n_Transaction is being processed\\.\\.\\._",
                                        escape_markdown_v2(&amount.to_string()),
                                        to_address
                                    )
                                ).parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;

                                println!("⏳ Calling withdraw::withdraw_sol...");
                                println!("📤 account_id: {}", user.openfort_account_id);

                                match withdraw::withdraw_sol(
                                    &openfort,
                                    &user.openfort_account_id,
                                    &user.solana_address,
                                    to_address,
                                    amount_lamports,
                                    "devnet",
                                ).await {
                                    Ok(txid) => {
                                        println!("✅ Withdraw successful! TXID: {}", txid);
                                        bot.send_message(
                                            chat_id,
                                            format!(
                                                "✅ Withdraw sent\\!\n\nTXID: `{}`\n\n[View on Explorer](https://explorer.solana.com/tx/{}?cluster={})",
                                                txid,
                                                txid,
                                                config.solana_network
                                            )
                                        ).parse_mode(teloxide::types::ParseMode::MarkdownV2).await?;
                                    }
                                    Err(e) => {
                                        println!("❌ Withdraw failed: {}", e);
                                        bot.send_message(
                                            chat_id,
                                            format!("❌ Withdraw failed: {}", e)
                                        ).await?;
                                    }
                                }
                            }
                            Err(e) => {
                                println!("❌ Balance check failed: {}", e);
                                bot.send_message(chat_id, format!("❌ Balance check failed: {}", e)).await?;
                            }
                        }
                    }
                    Ok(None) => {
                        println!("⚠️ No wallet found for user: {}", telegram_id);
                        bot.send_message(chat_id, "⚠️ No wallet. Use /create_wallet first.").await?;
                    }
                    Err(e) => {
                        println!("❌ DB error: {}", e);
                        bot.send_message(chat_id, format!("❌ DB error: {}", e)).await?;
                    }
                }
                return Ok(());
            }

            // ============================
            // /execute, /cancel
            // ============================
            if text == "/execute" || text == "/cancel" {
                bot.send_message(
                    chat_id,
                    "⚠️ Execute/cancel functionality coming soon!"
                ).await?;
                return Ok(());
            }

            bot.send_message(chat_id, "Unknown command. Try /start for help.").await?;
            Ok(())
        }
    };

    teloxide::repl(bot, handler).await;

    Ok(())
}