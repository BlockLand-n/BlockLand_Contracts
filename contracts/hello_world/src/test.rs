#![cfg(test)]



use super::*;
use soroban_sdk::{testutils::Address, Address as TestAddr, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BlockLand);
    let client = BlockLandClient::new(&env, &contract_id);

    let metadata = String::from_str(&env,"0x00");
    let cap_req: u128 = 1000;
    let exp_pft: u128 = 2000;

    client.create_farm(&metadata, &cap_req, &exp_pft);

    let created_farm = client.get_farm(&1);

    assert_eq!(created_farm.id,1);
    assert_eq!(created_farm.metadata,metadata);
    assert_eq!(created_farm.cap_req,cap_req);
    assert_eq!(created_farm.exp_pft,exp_pft);
    assert_eq!(created_farm.cap_rai,0);
    assert!(created_farm.investors.is_empty());
    assert!(created_farm.amounts.is_empty());



    // Prepare to add capital
    let farm_id: u128 = 1;
    let investor = <soroban_sdk::Address as Address>::generate(&env);
    let amount: i128 = 500;
    let r:u64 = 50;
    let asset  = TestAddr::from_string(&String::from_str(&env, "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA"));

    // Add capital
    client.add_capital(&farm_id, &investor, &amount, &asset);

    // Verify the farm after adding capital
    let updated_farm = client.get_farm(&farm_id);

    assert_eq!(updated_farm.cap_rai, amount as u128);
    assert_eq!(updated_farm.investors.len(), 1);
    assert_eq!(updated_farm.amounts.len(), 1);
    assert_eq!(updated_farm.investors.get(0).unwrap(), investor);
    assert_eq!(updated_farm.amounts.get(0).unwrap(), amount as u128);

}
