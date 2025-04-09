use crate::business::account::Account as BusinessAccount;
use crate::stores::account::Account;

pub fn marshal(orig: BusinessAccount) -> Account {
    Account {
        id: orig.id,
        username: orig.username,
        created_at: None,
        updated_at: None,
    }
}

pub fn unmarshal(orig: Account) -> BusinessAccount {
    BusinessAccount {
        id: orig.id,
        username: orig.username,
        created_at: orig.created_at,
        updated_at: orig.updated_at,
    }
}
