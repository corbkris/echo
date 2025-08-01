use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    connection::PostgresPool,
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

pub fn new_signup_verification_table<'a>(
    pool: &'a PostgresPool,
) -> BaseTable<'a, SignupVerification> {
    BaseTable::<SignupVerification>::new(DB::new(pool))
}

impl<'a> SignupVerificationStore<'a> {
    pub fn new(pool: &'a PostgresPool) -> Self {
        Self {
            base_table: new_signup_verification_table(pool),
        }
    }

    pub async fn find_by_id_code(
        &self,
        id: Uuid,
        code: &str,
    ) -> Result<SignupVerification, PostgresError> {
        let query = "
            SELECT sv.*
            FROM signup_verification sv
            WHERE sv.id = $1
            AND sv.code = $2";

        self.query(query, vec![Argument::Uuid(id), Argument::Str(code)])
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
