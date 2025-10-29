#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Structure to store product information at each step
#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub product_id: u64,
    pub name: String,
    pub manufacturer: String,
    pub current_location: String,
    pub status: String,
    pub timestamp: u64,
}

// Structure to track supply chain step
#[contracttype]
#[derive(Clone)]
pub struct SupplyStep {
    pub step_id: u64,
    pub product_id: u64,
    pub location: String,
    pub handler: String,
    pub timestamp: u64,
    pub notes: String,
}

// Mapping product_id to Product
#[contracttype]
pub enum ProductBook {
    Product(u64)
}

// Mapping step_id to SupplyStep
#[contracttype]
pub enum StepBook {
    Step(u64)
}

// Counter for product IDs
const PRODUCT_COUNT: Symbol = symbol_short!("P_COUNT");
// Counter for step IDs
const STEP_COUNT: Symbol = symbol_short!("S_COUNT");

#[contract]
pub struct SupplyChainContract;

#[contractimpl]
impl SupplyChainContract {
    
    // Function to register a new product in the supply chain
    pub fn register_product(env: Env, name: String, manufacturer: String, location: String) -> u64 {
        let mut product_count: u64 = env.storage().instance().get(&PRODUCT_COUNT).unwrap_or(0);
        product_count += 1;
        
        let timestamp = env.ledger().timestamp();
        
        let product = Product {
            product_id: product_count.clone(),
            name: name.clone(),
            manufacturer: manufacturer.clone(),
            current_location: location.clone(),
            status: String::from_str(&env, "Registered"),
            timestamp,
        };
        
        // Store the product
        env.storage().instance().set(&ProductBook::Product(product_count.clone()), &product);
        env.storage().instance().set(&PRODUCT_COUNT, &product_count);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Product Registered with ID: {}", product_count);
        product_count
    }
    
    // Function to add a supply chain step (movement/transfer)
    pub fn add_supply_step(env: Env, product_id: u64, location: String, handler: String, notes: String) -> u64 {
        let mut step_count: u64 = env.storage().instance().get(&STEP_COUNT).unwrap_or(0);
        step_count += 1;
        
        let timestamp = env.ledger().timestamp();
        
        // Create supply step record
        let step = SupplyStep {
            step_id: step_count.clone(),
            product_id: product_id.clone(),
            location: location.clone(),
            handler: handler.clone(),
            timestamp,
            notes: notes.clone(),
        };
        
        // Update product's current location and status
        let mut product = Self::view_product(env.clone(), product_id.clone());
        if product.product_id != 0 {
            product.current_location = location.clone();
            product.status = String::from_str(&env, "In Transit");
            product.timestamp = timestamp;
            
            env.storage().instance().set(&ProductBook::Product(product_id.clone()), &product);
        }
        
        // Store the supply step
        env.storage().instance().set(&StepBook::Step(step_count.clone()), &step);
        env.storage().instance().set(&STEP_COUNT, &step_count);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Supply Step Added with ID: {}", step_count);
        step_count
    }
    
    // Function to mark product as delivered to consumer
    pub fn mark_delivered(env: Env, product_id: u64, consumer_location: String) {
        let mut product = Self::view_product(env.clone(), product_id.clone());
        
        if product.product_id != 0 {
            let timestamp = env.ledger().timestamp();
            
            product.current_location = consumer_location;
            product.status = String::from_str(&env, "Delivered");
            product.timestamp = timestamp;
            
            env.storage().instance().set(&ProductBook::Product(product_id.clone()), &product);
            env.storage().instance().extend_ttl(5000, 5000);
            
            log!(&env, "Product ID: {} marked as Delivered", product_id);
        } else {
            log!(&env, "Product not found!");
            panic!("Product not found!");
        }
    }
    
    // Function to view product details by product_id
    pub fn view_product(env: Env, product_id: u64) -> Product {
        let key = ProductBook::Product(product_id.clone());
        
        env.storage().instance().get(&key).unwrap_or(Product {
            product_id: 0,
            name: String::from_str(&env, "Not_Found"),
            manufacturer: String::from_str(&env, "Not_Found"),
            current_location: String::from_str(&env, "Not_Found"),
            status: String::from_str(&env, "Not_Found"),
            timestamp: 0,
        })
    }
}