#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, token};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::StellarAssetClient as TokenClient;

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    // Регистрируем контракт ТОКЕНА (имитируем USDC или XLM)
    let admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract_v2(admin.clone());
    let token_admin = TokenClient::new(&env, &token_address.address());
    let token_user_client = token::Client::new(&env, &token_address.address());

    let admin = Address::generate(&env);

    let contract_id = env.register(KingOfMountain, ());
    let client = KingOfMountainClient::new(&env, &contract_id);
    client.init(&admin, &token_address.address());

    let admin_saved = client.get_admin();
    assert_eq!(admin_saved, admin);

    let amount: i128 = 100;

    let user = Address::generate(&env);

    // Проверяем возврат текущего сообщения контрактом без данных
    let current_message = client.message();
    assert_eq!(current_message, String::from_str(&env, "--- No message yet ---"));

    // Даем пользователю токены
    token_admin.mint(&user, &10000i128);

    // Тест1: Пользователь пытается захватить гору (удачно), одобряя контракту 100 токенов и отправляя сообщение "Hello! I am KING!!!"
    token_user_client.approve(&user, &contract_id, &amount, &1000);
    let msg1 = String::from_str(&env, "Hello! I am KING!!!");

    let try1 = client.capture(
        &user,
        &token_address.address(),
        &amount,
        &msg1,
    );

    assert!(try1);
    assert_eq!(token_user_client.balance(&contract_id), amount);

    let current_message = client.message();
    assert_eq!(current_message, msg1);

    // Тест2: Пользователь пытается захватить гору (неудачно), одобряя контракту 10 токенов и отправляя сообщение "Hello! I am SECOND KING!!!"
    let amount_low = 10;
    token_user_client.approve(&user, &contract_id, &amount_low, &1000);
    let msg2 = String::from_str(&env, "Hello! I am SECOND KING!!!");

    let try2 = client.capture(
        &user,
        &token_address.address(),
        &amount_low,
        &msg2,
    );
    assert!(!try2);
    assert_eq!(token_user_client.balance(&contract_id), amount);

    let current_message = client.message();
    assert_eq!(current_message, msg1);

    // Тест3: Пользователь пытается захватить гору (удачно), одобряя контракту 1000 токенов и отправляя сообщение "Hello! I am THIRD KING!!!"
    let amount_high = 1000;
    token_user_client.approve(&user, &contract_id, &amount_high, &1000);
    let msg3 = String::from_str(&env, "Hello! I am THIRD KING!!!");

    let try3 = client.capture(
        &user,
        &token_address.address(),
        &amount_high,
        &msg3,
    );
    assert!(try3);
    assert_eq!(token_user_client.balance(&contract_id), amount+amount_high);

    let current_message = client.message();
    assert_eq!(current_message, msg3);

    // Тест4: Перевод баланса контракта админу
    let contract_balance = token_user_client.balance(&contract_id);
    client.withdraw();
    assert_eq!(token_user_client.balance(&contract_id), 0);
    assert_eq!(token_user_client.balance(&admin), contract_balance);
}
