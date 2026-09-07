# Roadmap

This document describes the planned hardening work for Solana Starter Kit Bot in technical detail - what each milestone delivers, why it's scoped the way it is, and how the milestones depend on each other. It intentionally does not include budget figures.

**Scope:** This roadmap focuses on hardening the existing working implementation into a reusable, security-reviewed foundation — not a full-featured trading platform. See [Out of scope](#out-of-scope) below for specifics, and the main [README](./README.md#roadmap) for why.

**On sequencing:** Milestones below are listed with approximate month numbers, but they are sequential, completion-triggered phases, not fixed calendar slots. Each milestone begins once the previous one is complete and accepted. M3 in particular depends on a third-party security firm's availability, which is not fully within the maintainer's control - its actual start date tracks M1–M2's real completion, not the calendar alone. If M3 is delayed, M4 and M5 can proceed in parallel during the wait, since neither modifies the code M3 reviews (see M3's note below).

---

## M1: Transaction Security and Safety Controls
**Month 1**

**Goal:** Move from "the transaction works" to "the transaction flow has basic safety controls" - the minimum needed before real users should be trusted with real funds.

**Deliverables**
- Withdrawal confirmation using Telegram inline keyboards.
- Configurable withdrawal limits, with clear user feedback when a limit is exceeded.
- Solana address validation before transaction construction; malformed or invalid destinations are rejected before signing.
- Rate limiting for sensitive bot commands, with stronger limits on withdrawal and transaction-related operations.
- Solana-level transaction replay / duplicate-submission protection. (This is distinct from Openfort API request replay, which is already mitigated today via per-request JWT nonces - this closes the separate gap at the transaction-submission level.)
- Transaction history for supported wallet operations: withdrawals, swaps, signatures, timestamps, status.
- Reconciliation of in-flight transactions against on-chain state on startup — resolving any records left in a pending status by an unexpected shutdown (crash, power loss, or planned restart), not just by an explicit shutdown procedure.
- Structured transaction and security logging.
- Initial security baseline and threat-model documentation.
- Automated tests for critical withdrawal and wallet flows.

**Expected outcome:** The existing wallet and withdrawal implementation has explicit user confirmation, configurable transaction controls, replay-resistant transaction submission, auditable transaction records, and a documented security baseline for subsequent hardening work.

---

## M2: Account Recovery and Anti-Takeover Protection
**Month 2**

**Goal:** Make the Telegram-identity-to-wallet association recoverable while minimizing the risk of unauthorized account takeover.

**This is account recovery, not key recovery.** The mechanism restores the association between a Telegram identity and an existing Openfort account. It never touches, reconstructs, or gains access to the wallet's private signing key, which remains inside Openfort's infrastructure throughout. This protects against Telegram-identity takeover. It does not, and cannot, protect against a compromise of the backend's own signing authorization (addressed separately in M4).

**Deliverables**
- TOTP-based recovery factor, generated and confirmed once at wallet creation (QR code sent once; the bot deletes the message shortly after setup is confirmed).
- High-entropy, single-use backup codes, generated at the same time, shown once and stored only as hashes. It's a documented fallback if the authenticator device is lost, not a substitute for TOTP.
- Recovery initiated from a new Telegram identity by supplying a recovery identifier plus a valid TOTP code or an unused backup code.
- A recovery cooldown period before the new Telegram identity is bound, giving the legitimate owner a window to notice and cancel an unauthorized attempt.
- Notification to the original Telegram identity, if still reachable, with an explicit cancel action — best-effort, since delivery can fail, but requires no "change detection" to implement.
- Backup codes are single-use; a successful recovery revokes the code used and prompts re-registration of the TOTP factor.
- Recovery-specific rate limiting, separate from general command rate limiting.
- Baseline anti-enumeration: a uniform error response regardless of whether the supplied recovery identifier corresponds to a real account.
- Audit logging of all recovery attempts, successful and failed.
- Automated tests covering successful recovery, invalid attempts, replayed/reused backup codes, and unauthorized reassignment attempts.

