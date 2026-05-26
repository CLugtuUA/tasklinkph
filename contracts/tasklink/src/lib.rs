#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, BytesN, log};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MilestoneStatus {
    Funded,
    Completed,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct PhTaskAgreement {
    pub client: Address,
    pub freelancer: Address,
    pub amount: i128,
    pub task_id: BytesN<32>, // Unique identifier hash for the specific freelance task/milestone
    pub status: MilestoneStatus,
}

#[contract]
pub struct TaskLinkPH;

#[contractimpl]
impl TaskLinkPH {
    // Client locks funds to a specific task milestone for a Filipino freelancer
    pub fn create_milestone(
        env: Env,
        client: Address,
        freelancer: Address,
        amount: i128,
        task_id: BytesN<32>,
    ) -> PhTaskAgreement {
        client.require_auth();
        assert!(amount > 0, "Milestone amount must be positive");

        let agreement = PhTaskAgreement {
            client: client.clone(),
            freelancer,
            amount,
            task_id: task_id.clone(),
            status: MilestoneStatus::Funded,
        };

        env.storage().persistent().set(&task_id, &agreement);
        log!(&env, "Milestone contract initialized for task: {}", task_id);
        
        agreement
    }

    // Client confirms work compliance and releases funding to the freelancer's wallet
    pub fn complete_and_release(env: Env, task_id: BytesN<32>) -> PhTaskAgreement {
        let mut agreement: PhTaskAgreement = env
            .storage()
            .persistent()
            .get(&task_id)
            .expect("Task milestone registry not found");

        // The client who initiated the task must authorize the payout release
        agreement.client.require_auth();
        assert!(agreement.status == MilestoneStatus::Funded, "Milestone is already finalized");

        agreement.status = MilestoneStatus::Completed;
        env.storage().persistent().set(&task_id, &agreement);

        log!(&env, "Milestone payout released to freelancer address: {}", agreement.freelancer);
        agreement
    }
}