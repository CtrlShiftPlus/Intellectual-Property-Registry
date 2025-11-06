#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, String, log};

// Define the structure for an Intellectual Property record.
#[contracttype]
#[derive(Clone)]
pub struct IPRecord {
    pub unique_id: u64,
    pub title: String,
    pub description: String,
}

// Define the smart contract
#[contract]
pub struct IPRegistryContract;

#[contractimpl]
impl IPRegistryContract {
    // Store the next available ID for a new IP Record
    const NEXT_ID: Symbol = symbol!("next_id");
    
    // Function to create a new IP Record
    pub fn create_ip_record(env: Env, title: String, description: String) -> u64 {
        let mut id: u64 = env.storage().instance().get(&Self::NEXT_ID).unwrap_or(1);
        let ip_record = IPRecord {
            unique_id: id,
            title,
            description,
        };
        
        // Store the record with the generated unique ID
        env.storage().instance().set(&id, &ip_record);
        
        // Increment the ID counter for the next record
        id += 1;
        env.storage().instance().set(&Self::NEXT_ID, &id);
        
        log!(&env, "Created IP Record with ID: {}", id);
        id // Return the unique ID
    }

    // Function to retrieve an IP Record by its unique ID
    pub fn get_ip_record(env: Env, unique_id: u64) -> IPRecord {
        env.storage().instance().get(&unique_id).unwrap_or(IPRecord {
            unique_id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
        })
    }

    // Function to update an IP Record's description
    pub fn update_ip_record(env: Env, unique_id: u64, new_description: String) {
        let mut record = env.storage().instance().get(&unique_id).unwrap_or(IPRecord {
            unique_id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
        });
        
        // If the record exists, update the description
        if record.unique_id != 0 {
            record.description = new_description;
            env.storage().instance().set(&unique_id, &record);
            log!(&env, "Updated IP Record ID: {} with new description", unique_id);
        } else {
            log!(&env, "IP Record ID: {} not found", unique_id);
            panic!("IP Record not found!");
        }
    }

    // Function to delete an IP Record by its unique ID
    pub fn delete_ip_record(env: Env, unique_id: u64) {
        let record = env.storage().instance().get(&unique_id);
        
        if record.is_some() {
            env.storage().instance().remove(&unique_id);
            log!(&env, "Deleted IP Record ID: {}", unique_id);
        } else {
            log!(&env, "IP Record ID: {} not found", unique_id);
            panic!("IP Record not found!");
        }
    }
}
