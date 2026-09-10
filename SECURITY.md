# Security

Security is a core design consideration of this project and an area of active, ongoing work.

This repository is, at its core, a reference architecture for a specific class of problem: what happens when a messaging application's identity — a Telegram account — becomes the primary user interface and authorization entry point to a managed, backend-controlled Solana wallet. That framing is the actual contribution this project makes to the Solana developer ecosystem. The interesting engineering problem here is the identity/authorization boundary itself, ahead of the trading-bot UI wrapped around it.

> **Scope:** This document describes the security model, known limitations, and planned hardening of the Solana Starter Kit Bot. It is a living document that will be updated as the [proposed roadmap](./README.md#roadmap) progresses.

---

## Table of Contents

- [Security Model Overview](#security-model-overview)
- [Key Material and Export Boundary](#key-material-and-export-boundary)
- [Three-Layer Trust Model](#three-layer-trust-model)
- [Provider Independence](#provider-independence)
- [Credentials and Scope Model](#credentials-and-scope-model)
- [Compromised Component Analysis](#compromised-component-analysis)
- [Current Limitations](#current-limitations)
- [Planned Security Hardening](#planned-security-hardening)
- [Recovery Threat Model](#recovery-threat-model)
- [Reporting Security Issues](#reporting-security-issues)

---

## Security Model Overview

Per Openfort's own classification, the backend-wallet pattern used here is **custodial**: the backend controls the wallet's signing authority. This is a deliberate architecture choice for a Telegram-native application where the wallet is intended to remain invisible to the user.

The project **does not claim** that this architecture is non-custodial. Instead, the security objective is to minimize and document the risks inherent in a backend-controlled signing model.

Custody here specifically means **authorization control**, not direct possession of raw private-key material by the application. The application does not store or directly access private keys. Signing is delegated to Openfort Backend Wallet infrastructure and its secure signing environment.

However, the project does not treat the provider's signing infrastructure as a black-box guarantee. Openfort exposes separate API permissions for policy administration and private-key-share export. These capabilities are therefore treated as explicit trust-boundary considerations.

The intended production configuration is to provision runtime credentials according to least privilege and to exclude `private_key_shares:export`, policy administration, API-key administration, and other capabilities that are not required for normal runtime operation.

The project will independently verify the exact semantics of these permissions, the applicability of policy controls to Solana backend wallets, and the behavior of the system under compromised application credentials.

**The central security distinction** is between key protection and signing authorization. Protecting key material inside the signing infrastructure does not, by itself, prevent a compromised backend from requesting an unauthorized signature. This proposed security work therefore focuses on reducing the authorization surface and documenting which protections remain effective after different levels of backend compromise.

---

## Key Material and Export Boundary

Beyond the custody delegation described above, the Openfort project configuration exposes separate API-key permissions for **Private key shares → export**. This means that the security model cannot be reduced to "the key never leaves the TEE."

The project therefore treats private-key-share export as a separate administrative capability and will explicitly verify:

- which credentials can request private-key-share export;
- whether the capability applies to Solana backend wallets;
- what material is returned by the export operation;
- whether exported shares can independently enable signing or must be combined with other shares;
- whether the production bot credential can be provisioned without this capability;
- and how export capability affects the threat model under backend or credential compromise.

**The intended production configuration is to disable private-key-share export for all runtime credentials unless it is demonstrably required.**

---

## Three-Layer Trust Model

The security model is organized into three layers, each with a distinct trust boundary:

### Layer 1 — Account Identity
Telegram authentication, account recovery, and anti-takeover protections (cooldowns, notifications, revocation). Governs who is recognized as the legitimate operator of a given application account.

### Layer 2 — Application-Level Transaction Authorization
Withdrawal confirmation, limits, address validation, transaction simulation, rate limiting, monitoring, and anomaly detection. Governs which transactions the backend is willing to request signatures for.

### Layer 3 — Wallet Infrastructure
Openfort Backend Wallet infrastructure, its documented secure signing environment, Policy V2 controls where applicable, and credential-level restrictions. This layer is **intended** to provide constraints that remain outside the application's transaction-authorization logic.

A core part of the proposed work is to verify which of these controls are actually enforced for Solana backend wallets and how they behave under compromised application credentials.

Within Layer 3, the project further distinguishes:

```
Openfort
│
├── Signing environment / TEE
│
├── API credentials + scopes
│
├── Policy engine
│
└── Key-share export / migration capabilities
```

---

## Provider Independence

Openfort is the current wallet infrastructure provider used by this reference implementation. It is not treated as a trusted-by-default security boundary, and its specific implementation is not assumed to be the security model of the application.

The project is not provider-agnostic today. Parts of the code are written specifically for Openfort: how transactions are signed, how the `X-Wallet-Auth` JWT is built, how payloads are encoded, and how differences between Openfort API versions are handled.

The project does commit to three things, all part of the M6 milestone (see the [Roadmap](./README.md#roadmap)):

- Isolating that Openfort-specific code behind a clear internal boundary.
- Documenting which parts of the security model are specific to Openfort and which apply to any backend-wallet provider.
- Comparing the architecture, at a conceptual level, against other Solana wallet providers such as Turnkey and Crossmint.

This is a comparative documentation exercise, not a commitment to build or maintain a second working provider integration — doing so would meaningfully expand scope beyond what the proposed roadmap covers.

```
         Telegram / Application
                 │
                 ▼
      ┌──────────────────────┐
      │ Wallet Security      │
      │ Abstraction Layer    │
      │                      │
      │ AuthZ                │
      │ Recovery             │
      │ Transaction policy   │
      │ Monitoring           │
      │ Threat model         │
      └──────────┬───────────┘
                 │
          Provider adapter
                 │
      ┌──────────┴──────────┐
      ▼                     ▼
  Openfort              Turnkey / ...
```

---

## Credentials and Scope Model

The intended production credential configuration is summarized below. Exact Openfort scope semantics and policy behavior for Solana backend wallets are subject to verification in M4.

| Credential | Purpose | Must have | Must NOT have |
|---|---|---|---|
| **Key A — Runtime Secret** | Normal wallet/account operations and authorized signing | Minimum required `accounts:*` / `transaction:*` scopes | `policies:write`, `policies:delete`, `private_key_shares:export`, API-key management |
| **Key B — Policy Provisioning** | Provisioning of predefined wallet policies | Minimum policy-management scope required after verification | `accounts:sign`, `private_key_shares:export` |
| **Key C — Security Administration** | Manual security administration | Policy administration only | `accounts:sign`, `private_key_shares:export` |
| **Wallet Secret** | Backend-wallet signing authentication (JWT for `X-Wallet-Auth`) | Required by Openfort signing flow | Never exposed to users or application input |

> **Note:** Key A is a secret credential even though it carries limited scopes. It is never "public."

> **Design decision — export denied by default, for every credential.** No credential provisioned for this project — including the security-administration credential (Key C, otherwise the most privileged) — is granted `private_key_shares:export`. This is a deliberate provisioning choice, not an oversight: it closes the export path entirely at the credential level, for every key in the project, independent of the separate and still-open question (see Key Material and Export Boundary above) of whether a single exported share would even be sufficient on its own to reconstruct a usable signing key. If `TEE-only signing` is claimed anywhere in this project's materials, it is true specifically because export is denied to every credential — not because export is technically impossible for a credential that were granted it.

> **Unverified assumption — shared wallet secret.** Openfort documents only one active wallet secret per project at a time. If Keys A, B, and C all authenticate their `X-Wallet-Auth` JWTs using that same shared wallet secret, compromise of the wallet secret itself may be sufficient to forge a valid signing-authorization request regardless of which Bearer-scoped API key is otherwise used — independent of the credential-isolation work above. This is an explicit, separate verification item (M4) — scope separation alone does not resolve it.

---

## Compromised Component Analysis

The table below makes Layer 3's boundary concrete by breaking down exactly what an attacker gains from compromising each individual component of the system:

| Compromised component | Can sign? | Modify policies? | Delete policies? | Export key shares? |
|---|---|---|---|---|
| Bot runtime / Key A | ✅ | ❌ | ❌ | ❌ |
| Policy provisioning / Key B | ❌ | TBD / minimum required | ❌ | ❌ |
| Security admin / Key C | ❌ | ✅ | ✅ | ❌ |
| Database | ❌ | ❌ | ❌ | ❌ |
| Openfort signing infrastructure | Provider-dependent | Provider-dependent | Provider-dependent | Provider-dependent |

**The absence of `private_key_shares:export` from runtime credentials is an explicit security requirement, not an assumption.**

### Unverified assumption

This table depends on an assumption that is explicitly **not yet verified** and is itself part of the proposed work (M4): whether Key B (scoped to `policies:write`, without `policies:delete`) can attach a new, more permissive policy to an *existing* wallet that already has a policy attached — and, if so, how Openfort's project-level and account-level policies are combined or take precedence over one another.

If a compromised Key B can effectively override an existing wallet's policy by creating a new one, credential scoping alone does not close this gap, and the fixed, non-arbitrary provisioning workflow described in M4 becomes the primary control for this specific risk.

---

## Current Limitations

This repository is a working starter kit, not a fully hardened production system. Known limitations in the current implementation include:

- Withdrawals execute immediately without a confirmation step.
- Withdrawal limits are not currently enforced.
- There is no dedicated transaction history.
- Telegram command rate limiting is not currently implemented.
- Replay protection at the Solana transaction level (preventing the same swap or withdrawal from being submitted twice) beyond Solana's own blockhash expiry is not implemented — this is distinct from Openfort API request replay: each signing request includes a freshly generated JWT nonce (`jti`), and whether Openfort's server actually validates and rejects reused `jti` values remains an open item to verify during M3/M4.
- Transactions are not simulated before signing.
- There is no recovery path if a user's Telegram identity changes or is lost.
- Secrets are currently provided via environment variables only; integration with a dedicated secret manager (e.g. AWS Secrets Manager, Google Secret Manager, HashiCorp Vault) for production deployments is not yet implemented.
- SPL token withdrawals require the recipient's Associated Token Account (ATA) to already exist for that mint. The recipient ATA is not auto-created — doing so would mean someone (the project, via Kora, or the user) pays the one-time account-rent cost, and this project has not yet decided who that should be. A withdrawal to a recipient without an existing ATA fails with a clear error rather than silently creating one.
- SPL token withdrawals are not verified against Token-2022 mints that use extensions affecting transfer amounts (e.g. transfer fees, transfer hooks). `transfer_checked` does not account for those; only the legacy SPL Token program and plain Token-2022 mints (no amount-affecting extensions) are supported.
- Token symbol shorthand (`USDC`, `USDT`, etc., used by `/buy`, `/sell`, and `/withdraw`) resolves against a hardcoded table of **mainnet** mint addresses only. On devnet, these symbols do not resolve — the raw devnet mint address must be passed directly. Balance lookups, decimals, and token-program resolution are otherwise mint-agnostic and RPC-driven.

These limitations are intentionally documented so developers can clearly understand what the reference implementation does today and what still needs to be hardened before exposing it to real users with meaningful funds.

---

## Planned Security Hardening

The roadmap focuses on a defined set of security improvements:

- Withdrawal confirmation and configurable withdrawal limits.
- Address validation.
- Rate limiting reference implementation.
- Replay / duplicate-action protection.
- Transaction simulation before signing.
- Structured transaction and security logging.
- Independently verified account recovery with anti-takeover safeguards.
- Independent security review and remediation of critical or high-risk findings.
- Wallet secret rotation policy for the Openfort signing key, leveraging Openfort's built-in rotation endpoint.
- Reference integration with a platform secret manager (AWS Secrets Manager, Google Secret Manager, or HashiCorp Vault) for production secret storage.
- Verification of whether Keys A, B, and C share a single underlying Openfort wallet secret, and documentation of how that affects the credential-isolation model (see Credentials and Scope Model above).
- Verification of whether Openfort's server actually enforces uniqueness on the `jti` nonce carried by each `X-Wallet-Auth` JWT (see Current Limitations above).
- A documented internal boundary isolating Openfort-specific integration code from application-level security logic (see Provider Independence above).

The goal is not to claim that the starter kit becomes universally "production secure." Instead, the project will provide a significantly stronger and better-documented security baseline that developers can evaluate and extend for their own applications.

---

## Recovery Threat Model

> **Status:** Planned (M2 — Account Recovery & Anti-Takeover Protection). This section describes the intended design, not the current implementation.

### What the recovery mechanism proves

**This is account recovery, not key recovery.**

The recovery mechanism does not recover, reconstruct, export, or directly access the wallet's private signing key. Instead, it restores the association between a verified Telegram identity and an existing Openfort account, allowing the application to again request authorized signing operations for that account. It does not, however, protect against compromise of the backend's signing credentials or the Openfort authorization layer.

### Design principles

- **TOTP-based recovery factor**, generated and confirmed once at wallet creation (QR code sent once; the bot deletes the message shortly after setup is confirmed).
- **High-entropy, single-use backup codes**, generated at the same time, shown once, stored only as hashes — not as a substitute for TOTP, but as a documented fallback if the authenticator device is lost.
- Recovery is initiated from a new Telegram identity by supplying a recovery identifier plus a valid TOTP code or an unused backup code.
- A **recovery cooldown period** before the new Telegram identity is bound, giving the legitimate owner a window to notice and cancel an unauthorized attempt.
- **Notification to the original Telegram identity**, if still reachable, with an explicit cancel action.
- Backup codes are **single-use**; a successful recovery revokes the code used and prompts re-registration of the TOTP factor.
- **Recovery-specific rate limiting**, separate from general command rate limiting.
- **Baseline anti-enumeration**: a uniform error response regardless of whether the supplied recovery identifier corresponds to a real account. Timing-safe (constant-time) hardening against side-channel latency analysis is deferred as follow-up work — the uniform error response closes the primary enumeration vector; constant-time comparison closes a narrower, lower-severity residual gap.
- **Audit logging** of all recovery attempts, successful and failed.

### Scope control

The project will not require a third-party identity provider for the initial recovery implementation. The goal is a self-contained reference architecture using standard, well-understood primitives (TOTP, hashed backup codes), extensible later with stronger external identity mechanisms if a production application requires them.

---

## Reporting Security Issues

If you discover a security vulnerability in this project, please report it responsibly:

1. **Do not open a public issue.**
2. Email **jobpostgm@gmail.com** with the subject line `[SECURITY] Solana Starter Kit Bot`.
3. Include a description of the vulnerability, steps to reproduce, and the affected component(s).
4. Allow reasonable time for assessment and remediation before public disclosure.

Security reports are taken seriously and will be addressed as a priority.
