#[cfg(test)]
mod tests {

    use crate::basic::{delete, insert, search, update, ComparisonOperator, ConditonalOperator};
    use crate::models::account::Account;
    use crate::models::product::Product;
    use chrono::Utc;

    #[test]
    fn test_generic_insert() {
        let now = Utc::now();
        let formatted_now = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();
        assert_eq!(
            insert(Account::new(
                "".to_string(),
                "cardboard123".to_string(),
                "corbin268".to_string(),
                "dad".to_string(),
                Some(5),
                Some(true),
                Some(now),
                Some(now)
            )),
                    format!(
            "INSERT INTO accounts (id, username, email, password, days_active, verified, created_at, updated_at) VALUES (uuid_generate_v4(), 'cardboard123', 'corbin268', 'dad', 5, true, '{}', '{}') RETURNING *;",
           formatted_now,
           formatted_now
        )
        );
    }

    #[test]
    fn test_generic_insert_nil() {
        assert_eq!(
            insert(Account::new(
                "".to_string(),
                "".to_string(),
                "corbin268".to_string(),
                "dad".to_string(),
                None,
                None,
                None,
                None
            )),
            r#"INSERT INTO accounts (id, email, password) VALUES (uuid_generate_v4(), 'corbin268', 'dad') RETURNING *;"#
        );
    }

    #[test]
    fn test_generic_insert_uuid() {
        let now = Utc::now();
        let formatted_now = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();
        assert_eq!(
            insert(Product::new(
                "".to_string(),
                "blanket".to_string(),
                Some(now),
                Some(now)
            ),),
            format!("INSERT INTO products (id, name, created_at, updated_at) VALUES (uuid_generate_v4(), 'blanket', '{}', '{}') RETURNING *;", formatted_now,formatted_now)
        )
    }

    #[test]
    fn test_generic_delete() {
        assert_eq!(
            delete(Account::new(
                "5".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                None,
                None,
                None,
                None
            )),
            r#"DELETE FROM accounts WHERE id = '5'"#
        )
    }

    #[test]
    fn test_generic_update() {
        let now = Utc::now();
        let formatted_now = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();
        assert_eq!(
            update(Account::new(
                "5".to_string(),
                "".to_string(),
                "".to_string(),
                "gogins".to_string(),
                Some(7),
                Some(true),
                Some(now),
                Some(now)
            )),
            format!("UPDATE accounts SET password = 'gogins', days_active = 7, verified = true, created_at = '{}', updated_at = '{}' WHERE id = '5' RETURNING *;",formatted_now,formatted_now)
        )
    }

    #[test]
    fn test_generic_update_nil() {
        assert_eq!(
            update(Account::new(
                "5".to_string(),
                "".to_string(),
                "".to_string(),
                "gogins".to_string(),
                None,
                None,
                None,
                None
            )),
            r#"UPDATE accounts SET password = 'gogins' WHERE id = '5' RETURNING *;"#
        )
    }

    #[test]
    fn test_generic_search() {
        let now = Utc::now();
        let formatted_now = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();
        assert_eq!(
            search(
                Account::new(
                    "6".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    Some(7),
                    Some(true),
                    Some(now),
                    Some(now)
                ),
                ComparisonOperator::Equal,
                ConditonalOperator::AND,
            ),
            format!("SELECT * FROM accounts WHERE id = '6' AND days_active = 7 AND verified = true AND created_at = '{}' AND updated_at = '{}';",formatted_now,formatted_now)
        )
    }

    #[test]
    fn test_generic_search_nil() {
        assert_eq!(
            search(
                Account::new(
                    "6".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    None,
                    None,
                    None,
                    None
                ),
                ComparisonOperator::Equal,
                ConditonalOperator::Basic,
            ),
            r#"SELECT * FROM accounts WHERE id = '6';"#
        )
    }

    #[test]
    fn test_generic_search_basic() {
        assert_eq!(
            search(
                Account::new(
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    None,
                    None,
                    None,
                    None
                ),
                ComparisonOperator::Basic,
                ConditonalOperator::Basic,
            ),
            r#"SELECT * FROM accounts;"#
        )
    }
}
