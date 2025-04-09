use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::{PostgresError, PostgresQueryResult, DB},
    tables::booth::Booth as ModelBooth,
};
pub type Booth = ModelBooth;

pub struct BoothStore<'a> {
    db: &'a DB<'a>,
}

impl<'a> BoothStore<'a> {
    pub fn new(db: &'a DB) -> Self {
        Self { db }
    }

    pub async fn insert(&self, booth: &mut Booth) -> Option<PostgresError> {
        self.db.insert(booth).await
    }

    pub async fn update(&self, booth: &mut Booth) -> Option<PostgresError> {
        self.db.update(booth).await
    }

    pub async fn delete(&self, booth: &Booth) -> Result<PostgresQueryResult, PostgresError> {
        self.db.delete(booth).await
    }

    pub async fn basic_search(
        &self,
        booth: &Booth,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<Vec<Booth>, PostgresError> {
        self.db.search_all(booth, comparison, conditional).await
    }

    pub async fn basic_search_single(
        &self,
        booth: &Booth,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<Booth, PostgresError> {
        self.db.search(booth, comparison, conditional).await
    }
}
