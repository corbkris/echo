use echo_sql::{
    basic::{ComparisonOperator, ConditonalOperator},
    generic::DB,
    models::booth::Booth,
};
use sqlx::{postgres::PgQueryResult, Error};

pub struct BoothStore {
    db: DB,
}

impl BoothStore {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub async fn insert(&mut self, booth: &mut Booth) -> Option<Error> {
        self.db.insert(booth).await
    }

    pub async fn update(&mut self, booth: &mut Booth) -> Option<Error> {
        self.db.update(booth).await
    }

    pub async fn delete(&mut self, booth: &Booth) -> Result<PgQueryResult, Error> {
        self.db.delete(booth).await
    }

    pub async fn basic_search(
        &mut self,
        booth: &Booth,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<Vec<Booth>, Error> {
        self.db.search_all(booth, comparison, conditional).await
    }

    pub async fn basic_search_single(
        &mut self,
        booth: &Booth,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<Booth, Error> {
        self.db.search(booth, comparison, conditional).await
    }
}
