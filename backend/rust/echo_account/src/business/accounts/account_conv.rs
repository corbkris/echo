use uuid::Uuid;

use crate::business::account::Account as BusinessAccount;
use crate::stores::account::Account;

pub fn marshal(orig: BusinessAccount) -> Account {
    Account {
        id: Some(orig.id),
        username: orig.username,
        created_at: None,
        updated_at: None,
    }
}

pub fn unmarshal(orig: Account) -> BusinessAccount {
    BusinessAccount {
        id: match orig.id {
            Some(uuid) => uuid,
            None => Uuid::nil(),
        },
        username: orig.username,
        created_at: orig.created_at,
        updated_at: orig.updated_at,
    }
}
