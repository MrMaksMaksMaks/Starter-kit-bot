# Security

Security is a core design consideration of this project and an area of active, ongoing work.

This repository is, at its core, a reference architecture for a specific and under-documented class of problem: what happens when a messaging application's identity — a Telegram account — becomes the front door to a managed, backend-signed Solana wallet. That framing, not "a Telegram trading bot," is the actual contribution this project makes to the Solana developer ecosystem.

> **Scope:** This document describes the security model, known limitations, and planned hardening of the Solana Starter Kit Bot. It is a living document that will be updated as the [funded roadmap](../README.md#roadmap) progresses.

---

## Table of Contents

- [Security Model Overview](#security-model-overview)
- [Three-Layer Trust Model](#three-layer-trust-model)
- [Compromised Component Analysis](#compromised-component-analysis)
- [Current Limitations](#current-limitations)
- [Planned Security Hardening](#planned-security-hardening)
- [Recovery Threat Model](#recovery-threat-model)
- [Reporting Security Issues](#reporting-security-issues)

---

## Security Model Overview

Per Openfort's own classification, the backend-wallet pattern used here is **custodial**: "This pattern is fully custodial — your backend controls the key" (Openfort, *Server-side user wallets*). This is a deliberate architecture choice, not an oversight — Openfort explicitly recommends this pattern for products where the wallet should be invisible to the user, which matches this project's Telegram-native use case.

End users have no independent signing capability of their own; the application, through Openfort, holds full authorization control. The security work described in this document — and funded through the project's roadmap — is about hardening the trust boundaries of that custody model, not about claiming the architecture avoids custody altogether.

**Custody here specifically means authorization control, not possession of raw key material:** the application itself never stores or directly accesses private keys. Signing is delegated to Openfort Backend Wallet infrastructure, where key material is designed to be protected by Openfort's secure signing environment (TEE). The project does not assume that this eliminates all key-material export or administrative paths: Openfort exposes separate wallet-share export capabilities, which are therefore treated as part of the credential and trust-boundary analysis.

The private_key_shares:export scope is explicitly omitted from all runtime credentials (Key A, Key B, and Key C), ensuring that even with full compromise of application-layer components, private key material cannot be extracted from Openfort's TEE environment.

The bot backend can initiate authorized signing requests, but it never receives or handles private key material itself. Each authenticated request to Openfort carries a freshly generated nonce (`jti`), which mitigates replay of the same API authentication request at the Openfort layer. This is distinct from blockchain-level transaction replay, which is handled by Solana's blockhash expiry.

This distinction matters: TEE protection prevents key extraction, but it does not, by itself, make the application backend a trusted-by-default component for authorization. The separation of credentials (signing vs. policy management vs. security administration) ensures that compromise of any single component does not grant full system control.

---

## Three-Layer Trust Model

The security model is organized into three layers, each with a distinct trust boundary:

### Layer 1 — Account Identity
Telegram authentication, account recovery, and anti-takeover protections (cooldowns, notifications, revocation). Governs who is recognized as the legitimate operator of a given application account.

### Layer 2 — Application-Level Transaction Authorization
Withdrawal confirmation, limits, address validation, transaction simulation, rate limiting, monitoring, and anomaly detection. Governs which transactions the backend is willing to request signatures for.

### Layer 3 — Wallet Infrastructure
Openfort Backend Wallet, TEE-protected signing, Policy V2 controls where applicable, and credential/IP restrictions. Governs what Openfort's signing infrastructure will actually sign, independent of whether the request came from a legitimate or compromised instance of the application backend.

The project does not assume that Layer 2, or mutable off-chain Layer 3 controls, remain trustworthy after a complete backend compromise that includes administrative Openfort credentials. Layers 1 and 2 protect against account takeover and unsafe transaction requests within a legitimate application context; they are not a substitute for the independent, Openfort-side constraints that Layer 3 aims to add.

---

## Compromised Component Analysis

The table below makes Layer 3's boundary concrete — specifically, what an attacker gains from compromising each individual component of the system, rather than a generic "backend compromise":

| Compromised component | What the attacker gains | Can sign transactions? | Can modify policies? | Can delete policies? |
|---|---|---|---|---|
| Bot runtime (Key A) | Key A credentials | ✅ | ❌ | ❌ |
| Policy provisioning path (Key B) | Key B credentials | ❌ | Potentially ✅ — subject to verification (see below) | ❌ |
| Database | User/account IDs, application metadata | ❌ | ❌ | ❌ |
| Security administration environment (Key C) | Key C credentials | ❌* | ✅ | ✅ |
| Openfort signing infrastructure | TEE boundary | Depends on Openfort's own controls | Depends on Openfort's own controls | Depends on Openfort's own controls |

*\*assuming Key C is genuinely provisioned without `accounts:sign`.*

> **Private key export is explicitly disabled at the scope level.** All runtime credentials are provisioned without the `private_key_shares:export` scope. This is an explicit security requirement, not an assumption — it ensures that even with full compromise of application-layer components, the attacker cannot extract private key material from Openfort's TEE environment. The only way to obtain private keys would require compromising the TEE itself, which is outside the application's threat model and is independently secured by Openfort's infrastructure.

### Unverified assumption

This table depends on an assumption that is explicitly **not yet verified** and is itself part of the funded work (M4): whether Key B (scoped to `policies:write`, without `policies:delete`) can attach a new, more permissive policy to an *existing* wallet that already has a policy attached — and, if so, how Openfort's project-level and account-level policies are combined or take precedence over one another.

If a compromised Key B can effectively override an existing wallet's policy by creating a new one, credential scoping alone does not close this gap, and the fixed, non-arbitrary provisioning workflow described in M4 becomes the primary control, not a secondary one.

The "Policy provisioning path" row is precisely why the roadmap proposes credential isolation for policy management, with policy provisioning itself restricted to a fixed, non-user-controlled workflow rather than accepting arbitrary policy definitions: without both of those controls together, the credentials needed for routine wallet provisioning are the same credentials that could be used to weaken an existing wallet's protection.

---

## Current Limitations

This repository is a working starter kit, not a fully hardened production system. Known limitations in the current implementation include:

- Withdrawals execute immediately without a confirmation step.
- Withdrawal limits are not currently enforced.
- There is no dedicated transaction history.
- Telegram command rate limiting is not currently implemented.
- Replay protection at the Solana transaction level (preventing the same swap or withdrawal from being submitted twice) beyond Solana's own blockhash expiry is not implemented — this is distinct from Openfort API request replay, which is already mitigated via per-request JWT nonces.
- Transactions are not simulated before signing.
- There is no recovery path if a user's Telegram identity changes or is lost.
- Secrets are currently provided via environment variables only; integration with a dedicated secret manager (e.g. AWS Secrets Manager, Google Secret Manager, HashiCorp Vault) for production deployments is not yet implemented.

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

The goal is not to claim that the starter kit becomes universally "production secure." Instead, the project will provide a significantly stronger and better-documented security baseline that developers can evaluate and extend for their own applications.

---

## Recovery Threat Model

> **Status:** Planned (M2 — Account Recovery & Anti-Takeover Protection). This section describes the intended design, not the current implementation.

### What the recovery mechanism proves

The recovery mechanism restores the **association between a Telegram identity and an existing Openfort account** (Layer 1). It never touches, reconstructs, or gains access to the wallet's private signing key, which remains inside Openfort's infrastructure throughout and is entirely unaffected by the recovery process.

This mechanism protects against Telegram-identity takeover. It does not, and cannot, protect against a compromise of the backend's own signing authorization (addressed separately in M4 and the Layer 3 analysis above).

### Design principles

- **TOTP-based recovery factor**, generated and confirmed once at wallet creation (QR code shown a single time, never re-displayed).
- **High-entropy, single-use backup codes**, generated at the same time, shown once, stored only as hashes — not as a substitute for TOTP, but as a documented fallback if the authenticator device is lost.
- Recovery is initiated from a new Telegram identity by supplying a recovery identifier plus a valid TOTP code or an unused backup code.
- A **recovery cooldown period** before the new Telegram identity is bound, giving the legitimate owner a window to notice and cancel an unauthorized attempt.
- **Notification to the original Telegram identity**, if still reachable, with an explicit cancel action.
- Backup codes are **single-use**; a successful recovery revokes the code used and prompts re-registration of the TOTP factor.
- **Recovery-specific rate limiting**, separate from general command rate limiting.
- **No account-existence disclosure** prior to successful verification (protection against enumeration attacks).
- **Audit logging** of all recovery attempts, successful and failed.

### Scope control

The project will not require a third-party identity provider for the initial recovery implementation. The goal is a self-contained reference architecture using standard, well-understood primitives (TOTP, hashed backup codes) rather than novel cryptography, extensible later with stronger external identity mechanisms if a production application requires them.

---

## Reporting Security Issues

If you discover a security vulnerability in this project, please report it responsibly:

1. **Do not open a public issue.**
2. Email **jobpostgm@gmail.com** with the subject line `[SECURITY] Solana Starter Kit Bot`.
3. Include a description of the vulnerability, steps to reproduce, and the affected component(s).
4. Allow reasonable time for assessment and remediation before public disclosure.

Security reports are taken seriously and will be addressed as a priority.
