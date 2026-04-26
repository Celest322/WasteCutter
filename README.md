DWasteCutter 

Stop bleeding cash on forgotten SaaS subscriptions. Find waste. Cut it. Keep profits.

What is WasteCutter?

WasteCutter is a subscription waste detection and recovery tool built on Stellar Soroban. Freelancers and small agencies upload credit card statements (CSV) or connect via Plaid, and our AI identifies recurring software subscriptions costing them $50-500 monthly. The app creates on-chain cancellation reminders with escrowed USDC deposits - if you cancel on time, you get your deposit back. If you forget, the deposit funds future features.

Real users. Real money. Real ROI.
The Problem We Solve

Person: Maria, a freelance project manager in Manila
Situation: She manages 12+ clients, each needing different SaaS tools (Canva Pro, Slack, Figma, Zoom)
The Friction: After a 2-week branding project ends, she forgets to cancel the Canva Pro team plan
The Cost: $119/month bleeds for 6 months = $714 wasted = 2 weeks of grocery money in Manila

The Market: Southeast Asia has 4.2M+ freelancers (Philippines, Indonesia, Vietnam). Average waste per freelancer: $340/year on unused subscriptions. Total addressable waste: $1.4B annually.
How It Works
Architecture Flow
text

User Uploads CSV → AI Categorization → Soroban Contract → Dashboard → User Action
        ↓                  ↓                    ↓              ↓            ↓
    Bank statement    Identifies subs     Creates escrow    Shows waste   Cancels & 
                                      reminder contract      dashboard     gets refund

Step-by-Step User Journey

    Upload - User uploads credit card CSV or connects via Plaid API

    Detect - AI (GPT-4o-mini) categorizes charges, identifies recurring software subscriptions, flags price hikes

    Set Reminder - User clicks "Set Cancellation Reminder" on any subscription

    Deposit Escrow - User deposits $5 USDC as commitment (held in Soroban contract)

    Get Notified - Contract emits event → Calendar invite + email reminder before due date

    Cancel & Refund - User cancels subscription, clicks "Confirm Cancellation" → Refund of $5 USDC

    Or Lose Deposit - If user forgets, $5 USDC funds platform development

On-Chain Flow (Technical)
rust

// 1. Add subscription from CSV
add_subscription(user, "Canva Pro", 11900, due_date, 500) → subscription_id

// 2. Set reminder with escrow
set_cancellation_reminder(user, subscription_id, reminder_timestamp) → true

// 3. Confirm cancellation
confirm_cancellation(user, subscription_id) → refund_amount (500 or 0)

Deployed Smart Contract
Contract ID (Testnet)
text

CA54SPYD3W6ACDBGLEERYQQ6TZ2XMLYWQP2GBCIFSPLKH7RGYXQ7D7N2

**View on Stellar Expert**

🔗 Click here to view the contract on Stellar Expert