**Scope control:** No third-party identity provider is required for this implementation. The design uses standard, well-understood primitives (TOTP, hashed backup codes) rather than novel cryptography, extensible later with stronger external identity mechanisms if a production application requires them.

**Non-goal for this milestone:** Timing-safe (constant-time) anti-enumeration hardening — protecting specifically against side-channel timing analysis of response latency — is deferred as follow-up work. The uniform error-response message closes the primary enumeration vector; constant-time comparison closes a narrower, lower-severity residual gap.

---

## M3: Independent Security Review and Remediation
**Month 3 (target)**

**Goal:** Provide independent, external security verification of the highest-risk code — wallet association, signing, withdrawal, and recovery — completed in M1 and M2, before further feature work builds on top of it.

**Deliverables**
- External security review covering: wallet/account association, transaction construction, transaction signing flow, withdrawal flow, authentication/authorization boundaries, the recovery flow, and sensitive configuration/secret handling.
- Threat-model review by an independent security professional.
- Documented findings, prioritized by severity, with remediation recommendations.
- Remediation of critical and high-risk findings within scope.
- Regression / re-testing after remediation.
- Public documentation of relevant findings and implemented fixes, where disclosure is appropriate.

**Why this is scoped to happen right after M1–M2, not at the end of the project:** the review's scope is deliberately limited to the wallet, signing, withdrawal, and recovery code completed in M1–M2. Infrastructure hardening (M4) and testing/developer-experience work (M5) do not modify this reviewed surface, so reviewing immediately after the highest-risk code is complete - rather than waiting until the end — allows critical findings to be fixed with the rest of the roadmap still ahead, instead of being discovered at the very end with no room left to address them.

**Contingency:** If reviewer availability or scope constraints require it, review priority goes to the highest-risk flows first — signing, withdrawal, and recovery — with any remaining scope addressed as a documented follow-up rather than silently dropped.

---

## M4: Infrastructure Hardening and Operational Reliability
**Month 4**

**Goal:** Move from a single-provider, undocumented operational setup to a more resilient reference deployment architecture, and reduce the authorization surface that a compromised application backend could exploit.

**Deliverables**

*Operational reliability:*
- Support for multiple Solana RPC providers, with automatic fallback on provider failure.
- Improved handling of external API failures involving Solana RPC, Jupiter, Openfort, and Kora.
- A simple, documented SQLite backup script (scheduled file copy) and a documented recovery procedure — a working baseline, not a production-grade backup pipeline (WAL-aware snapshotting, retention policies, automated restore testing).
- Structured operational logs for transactions and security-relevant events.
- Documentation covering production-oriented operational configuration.

*Least-privilege and Openfort security hardening:*
- Credential isolation for Openfort policy management, using separate least-privilege API credentials:
  - **Key A — Bot runtime:** signing and routine account operations; no `policies:write/delete`, no `accounts:export`, no `private_key_shares:export`.
  - **Key B — Policy provisioning:** `policies:read/write` only, no `accounts:sign`. Never accepts arbitrary policy JSON — limited to a fixed, non-user-controlled workflow of predefined policy profiles.
  - **Key C — Security administration:** `policies:read/write/delete`, explicitly without `accounts:sign` and without `private_key_shares:export`; held and used outside the production runtime.
- Verification of whether Keys A, B, and C share a single underlying Openfort wallet secret (Openfort documents only one active wallet secret per project), and what that implies for the credential-isolation model if so.
- Verification of whether Openfort's server actually enforces uniqueness on the `jti` nonce carried by each signing-authentication request — confirming or ruling out real replay protection at that layer, rather than assuming it from the nonce's presence alone.
- Verification of whether a Key-B-scoped credential can attach a new, more permissive policy to a wallet that already has one, and how Openfort resolves multiple project-level and account-level policies on the same account.
- Restrict production API-key access by IP where supported.
- Keep Openfort credentials outside source control at every stage, including CI/CD and deployment tooling.
- Monitor signing requests and policy changes; detect abnormal signing activity - sudden increases in signing-request volume, transactions to previously-unseen destinations, unusual transaction frequency or value.
- Explicit, public documentation of the verified trust boundaries of this credential model - including any case where scope separation doesn't hold as expected.

