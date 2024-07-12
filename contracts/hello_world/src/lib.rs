#![no_std]
use core::ops::Add;

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,Map};
use soroban_sdk::token::Client as TokenClient;

#[contracttype]
#[derive(Clone)]
pub struct Farm {
    pub id: u128,
    pub metadata: String,
    pub cap_req: u128,
    pub cap_rai: u128,
    pub exp_pft: u128,
    pub investors: Vec<Address>,
    pub amounts: Vec<u128>,
}

#[contracttype]
pub enum FarmFactory {
    Farm(u128),
}

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct BlockLand;

#[contractimpl]
impl BlockLand {
    pub fn register_farm(env: Env, _metadata: String, _cap_req: u128, _exp_pft: u128) {
        let mut count: u128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count += 1;

        let farm = Farm {
            id: count,
            metadata: _metadata.clone(),
            cap_req: _cap_req,
            cap_rai: 0,
            exp_pft: _exp_pft,
            investors: Vec::new(&env),
            amounts: Vec::new(&env),
        };

        env.storage().instance().set(&COUNTER, &count);
        env.storage().persistent().set(&FarmFactory::Farm(count), &farm);
    }

    pub fn get_farm(env: Env, id: u128) -> Farm {
        Self::view_farm(&env, id)
    }


    pub fn distribute_profit(env: Env, farm_id: u128, profit: u128, asset: Address) {
        let farm = Self::view_farm(&env, farm_id);
        let total_raised = farm.cap_rai;

        for (i, investor) in farm.investors.iter().enumerate() {
            let amount_invested = farm.amounts.get(i as u32).unwrap_or(0);
            let investor_share = (amount_invested as u128 * 98 / total_raised) * profit / 100;

            TokenClient::new(&env, &asset).transfer(&env.current_contract_address(), &investor, &(investor_share as i128));
        }
    }

    pub fn add_capital(env: Env, farm_id: u128, investor: Address, amount: i128, asset: Address) {
        investor.require_auth();

        TokenClient::new(&env, &asset).transfer(&investor, &env.current_contract_address(), &amount);

        let mut farm = Self::view_farm(&env, farm_id);
        farm.investors.push_back(investor.clone());
        farm.amounts.push_back(amount as u128);
        farm.cap_rai += amount as u128 ;

        env.storage().persistent().set(&FarmFactory::Farm(farm_id), &farm);
    }

    pub fn get_all_farms(env: Env) -> Vec<Farm> {
        let mut farms = Vec::new(&env);
        let count: u128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        for id in 1..=count {
            let farm = Self::view_farm(&env, id);
            if farm.id != 0 { 
                farms.push_back(farm);
            }
        }
        farms
    }

    pub fn get_investments(env: Env, user: Address) -> Map<u128, u128> {
        let mut investments = Map::new(&env);
        let count: u128 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        
        for id in 1..=count {
            let farm = Self::view_farm(&env, id);
            for i in 0..farm.investors.len() {
                if farm.investors.get(i).unwrap() == user {
                    let invested_amount = farm.amounts.get(i).unwrap();
                    if let Some(current_amount) = investments.get(id) {
                        investments.set(id, current_amount + invested_amount);
                    } else {
                        investments.set(id, invested_amount);
                    }
                }
            }
        }
    
        investments
    }

    fn view_farm(env: &Env, id: u128) -> Farm {
        let key = FarmFactory::Farm(id);
        env.storage().persistent().get(&key).unwrap_or(Farm {
            id: 0,
            metadata: String::from_str(env, "0x00"),
            cap_req: 0,
            cap_rai: 0,
            exp_pft: 0,
            investors: Vec::new(env),
            amounts: Vec::new(env),
        })
    }
}

mod test;