use chrono::Utc;
use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::{Argument, PostgresError, DB},
    impl_deref_store,
    table::BaseTable,
    tables::signup_verification::SignupVerification as TableSignupVerification,
};
use uuid::Uuid;

pub type SignupVerification = TableSignupVerification;

impl_deref_store!(SignupVerificationStore, SignupVerification);
pub struct SignupVerificationStore<'a> {
    pub base_table: BaseTable<'a, SignupVerification>,
}

pub fn new_signup_verification_table<'a>(db: &'a DB) -> BaseTable<'a, SignupVerification> {
    BaseTable::<SignupVerification>::new(db)
}

impl<'a> SignupVerificationStore<'a> {
    pub fn new(base_table: BaseTable<'a, SignupVerification>) -> Self {
        Self { base_table }
    }

    pub async fn find_unexpired_by_id_code(
        &self,
        id: Uuid,
        code: &str,
    ) -> Result<SignupVerification, PostgresError> {
        let query = "
            SELECT sv.*
            FROM signup_verification sv
            WHERE sv.id = $1
            AND sv.code = $2
            AND sv.expiration > $1
            ";

        self.query(
            query,
            vec![
                Argument::Str(id.to_string()),
                Argument::Str(code.to_string()),
                Argument::Str(Utc::now().to_string()),
            ],
        )
        .await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<SignupVerification, PostgresError> {
        self.search(
            &SignupVerification {
                id: Some(id),
                ..Default::default()
            },
            ComparisonOperator::Equal,
            ConditonalOperator::Basic,
        )
        .await
    }
}