*Key-material export verification:*
- Verify whether the `private_key_shares:export` permission applies to Solana backend wallets, and what material the export operation actually returns.
- Determine whether exported shares can independently enable signing or must be combined with other shares.
- Confirm that production runtime credentials can be provisioned without this capability.
- Document how key-share export capability affects the threat model under backend or credential compromise.

**Non-goals for this milestone:** A fully separate, network-isolated policy-management service (its own deployment, monitoring, and authenticated internal API) is a stronger but materially more expensive version of the credential-isolation control above. It's documented as a possible future extension, not delivered here — the work above closes most of the practical risk (via a fixed internal function unreachable by arbitrary user input) without the added operational surface of a second service. Likewise, Openfort's on-chain, contract-enforced permissions are currently documented for EVM/ERC-4337 smart accounts, not confirmed available for Solana backend wallets, and are noted as a possible future direction rather than promised here. Administrative alerting infrastructure and metrics dashboards are also excluded - structured logs and backups are delivered, but real-time alerting/visualization tooling is documented as a production recommendation rather than implemented here.

**Expected outcome:** A more resilient reference deployment, and a credential model whose actual boundaries - not just intended ones — have been verified against the live Openfort API rather than assumed from documentation.

---

## M5: Testing, Developer Experience and Reproducibility
**Month 5**

**Deliverables**
- Expanded automated test coverage for wallet, balance, swap, withdrawal, and recovery flows, building on the initial tests from M1–M2.
- Integration-test structure for external Solana infrastructure where practical (controlled environments or mocked dependencies).
- Reproducible configuration examples.
- Improved error handling and developer-facing diagnostics.
- Inline Telegram keyboard flows for the security-sensitive interactions introduced during the project (confirmation, cancellation, recovery steps).
- Documentation of integration pitfalls and testing procedures.

**Expected outcome:** A developer cloning the repository can understand the architecture, configure the required infrastructure, run the application, and reproduce the main supported flows without reverse-engineering undocumented behavior.

---

## M6: Documentation, Operational Hardening and Public Release
**Month 6**

**Deliverables**
- Wallet secret rotation policy (e.g., every 90 days), leveraging Openfort's built-in secret-rotation endpoint. Applying a rotated secret requires a bot restart — the rotation procedure documents this explicitly, executed as a graceful shutdown (stop accepting new commands, let in-flight operations finish, then restart) rather than an abrupt process kill, with a user-facing notice during the window.
- Reference integration with a platform secret manager (e.g., AWS Secrets Manager, Google Secret Manager, or HashiCorp Vault) for production secret storage, in place of plain environment variables.
- **Wallet provider abstraction boundary:** the existing Openfort integration is structured behind a clear internal boundary, isolating provider-specific code from application-level security logic (authZ, recovery, transaction policy, monitoring). Which parts of the integration are Openfort-specific (API version quirks, payload encoding, message-vs-full-tx signing, credential scoping) versus generic to any backend-wallet provider is explicitly documented. This is documentation and code organization, not a commitment to integrate additional providers.
- Complete architecture documentation.
- Production deployment guide: environment configuration, Openfort setup, Solana RPC configuration, Jupiter configuration, Kora configuration, database setup, deployment process, security considerations.
- Consolidated security documentation: assumptions, known limitations, transaction protection, the recovery model, operational recommendations.
- Configuration and environment-variable reference.
- Mainnet testing/demo documentation.
- Final README and developer onboarding documentation.
- Final review confirming documented functionality matches the actual implementation.
- Public release of the completed milestone set under the existing MIT license.

**Expected outcome:** A documented, reproducible open-source reference implementation - with a defined operational-security posture, not just application code — that developers can fork and adapt for Telegram-native Solana applications.

---

## Out of scope

Advanced trading features: limit orders, DCA, token sniping and copy trading are intentionally out of scope for this open-source repository. They carry meaningfully higher security, execution-reliability, and abuse-prevention requirements than the core wallet / swap / withdraw flows this repository demonstrates, and may be developed separately, outside this roadmap.

Full architecture and trust-boundary documentation for what's already built: [`SECURITY.md`](./SECURITY.md).

