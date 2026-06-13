#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

// Khung dữ liệu chứa thông tin tài sản
#[contracttype]
pub struct AssetMetadata {
    pub title: String,
    pub asset_type: String, // Loại tài sản: "Book", "Music", "Art", v.v.
    pub file_hash: String,  // Dấu vân tay kỹ thuật số SHA-256
    pub token_uri: String,  // Đường dẫn chứa thông tin chi tiết trên máy chủ của bạn
}

#[contract]
pub struct YourCopyrightContract;

#[contractimpl]
impl YourCopyrightContract {
    // 1. Hàm Cấp Bản Quyền (Mint)
    pub fn register_asset(env: Env, creator: Address, asset_id: u32, metadata: AssetMetadata) {
        // Yêu cầu chính chủ ký xác nhận
        creator.require_auth();

        // Chống ghi đè: Đảm bảo ID chưa từng tồn tại
        if env.storage().persistent().has(&asset_id) {
            panic!("Tài sản với ID này đã tồn tại trên hệ thống!");
        }

        // Lưu thông tin chi tiết & Gán quyền sở hữu
        env.storage().persistent().set(&asset_id, &metadata);
        env.storage().persistent().set(&("owner", asset_id), &creator);
    }

    // 2. Hàm Tra cứu Thông tin Tài sản
    pub fn get_asset_info(env: Env, asset_id: u32) -> AssetMetadata {
        env.storage().persistent().get(&asset_id).expect("Không tìm thấy tài sản!")
    }

    // 3. Hàm Tra cứu Chủ sở hữu
    pub fn get_owner(env: Env, asset_id: u32) -> Address {
        env.storage().persistent().get(&("owner", asset_id)).expect("Tài sản chưa có chủ!")
    }
}