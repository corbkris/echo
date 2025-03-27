use echo_sql::generic::UUID;

use crate::business::account::Account as BusinessAccount;
use crate::stores::account::Account;

pub fn marshal(business_account: BusinessAccount) -> Account {
    Account {
        id: Some(business_account.id),
        username: business_account.username,
        created_at: None,
        updated_at: None,
    }
}

pub fn unmarshal(model_account: Account) -> BusinessAccount {
    BusinessAccount {
        id: match model_account.id {
            Some(uuid) => uuid,
            None => UUID::nil(),
        },
        email: "".to_string(),
        days_active: None,
        verified: None,
        username: model_account.username,
    }
}
