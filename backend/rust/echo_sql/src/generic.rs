use crate::basic::{
    delete, insert, search, update, ComparisonOperator, ConditonalOperator, ModelBuilder,
};
use sqlx::{
    postgres::{PgPool, PgQueryResult, PgRow},
    query, query_as, Error, FromRow, Postgres,
};

#[derive(Clone)]
pub struct DB {
    pub pool: PgPool,
}

impl DB {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert<T>(&mut self, model: T) -> Result<T, Error>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        query_as::<Postgres, T>(&insert(model))
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update<T>(&mut self, model: T) -> Result<T, Error>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        query_as::<Postgres, T>(&update(model))
            .fetch_one(&self.pool)
            .await
    }

    pub async fn delete<T>(&mut self, model: T) -> Result<PgQueryResult, Error>
    where
        T: ModelBuilder,
    {
        query(&delete(model)).execute(&self.pool).await
    }

    pub async fn search<T>(
        &mut self,
        model: T,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<T, Error>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        query_as::<Postgres, T>(&search(model, comparison, conditional))
            .fetch_one(&self.pool)
            .await
    }

    pub async fn search_all<T>(
        &mut self,
        model: T,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<Vec<T>, Error>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        query_as::<Postgres, T>(&search(model, comparison, conditional))
            .fetch_all(&self.pool)
            .await
    }
}
