use crate::business::account::Account as BusinessAccount;
use crate::stores::account::Account;

pub fn marshal(business_account: BusinessAccount) -> Account {
    Account {
        id: business_account.id,
        username: business_account.username,
        password: business_account.password,
        created_at: None,
        updated_at: None,
    }
}

pub fn unmarshal(model_account: Account) -> BusinessAccount {
    BusinessAccount {
        id: model_account.id,
        email: "".to_string(),
        days_active: None,
        verified: None,
        username: model_account.username,
        password: model_account.password,
    }
}
