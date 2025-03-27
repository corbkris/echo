use crate::{
    basic::{delete, insert, search, update, ComparisonOperator, ConditonalOperator, ModelBuilder},
    connection::PostgresPool,
};
use sqlx::{
    postgres::{PgQueryResult, PgRow},
    query, query_as, Error, FromRow, Postgres,
};
use uuid::Uuid;

pub type PostgresError = Error;
pub type PostgresQueryResult = PgQueryResult;
pub type UUID = Uuid;

#[derive(Debug)]
pub enum Argument {
    Int(i32),
    Str(String),
    Float(f64),
    Bool(bool),
}

pub struct DB<'a> {
    pub pool: &'a PostgresPool,
}

impl<'a> DB<'a> {
    pub fn new(pool: &'a PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn insert<T>(&self, model: &mut T) -> Option<PostgresError>
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

    pub async fn update<T>(&self, model: &mut T) -> Option<PostgresError>
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

    pub async fn delete<T>(&self, model: &T) -> Result<PostgresQueryResult, PostgresError>
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
    ) -> Result<T, PostgresError>
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
    ) -> Result<Vec<T>, PostgresError>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        query_as::<Postgres, T>(&search(model, comparison, conditional))
            .fetch_all(self.pool)
            .await
    }

    /// Example
    /// let id = 123;
    /// let active = true;
    /// let query = "SELECT * FROM users WHERE id = $1 AND active = $2";
    /// let mut args = vec![Argument::Int(5)];
    /// args.push(Argument::Bool(true));
    /// let result = self.query(query, arguments).await?;
    pub async fn query<T>(&self, query: &str, args: Vec<Argument>) -> Result<T, PostgresError>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        let mut query = query_as::<Postgres, T>(query);
        for arg in args {
            match arg {
                Argument::Int(i) => query = query.bind(i),
                Argument::Str(s) => query = query.bind(s),
                Argument::Float(f) => query = query.bind(f),
                Argument::Bool(b) => query = query.bind(b),
            }
        }
        query.fetch_one(self.pool).await
    }

    pub async fn query_all<T>(
        &self,
        query: String,
        args: Vec<Argument>,
    ) -> Result<Vec<T>, PostgresError>
    where
        T: ModelBuilder + Send + Unpin + for<'r> FromRow<'r, PgRow>,
    {
        let mut query = query_as::<Postgres, T>(&query);
        for arg in args {
            match arg {
                Argument::Int(i) => query = query.bind(i),
                Argument::Str(s) => query = query.bind(s),
                Argument::Float(f) => query = query.bind(f),
                Argument::Bool(b) => query = query.bind(b),
            }
        }
        query.fetch_all(self.pool).await
    }
}