(https://stellar.expert/explorer/testnet/contract/CA54SPYD3W6ACDBGLEERYQQ6TZ2XMLYWQP2GBCIFSPLKH7RGYXQ7D7N2?filter=history)
<img width="1908" height="942" alt="image" src="https://github.com/user-attachments/assets/37f48f60-da95-459d-8242-aac2f7087355" />

Above: Contract view showing initialization, subscription storage, and reminder functions
Key Features
MVP Features (Demo-ready)

    ✅ CSV bank statement upload

    ✅ AI subscription detection (simulated or GPT-4o-mini)

    ✅ Multi-subscription dashboard

    ✅ One-click reminder setup with escrow

    ✅ Cancellation confirmation with refund logic

    ✅ On-chain state tracking

Bonus Features (Post-MVP)

    🔄 Plaid API integration (direct bank connection)

    📧 Automated email reminders

    📱 Telegram bot notifications

    📊 Price hike alerts (AI comparison vs historical)

    🔗 Slack/GitHub API integration (seat activity verification)

Why Stellar?
Feature	How WasteCutter Uses It	Why Not Ethereum/Solana
Soroban Smart Contracts	Escrow logic, reminder storage	Ethereum gas would cost $50+ per subscription
USDC	Stable deposits and refunds	No volatility risk for user deposits
5-second finality	Instant reminder confirmation	Freelancers need immediate feedback
Cheap storage	Store 1000+ subscription records	$0.0001 per subscription vs $1+ on ETH
Address auth	User ownership verification	Built-in, no extra libraries
Technical Implementation
Prerequisites
bash

rustc 1.75+
stellar-cli 20.0.0
cargo 1.75+

Installation & Testing
bash

# Clone repository
git clone https://github.com/yourusername/wastecutter
cd wastecutter

# Build contract
cargo build --target wasm32-unknown-unknown --release
soroban contract build

# Run tests (5 comprehensive tests)
cargo test

# Expected output:
# test test_happy_path_add_and_cancel_subscription ... ok
# test test_edge_case_unauthorized_user ... ok
# test test_state_verification_after_reminder_set ... ok
# test test_multiple_subscriptions_per_user ... ok
# test test_late_cancellation_forfeits_escrow ... ok

Deploy to Testnet
bash

# Generate identity (first time only)
stellar keys generate wastecutter --network testnet

# Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/subscription_tracker.wasm \
  --source wastecutter \
  --network testnet

# Output: Contract ID: CDIQ3YJBI3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W

Interact with Contract
bash

# Add a subscription
soroban contract invoke \
  --id CDIQ3YJBI3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W \
  --source wastecutter \
  --network testnet \
  -- \
  add_subscription \
  --user GC3VQK4B4M4P3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W \
  --merchant "Canva Pro" \
  --amount 11900 \
  --recurring_date 1743638400 \
  --escrow_amount 500

# Returns: 0 (subscription ID)

# Set cancellation reminder
soroban contract invoke \
  --id CDIQ3YJBI3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W \
  --source wastecutter \
  --network testnet \
  -- \
  set_cancellation_reminder \
  --user GC3VQK4B4M4P3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W \
  --subscription_id 0 \
  --reminder_timestamp 1743033600

# Returns: true

# Get user's subscriptions
soroban contract invoke \
  --id CDIQ3YJBI3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W \
  --source wastecutter \
  --network testnet \
  -- \
  get_user_subscriptions \
  --user GC3VQK4B4M4P3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W3Y5J3W

Demo Script (2 Minutes)
Minute 1: Setup

    Open WasteCutter dashboard

    Upload sample_statement.csv (provided in /demo)

    Watch AI detect: "Canva Pro - $119", "Slack Pro - $80", "Zoom Business - $20"

Minute 2: Action

    Click "Set Reminder" on Canva Pro

    System prompts: "Deposit 5 USDC as commitment"

    Click "Confirm Cancellation" (simulated)

    Dashboard shows: "✅ Refund of 5 USDC sent - you saved $119!"

ROI Calculation

For a Manila freelancer earning $500/month:
Scenario	Monthly Waste	Annual Waste	WasteCutter Cost (annual)	Savings
Without WasteCutter	$150 (3 subs)	$1,800	$0	-$1,800
With WasteCutter	$30 (1 sub missed)	$360	$60 ($5/mo)	+$1,380

Break-even: First forgotten subscription found pays for entire year of WasteCutter.
Roadmap
Phase 1 (Hackathon MVP) - ✅ Complete

    CSV upload + AI detection

    Soroban contract with escrow

    Basic dashboard UI

    5 test cases passing

Phase 2 (Post-Hackathon)

    Plaid API integration (real bank connections)

    Email + Telegram reminders

    Automated seat usage checks (Slack API, GitHub API)

    Price hike detection across 50+ SaaS tools

Phase 3 (Scale)

    Mobile app (React Native)

    Multi-currency support (PHP, IDR, VND via StableX)

    Team accounts for agencies (10+ seats)

    B2B white-label version

Business Model

Freemium Tiers:

    Free: Track 3 subscriptions, manual CSV upload

    Pro ($9/mo): Unlimited subs, Plaid integration, email reminders

    Agency ($49/mo): Team seats, API access, white-label reports

Success Fee (Optional): 10% of first year's savings (only if user saves >$100)

Localized pricing for SEA:

    Philippines: ₱499/mo (~$9)

    Indonesia: Rp 139,000/mo (~$9)

    Vietnam: ₫225,000/mo (~$9)

Team

    Smart Contract Lead: [Your Name] - Soroban expert, 3+ Stellar hackathon participant

    Frontend: [Name] - React/Next.js specialist

    AI/ML: [Name] - GPT-4 integration, CSV parsing

License

MIT - Free for open-source use. Commercial licenses available for agencies.
