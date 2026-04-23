//! Payment processing implementation.
use soroban_sdk::{Address, Env, String};

use crate::errors::PaymentError;
use crate::storage::DataKey;
use crate::types::{AgreementStatus, EscalationType, PaymentRecord, RentAgreement, RentEscalationConfig};

/// Calculate the rent amount for a specific period (payment number) with escalation
pub fn calculate_rent_for_period(
    base_rent: i128,
    payment_number: u32,
    config: &RentEscalationConfig,
) -> i128 {
    match config.escalation_type {
        EscalationType::None => base_rent,
        EscalationType::FixedAnnual => {
            if config.payments_per_year == 0 {
                return base_rent;
            }

            // Calculate how many years have passed since the first payment
            // payment_number is 1-indexed (1st payment, 2nd payment, etc.)
            let years_passed = (payment_number - 1) / config.payments_per_year;

            if years_passed == 0 {
                return base_rent;
            }

            // Calculate escalated rent: Rent = BaseRent * (1 + rate)^years
            let mut current_rent = base_rent;
            for _ in 0..years_passed {
                // annual_rate_bps is in basis points (1 bps = 0.01%)
                let increase = (current_rent * (config.annual_rate_bps as i128)) / 10000;
                current_rent += increase;
            }
            current_rent
        }
    }
}

/// Create an immutable payment record
pub fn create_payment_record(
    _env: &Env,
    agreement_id: &String,
    amount: i128,
    landlord_amount: i128,
    agent_amount: i128,
    tenant: &Address,
    payment_number: u32,
    timestamp: u64,
) -> Result<PaymentRecord, PaymentError> {
    Ok(PaymentRecord {
        agreement_id: agreement_id.clone(),
        payment_number,
        amount,
        landlord_amount,
        agent_amount,
        timestamp,
        tenant: tenant.clone(),
    })
}

/// Calculate payment split between landlord and agent
pub fn calculate_payment_split(amount: &i128, commission_rate: &u32) -> (i128, i128) {
    // commission_rate is in basis points (1 basis point = 0.01%)
    let agent_amount = (amount * (*commission_rate as i128)) / 10000;
    let landlord_amount = amount - agent_amount;
    (landlord_amount, agent_amount)
}

/// Process rent payment with automatic commission splitting
/// This is the alternate implementation used by RentalContract
#[allow(deprecated)]
#[allow(dead_code)]
pub fn pay_rent_with_agent(
    env: Env,
    agreement_id: String,
    token: Address,
    amount: i128,
) -> Result<(), PaymentError> {
    use soroban_sdk::token::Client as TokenClient;

    // Load agreement
    let mut agreement: RentAgreement = env
        .storage()
        .persistent()
        .get(&DataKey::Agreement(agreement_id.clone()))
        .ok_or(PaymentError::InvalidAmount)?;

    // Validate agreement is active
    if agreement.status != AgreementStatus::Active {
        return Err(PaymentError::AgreementNotActive);
    }

    // Validate amount is strictly positive to prevent logical errors
    if amount <= 0 {
        return Err(PaymentError::InvalidAmount);
    }

    // Validate amount matches monthly rent exactly
    if amount != agreement.monthly_rent {
        return Err(PaymentError::InvalidAmount);
    }

    // Authorize tenant
    agreement.tenant.require_auth();

    // Calculate payment split
    let (landlord_amount, agent_amount) =
        calculate_payment_split(&amount, &agreement.agent_commission_rate);

    // Execute atomic token transfers
    let token_client = TokenClient::new(&env, &token);

    // Transfer to landlord
    token_client.transfer(&agreement.tenant, &agreement.landlord, &landlord_amount);

    // Transfer to agent if present
    if let Some(agent_address) = &agreement.agent {
        if agent_amount > 0 {
            token_client.transfer(&agreement.tenant, agent_address, &agent_amount);
        }
    }

    // Create payment record
    let timestamp = env.ledger().timestamp();
    let payment_record = create_payment_record(
        &env,
        &agreement_id,
        amount,
        landlord_amount,
        agent_amount,
        &agreement.tenant,
        agreement.payment_count + 1,
        timestamp,
    )?;

    // Update agreement totals
    agreement.total_rent_paid += amount;
    agreement.payment_count += 1;

    // Persist updated agreement
    env.storage()
        .persistent()
        .set(&DataKey::Agreement(agreement_id.clone()), &agreement);

    // Persist payment record
    env.storage().persistent().set(
        &DataKey::PaymentRecord(agreement_id.clone(), agreement.payment_count),
        &payment_record,
    );

    // Emit event
    env.events().publish(
        (String::from_str(&env, "rent_paid"), agreement_id),
        (amount, landlord_amount, agent_amount, timestamp),
    );

    Ok(())
}
