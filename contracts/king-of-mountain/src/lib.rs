#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Env, String, Address};
use crate::StorageDataKey::{AdminAddress, KingMessage, LastKingAmount, TokenAddress};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserEntry {
    pub user: Address,
    pub message: String,
}

#[contracttype]
pub enum StorageDataKey {
    KingMessage,
    LastKingAmount,
    AdminAddress,
    TokenAddress,
}

#[contract]
pub struct KingOfMountain;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.
#[contractimpl]
impl KingOfMountain {
    pub fn capture(env: Env, user: Address, token_address: Address, amount: i128, msg: String) -> bool {
        // Проверяем разрешен-ли текущий токен
        if !Self::is_token_enabled(env.clone(), &token_address) {
            panic!("Token not enabled for this contract");
        }

        // Сначала сравниваем переданное количество токенов с последним захватом. Если оно меньше или равно, то не пропускаем.
        let key = LastKingAmount;
        let last_amount = env.storage().persistent().get(&key).unwrap_or(0);
        if amount <= last_amount {
            return false;
        }

        user.require_auth();

        // Если больше - переводим токены от пользователя контракту
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer_from(
            &env.current_contract_address(), // Кто инициирует (spender)
            &user,                           // У кого забираем (from)
            &env.current_contract_address(), // Кому отдаем (to)
            &amount                          // Сколько
        );

        // Сохраняем последнюю сумму
        env.storage().persistent().set(&key, &amount);

        // Сохраняем сообщение о захвате в хранилище
        let key = KingMessage;
        let message = UserEntry {
            user: user.clone(),
            message: msg.clone(),
        };
        env.storage().persistent().set(&key, &message);

        true
    }

    pub fn message(env: Env) -> String {
        let key = KingMessage;
        let message = env.storage().persistent().get(&key).unwrap_or(UserEntry {
            // Получаем адрес текущего контракта
            user: env.current_contract_address(),
            message: String::from_str(&env, "--- No message yet ---"),
        });
        message.message
    }

    // Вызываем один раз при деплое
    pub fn init(env: Env, admin: Address, token_address: Address) {
        admin.require_auth();

        // Проверяем, есть ли уже админ в хранилище
        if env.storage().instance().has(&AdminAddress) {
            panic!("Contract already initialized");
        }

        env.storage().instance().set(&AdminAddress, &admin);
        env.storage().instance().set(&TokenAddress, &token_address);
    }

    pub fn get_admin(env: Env) -> Address {
        let admin: Address = env.storage().instance()
            .get(&AdminAddress)
            .expect("Contract not initialized");

        admin
    }

    pub fn withdraw(env: Env) {
        // 0. Извлекаем админа из хранилища
        let admin: Address = env.storage().instance()
            .get(&AdminAddress)
            .expect("Contract not initialized");

        let token_enabled: Address = env.storage().instance()
            .get(&TokenAddress)
            .expect("Contract not initialized");

        // 1. Опционально: проверить, кто имеет право выводить средства (например, админ)
        admin.require_auth();

        // 2. Создаем клиент токена
        let token_client = token::Client::new(&env, &token_enabled);
        let balance = token_client.balance(&env.current_contract_address());

        // 3. Вызываем метод transfer
        // Отправитель — адрес текущего контракта
        token_client.transfer(&env.current_contract_address(), &admin, &balance);
    }

    fn is_token_enabled(env: Env, token_address: &Address) -> bool {
        let token_enabled: Address = env.storage().instance()
            .get(&TokenAddress)
            .expect("Contract not initialized");

        token_enabled == *token_address
    }
}

mod test;
