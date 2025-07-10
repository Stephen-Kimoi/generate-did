use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct User {
    id: u64,
    name: String,
}

#[ic_cdk::query]
fn get_user(id: u64) -> User {
    User {
        id,
        name: format!("User {}", id),
    }
}

#[ic_cdk::update]
fn create_user(name: String) -> User {
    User {
        id: 1,
        name,
    }
}

ic_cdk::export_candid!();