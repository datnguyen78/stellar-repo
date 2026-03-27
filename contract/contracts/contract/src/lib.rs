#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, symbol_short};

const ADMIN: Symbol = symbol_short!("ADMIN");

#[contract]
pub struct BloodCertificateContract;

#[contractimpl]
impl BloodCertificateContract {
    // 1. Khởi tạo: Đặt địa chỉ bệnh viện làm Admin
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) { panic!("Already initialized"); }
        env.storage().instance().set(&ADMIN, &admin);
    }

    // 2. Cấp chứng chỉ: Lưu link IPFS vào địa chỉ ví người hiến
    pub fn issue(env: Env, admin: Address, donor: Address, cert_uri: String) {
        admin.require_auth(); // Xác thực đúng là bệnh viện đang gọi hàm này
        
        let stored_admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if admin != stored_admin { panic!("Not authorized"); }

        // Lưu thông tin: Key là địa chỉ người hiến, Value là link chứng chỉ
        env.storage().persistent().set(&donor, &cert_uri);
    }

    // 3. Tra cứu: Ai cũng có thể kiểm tra chứng chỉ của một ví
    pub fn get_cert(env: Env, donor: Address) -> String {
        env.storage().persistent().get(&donor).unwrap_or(String::from_str(&env, "No certificate found"))
    }
}