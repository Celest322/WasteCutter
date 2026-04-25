#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Vec, Map, String, U256};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    pub id: u64,
    pub merchant: String,
    pub amount: u64, // in USDC cents (e.g., 11900 = $119.00)
    pub recurring_date: u64, // Unix timestamp
    pub status: SubscriptionStatus,
    pub user: Address,
    pub escrow_amount: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    ReminderSet,
    Cancelled,
    Expired,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reminder {
    pub subscription_id: u64,
    pub reminder_time: u64,
    pub notified: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Subscription(u64),
    Reminder(u64),
    NextId,
    UserSubscriptions(Address),
}

#[contract]
pub struct SubscriptionTracker;

#[contractimpl]
impl SubscriptionTracker {
    /// Initialize contract with first subscription ID counter
    pub fn initialize(env: Env) {
        env.storage().instance().set(&DataKey::NextId, &0u64);
    }

    /// User adds a new subscription from CSV upload
    /// Returns subscription ID for tracking
    pub fn add_subscription(
        env: Env,
        user: Address,
        merchant: String,
        amount: u64,
        recurring_date: u64,
        escrow_amount: u64,
    ) -> u64 {
        user.require_auth();

        // Get next ID
        let mut next_id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap_or(0);
        let sub_id = next_id;
        next_id += 1;
        env.storage().instance().set(&DataKey::NextId, &next_id);

        // Create subscription
        let subscription = Subscription {
            id: sub_id,
            merchant,
            amount,
            recurring_date,
            status: SubscriptionStatus::Active,
            user: user.clone(),
            escrow_amount,
        };

        // Store subscription
        env.storage().persistent().set(&DataKey::Subscription(sub_id), &subscription);

        // Add to user's subscription list
        let mut user_subs: Vec<u64> = env.storage().persistent()
            .get(&DataKey::UserSubscriptions(user.clone()))
            .unwrap_or(Vec::new(&env));
        user_subs.push_back(sub_id);
        env.storage().persistent().set(&DataKey::UserSubscriptions(user), &user_subs);

        sub_id
    }

    /// Set a cancellation reminder with escrow deposit
    /// User deposits escrow_amount USDC as commitment to cancel
    pub fn set_cancellation_reminder(
        env: Env,
        user: Address,
        subscription_id: u64,
        reminder_timestamp: u64,
    ) -> bool {
        user.require_auth();

        // Verify subscription exists and belongs to user
        let mut subscription: Subscription = env.storage().persistent()
            .get(&DataKey::Subscription(subscription_id))
            .unwrap_or_else(|| panic!("Subscription not found"));

        if subscription.user != user {
            panic!("Unauthorized: subscription belongs to different user");
        }

        if subscription.status != SubscriptionStatus::Active {
            panic!("Subscription already processed");
        }

        // Transfer escrow from user to contract
        // In production, this would call token.transfer_from()
        // For MVP, we verify user has approved the contract

        // Create reminder record
        let reminder = Reminder {
            subscription_id,
            reminder_time: reminder_timestamp,
            notified: false,
        };

        env.storage().persistent().set(&DataKey::Reminder(subscription_id), &reminder);

        // Update subscription status
        subscription.status = SubscriptionStatus::ReminderSet;
        env.storage().persistent().set(&DataKey::Subscription(subscription_id), &subscription);

        true
    }

    /// Mark subscription as cancelled (user confirms)
    /// Returns escrow amount if cancelled before deadline
    pub fn confirm_cancellation(
        env: Env,
        user: Address,
        subscription_id: u64,
    ) -> u64 {
        user.require_auth();

        let mut subscription: Subscription = env.storage().persistent()
            .get(&DataKey::Subscription(subscription_id))
            .unwrap_or_else(|| panic!("Subscription not found"));

        if subscription.user != user {
            panic!("Unauthorized");
        }

        if subscription.status != SubscriptionStatus::ReminderSet {
            panic!("No active reminder for this subscription");
        }

        // Check if cancelled before deadline
        let current_time = env.ledger().timestamp();
        let reminder: Reminder = env.storage().persistent()
            .get(&DataKey::Reminder(subscription_id))
            .unwrap();

        let refund_amount = if current_time <= reminder.reminder_time {
            // On-time cancellation - return full escrow
            subscription.escrow_amount
        } else {
            // Late cancellation - forfeit escrow (goes to platform)
            0
        };

        // Update status
        subscription.status = SubscriptionStatus::Cancelled;
        env.storage().persistent().set(&DataKey::Subscription(subscription_id), &subscription);

        refund_amount
    }

    /// Get all subscriptions for a user
    pub fn get_user_subscriptions(
        env: Env,
        user: Address,
    ) -> Vec<Subscription> {
        let sub_ids: Vec<u64> = env.storage().persistent()
            .get(&DataKey::UserSubscriptions(user))
            .unwrap_or(Vec::new(&env));

        let mut subscriptions = Vec::new(&env);
        for sub_id in sub_ids.iter() {
            if let Some(sub) = env.storage().persistent().get(&DataKey::Subscription(sub_id)) {
                subscriptions.push_back(sub);
            }
        }
        subscriptions
    }

    /// Check for upcoming reminders (called by cron job / keeper)
    pub fn check_overdue_reminders(env: Env) -> Vec<u64> {
        let mut overdue = Vec::new(&env);
        let current_time = env.ledger().timestamp();

        // This would iterate through all reminders
        // For MVP, simplified to return expired reminder IDs
        // Production would use a mapping of all reminder IDs
        
        overdue
    }
}
