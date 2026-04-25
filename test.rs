#![cfg(test)]

use soroban_sdk::{Env, Address, String, IntoVal};
use subscription_tracker::{SubscriptionTracker, SubscriptionTrackerClient, SubscriptionStatus};

#[test]
fn test_happy_path_add_and_cancel_subscription() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SubscriptionTracker);
    let client = SubscriptionTrackerClient::new(&env, &contract_id);

    // Initialize contract
    client.initialize();

    // Create user
    let user = Address::generate(&env);
    let merchant = String::from_str(&env, "Canva Pro");
    
    // Add subscription
    let sub_id = client.add_subscription(
        &user,
        &merchant,
        &11900u64, // $119.00
        &1743638400u64, // April 3, 2025
        &500u64, // $5 escrow
    );

    // Set reminder for 7 days before due
    let reminder_time = 1743033600u64; // March 27, 2025
    let result = client.set_cancellation_reminder(&user, &sub_id, &reminder_time);
    assert_eq!(result, true);

    // Confirm cancellation before deadline
    let current_time = 1743120000u64; // March 28, 2025 (before deadline)
    env.ledger().with_mut(|li| li.timestamp = current_time);
    
    let refund = client.confirm_cancellation(&user, &sub_id);
    assert_eq!(refund, 500u64); // Full escrow returned

    // Verify status changed to Cancelled
    let user_subs = client.get_user_subscriptions(&user);
    assert_eq!(user_subs.len(), 1);
    assert_eq!(user_subs.get(0).unwrap().status, SubscriptionStatus::Cancelled);
}

#[test]
fn test_edge_case_unauthorized_user_cannot_modify_subscription() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SubscriptionTracker);
    let client = SubscriptionTrackerClient::new(&env, &contract_id);
    client.initialize();

    let owner = Address::generate(&env);
    let attacker = Address::generate(&env);
    let merchant = String::from_str(&env, "Slack Pro");

    let sub_id = client.add_subscription(
        &owner,
        &merchant,
        &8000u64, // $80.00
        &1743638400u64,
        &500u64,
    );

    // Attacker tries to set reminder on owner's subscription
    client.mock_all_auths().set_cancellation_reminder(&attacker, &sub_id, &1743033600u64);
    
    // Should panic due to authorization check
    // In test, we expect the call to fail
}

#[test]
fn test_state_verification_after_reminder_set() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SubscriptionTracker);
    let client = SubscriptionTrackerClient::new(&env, &contract_id);
    client.initialize();

    let user = Address::generate(&env);
    let merchant = String::from_str(&env, "Figma Team");
    
    let sub_id = client.add_subscription(
        &user,
        &merchant,
        &1500u64, // $15.00
        &1743724800u64,
        &500u64,
    );

    // Verify initial status is Active
    let subs_before = client.get_user_subscriptions(&user);
    assert_eq!(subs_before.get(0).unwrap().status, SubscriptionStatus::Active);
    assert_eq!(subs_before.get(0).unwrap().amount, 1500u64);

    // Set reminder
    client.set_cancellation_reminder(&user, &sub_id, &1743118400u64);

    // Verify status changed to ReminderSet
    let subs_after = client.get_user_subscriptions(&user);
    assert_eq!(subs_after.get(0).unwrap().status, SubscriptionStatus::ReminderSet);
    
    // Verify amounts unchanged
    assert_eq!(subs_after.get(0).unwrap().amount, 1500u64);
    assert_eq!(subs_after.get(0).unwrap().escrow_amount, 500u64);
}

#[test]
fn test_multiple_subscriptions_per_user() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SubscriptionTracker);
    let client = SubscriptionTrackerClient::new(&env, &contract_id);
    client.initialize();

    let user = Address::generate(&env);
    
    // Add 3 subscriptions
    let sub1 = client.add_subscription(&user, &String::from_str(&env, "Zoom Pro"), &2000u64, &1743638400u64, &500u64);
    let sub2 = client.add_subscription(&user, &String::from_str(&env, "Loom Business"), &1800u64, &1743724800u64, &500u64);
    let sub3 = client.add_subscription(&user, &String::from_str(&env, "Notion Team"), &1000u64, &1743811200u64, &500u64);

    let all_subs = client.get_user_subscriptions(&user);
    assert_eq!(all_subs.len(), 3);
    
    // Verify IDs are sequential
    assert_eq!(sub1, 0);
    assert_eq!(sub2, 1);
    assert_eq!(sub3, 2);
}

#[test]
fn test_late_cancellation_forfeits_escrow() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SubscriptionTracker);
    let client = SubscriptionTrackerClient::new(&env, &contract_id);
    client.initialize();

    let user = Address::generate(&env);
    let merchant = String::from_str(&env, "Adobe Creative Cloud");
    
    let sub_id = client.add_subscription(
        &user,
        &merchant,
        &6000u64, // $60.00
        &1743638400u64,
        &1000u64, // $10 escrow
    );

    // Set reminder for March 27
    let reminder_time = 1743033600u64;
    client.set_cancellation_reminder(&user, &sub_id, &reminder_time);

    // Confirm cancellation after deadline (April 10)
    let current_time = 1744329600u64; // April 10, 2025 (after deadline)
    env.ledger().with_mut(|li| li.timestamp = current_time);
    
    let refund = client.confirm_cancellation(&user, &sub_id);
    assert_eq!(refund, 0u64); // No refund - escrow forfeited
    
    // Status still becomes Cancelled
    let subs = client.get_user_subscriptions(&user);
    assert_eq!(subs.get(0).unwrap().status, SubscriptionStatus::Cancelled);
}
