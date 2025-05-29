use candid::{CandidType, Deserialize};
use ic_cdk::api::call::CallResult;

#[derive(CandidType, Deserialize)]
pub struct User {
    id: u64,
    name: String,
}

#[ic_cdk::query]
fn get_user(id: u64) -> CallResult<User> {
    Ok(User {
        id,
        name: format!("User {}", id),
    })
}

#[ic_cdk::update]
fn create_user(name: String) -> CallResult<User> {
    Ok(User {
        id: 1,
        name,
    })
} 