#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Env, String, Address, BytesN};
use crate::StorageDataKey::{AdminAddress, KingMessage, LastKingAmount, TokenAddress, Version};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserEntry {
    pub user: Address,
    pub message: String,
}

#[contracttype]
pub enum StorageDataKey {
    Version,
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
    pub fn __constructor(env: Env, admin: Address, token_address: Address) {
        env.storage().instance().set(&AdminAddress, &admin);
        env.storage().instance().set(&TokenAddress, &token_address);
    }

    pub fn version() -> u32 {
        2
    }

    /// Функция для обновления кода контракта
    pub fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        // new_wasm_hash — это SHA-256 хеш нового Wasm-файла, уже загруженного в сеть
        // stellar contract upload --wasm path/to/new_contract.wasm --source-account admin --network testnet
        // stellar contract invoke --id <CONTRACT_ID> --source-account admin --network testnet -- upgrade --new_wasm_hash <NEW_WASM_HASH>

        // 1. Получаем адрес администратора из хранилища
        let admin: Address = env.storage().instance().get(&AdminAddress).unwrap();

        // 2. Проверяем подпись администратора (обязательно!)
        admin.require_auth();

        // 3. Вызываем системную функцию для замены Wasm-кода по текущему адресу
        env.deployer().update_current_contract_wasm(new_wasm_hash);

        // 4. Вызываем метод миграции уже нового контракта
        // С этого момента выполняется код НОВОГО WASM.
        // Мы вызываем внутреннюю функцию нового контракта.
        Self::migrate(env);
    }

    /// Функция для миграции данных при обновлении контракта. Вызывается внутри функции upgrade уже после замены кода.
    pub fn migrate(env: Env) {
        // 1. Получаем адрес администратора из хранилища
        let admin: Address = env.storage().instance().get(&AdminAddress).unwrap();

        // 2. Проверяем подпись администратора (обязательно!)
        admin.require_auth();

        // 3. Обновляем версию контракта в хранилище. Это позволит нам отслеживать, что контракт был успешно обновлен.
        let old_version: u32 = env.storage().instance().get(&Version).unwrap_or(0);
        env.storage().instance().set(&Version, &Self::version());

        // 4. Обновляем данные контракта если нужно
        if old_version != Self::version() {
            // Здесь можно добавить логику для обновления данных контракта, например,
            // перенос данных из старой версии в новую, если структура данных изменилась.
            // В нашем случае структура данных не меняется, поэтому просто оставляем это место для примера.
        }
    }

    /// Функция для захвата горы. Пользователь должен отправить определенное количество токенов, чтобы стать новым королем.
    pub fn capture(env: Env, user: Address, amount: i128, msg: String) -> bool {
        user.require_auth();

        let token_address: Address = env.storage().instance()
            .get(&TokenAddress)
            .expect("Contract not initialized");

        // Сначала сравниваем переданное количество токенов с последним захватом. Если оно меньше или равно, то не пропускаем.
        let key = LastKingAmount;
        let last_amount = env.storage().persistent().get(&key).unwrap_or(0);
        if amount <= last_amount {
            return false;
        }

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

        // Сохраняем сообщение нового короля в хранилище
        let key = KingMessage;
        let message = UserEntry {
            user: user.clone(),
            message: msg.clone(),
        };
        env.storage().persistent().set(&key, &message);

        true
    }

    /// Получение сообщения короля
    pub fn message(env: Env) -> String {
        let key = KingMessage;
        let message = env.storage().persistent().get(&key).unwrap_or(UserEntry {
            // Получаем адрес текущего контракта
            user: env.current_contract_address(),
            message: String::from_str(&env, "--- No message yet ---"),
        });
        message.message
    }

    /// Получение адреса администратора
    pub fn get_admin(env: Env) -> Address {
        let admin: Address = env.storage().instance()
            .get(&AdminAddress)
            .expect("Contract not initialized");

        admin
    }

    /// Вывод средств администратору (вызов разрешен только администратору)
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
}

mod test;
