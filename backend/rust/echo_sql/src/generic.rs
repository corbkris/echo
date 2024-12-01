use crate::{
    basic::{delete, insert, search, update, ComparisonOperator, ConditonalOperator, ModelBuilder},
    connection::PostgresPool,
};
use sqlx::{
    postgres::{PgQueryResult, PgRow},
    query, query_as, Error, FromRow, Postgres,
};

pub struct DB<'a> {
    pub pool: &'a PostgresPool,
}

impl<'a> DB<'a> {
    pub fn new(pool: &'a PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn insert<T>(&self, model: &mut T) -> Option<Error>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        match query_as::<Postgres, T>(&insert(model))
            .fetch_one(self.pool)
            .await
        {
            Ok(updated_model) => {
                *model = updated_model;
                None
            }
            Err(err) => Some(err),
        }
    }

    pub async fn update<T>(&self, model: &mut T) -> Option<Error>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        match query_as::<Postgres, T>(&update(model))
            .fetch_one(self.pool)
            .await
        {
            Ok(updated_model) => {
                *model = updated_model;
                None
            }
            Err(err) => Some(err),
        }
    }

    pub async fn delete<T>(&self, model: &T) -> Result<PgQueryResult, Error>
    where
        T: ModelBuilder,
    {
        query(&delete(model)).execute(self.pool).await
    }

    pub async fn search<T>(
        &self,
        model: &T,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<T, Error>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        query_as::<Postgres, T>(&search(model, comparison, conditional))
            .fetch_one(self.pool)
            .await
    }

    pub async fn search_all<T>(
        &self,
        model: &T,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<Vec<T>, Error>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        query_as::<Postgres, T>(&search(model, comparison, conditional))
            .fetch_all(self.pool)
            .await
    }
}
