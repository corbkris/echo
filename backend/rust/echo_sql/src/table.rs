use crate::{
    basic::{ComparisonOperator, ConditonalOperator, ModelBuilder},
    generic::{Argument, PostgresError, PostgresQueryResult, DB},
};

use sqlx::{postgres::PgRow, FromRow};

pub struct BaseTable<'a, T: ModelBuilder + Send + Sync> {
    db: &'a DB<'a>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: ModelBuilder + Send + Sync + Unpin + for<'r> FromRow<'r, PgRow>> BaseTable<'a, T> {
    pub fn new(db: &'a DB) -> Self {
        Self {
            db,
            _marker: std::marker::PhantomData,
        }
    }

    pub async fn insert(&self, item: &mut T) -> Option<PostgresError> {
        self.db.insert(item).await
    }

    pub async fn update(&self, item: &mut T) -> Option<PostgresError> {
        self.db.update(item).await
    }

    pub async fn delete(&self, item: &T) -> Result<PostgresQueryResult, PostgresError> {
        self.db.delete(item).await
    }

    //use for basic queries that return single objects
    pub async fn search(
        &self,
        item: &T,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<T, PostgresError> {
        self.db.search(item, comparison, conditional).await
    }

    //use for basic queries that return multiple results
    pub async fn search_all(
        &self,
        item: &T,
        comparison: ComparisonOperator,
        conditional: ConditonalOperator,
    ) -> Result<Vec<T>, PostgresError> {
        self.db.search_all(item, comparison, conditional).await
    }

    //use for complex queries that return single objects
    pub async fn query(&self, query: &str, args: Vec<Argument>) -> Result<T, PostgresError> {
        self.db.query(query, args).await
    }

    //use for complex queries that return multiple results
    pub async fn query_all(
        &self,
        query: String,
        args: Vec<Argument>,
    ) -> Result<Vec<T>, PostgresError> {
        self.db.query_all(query, args).await
    }
}

#[macro_export]
macro_rules! impl_deref_store {
    ($store_type:ident, $target_type:ident) => {
        impl<'a> std::ops::Deref for $store_type<'a> {
            type Target = BaseTable<'a, $target_type>;

            fn deref(&self) -> &Self::Target {
                &self.base_table
            }
        }

        impl<'a> std::ops::DerefMut for $store_type<'a> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base_table
            }
        }
    };
}
