# Hapi integration example

There are two ways to integrate Hapi into a project. Simplified and Extended

Key differences:

**Simplified** - simplicity (one risk score is selected for all categories).
**Extended** - flexibility (for each category you can set the individual risk score).

## What is categories and risk score? 

Hapi protocol supports 18 categories

| Category | Description |
|----------|-------|
| None | |
| WalletService | Wallet service - custodial or mixed wallets |
| MerchantService | Merchant service |
| MiningPool | Mining pool |
| LowRiskExchange | Low risk exchange - Exchange with high KYC standards |
| MediumRiskExchange | Medium risk exchange |
| DeFi | DeFi application |
| OTCBroker | OTC Broker |
| ATM | Cryptocurrency ATM |
| Gambling | Gambling |
| IllicitOrganization | Illicit organization |
| Mixer | Mixer |
| DarknetService | Darknet market or service |
| Scam | Scam |
| Ransomware | Ransomware |
| Theft | Theft - stolen funds |
| Counterfeit | Counterfeit - fake assets |
| TerroristFinancing | Terrorist financing |
| ChildAbuse | Child abuse and porn materials |


If the address belongs to some category, it will have a
Risk score (on the scale from 0..10, i.e. max risk).