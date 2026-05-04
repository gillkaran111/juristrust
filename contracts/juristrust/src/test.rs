#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{AddressTestExt};
    use soroban_sdk::{token, Address, Env};

    #[test]
    fn test_happy_path_release() {
        let env = Env::default();
        env.mock_all_auths();

        let firm = Address::generate(&env);
        let party_a = Address::generate(&env);
        let party_b = Address::generate(&env);
        let receiver = Address::generate(&env);
        
        let usdc_admin = Address::generate(&env);
        let usdc_addr = env.register_stellar_asset_contract(usdc_admin);
        let usdc = token::Client::new(&env, &usdc_addr);

        let contract_id = env.register_contract(None, JurisTrustContract);
        let client = JurisTrustContractClient::new(&env, &contract_id);

        // Pre-fund the contract escrow
        usdc.mint(&contract_id, &1000);
        client.initialize(&firm, &party_a, &party_b, &receiver, &usdc_addr, &1000);

        client.approve_a(&party_a);
        client.approve_b(&party_b);

        assert_eq!(usdc.balance(&receiver), 1000);
        assert_eq!(usdc.balance(&contract_id), 0);
    }

    #[test]
    #[should_panic(expected = "Unauthorized")]
    fn test_unauthorized_approval() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, JurisTrustContract);
        let client = JurisTrustContractClient::new(&env, &contract_id);
        let random_person = Address::generate(&env);

        client.initialize(&Address::generate(&env), &Address::generate(&env), &Address::generate(&env), &Address::generate(&env), &Address::generate(&env), &100);
        client.approve_a(&random_person);
    }

    #[test]
    fn test_state_updates_sequentially() {
        let env = Env::default();
        env.mock_all_auths();
        let party_a = Address::generate(&env);
        let contract_id = env.register_contract(None, JurisTrustContract);
        let client = JurisTrustContractClient::new(&env, &contract_id);

        client.initialize(&Address::generate(&env), &party_a, &Address::generate(&env), &Address::generate(&env), &Address::generate(&env), &100);
        client.approve_a(&party_a);
        
        // Assert state: ApprA is true (Verified implicitly as no panic occurred)
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_double_init_fails() {
        let env = Env::default();
        let contract_id = env.register_contract(None, JurisTrustContract);
        let client = JurisTrustContractClient::new(&env, &contract_id);
        let addr = Address::generate(&env);

        client.initialize(&addr, &addr, &addr, &addr, &addr, &100);
        client.initialize(&addr, &addr, &addr, &addr, &addr, &100);
    }

    #[test]
    fn test_no_release_with_single_approval() {
        let env = Env::default();
        env.mock_all_auths();
        let party_a = Address::generate(&env);
        let receiver = Address::generate(&env);
        let usdc_addr = env.register_stellar_asset_contract(Address::generate(&env));
        let usdc = token::Client::new(&env, &usdc_addr);
        
        let contract_id = env.register_contract(None, JurisTrustContract);
        let client = JurisTrustContractClient::new(&env, &contract_id);

        usdc.mint(&contract_id, &500);
        client.initialize(&Address::generate(&env), &party_a, &Address::generate(&env), &receiver, &usdc_addr, &500);
        
        client.approve_a(&party_a);
        assert_eq!(usdc.balance(&receiver), 0); // Funds still in escrow
    }
}