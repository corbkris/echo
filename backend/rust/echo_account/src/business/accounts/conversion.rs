use crate::business::account::Account as BusinessAccount;
use crate::stores::account::Account;

pub fn marshal(business_account: BusinessAccount) -> Account {
    Account {
        id: business_account.id,
        email: business_account.email,
        password: business_account.password,
        days_active: business_account.days_active,
        verified: business_account.verified,
        created_at: None,
        updated_at: None,
    }
}

pub fn unmarshal(model_account: Account) -> BusinessAccount {
    BusinessAccount {
        id: model_account.id,
        email: model_account.email,
        password: model_account.password,
        days_active: model_account.days_active,
        verified: model_account.verified,
    }
}
