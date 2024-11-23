#[cfg(test)]
mod tests {

    use crate::basic::{delete, insert, search, update, ComparisonOperator, ConditonalOperator};
    use crate::models::account::Account;
    use chrono::Utc;

    #[test]
    fn test_generic_insert() {
        assert_eq!(
            insert(Account::new(
                "".to_string(),
                "corbin268".to_string(),
                "password".to_string(),
                None,
                None
            )),
                    format!(
            "INSERT INTO accounts (username, password) VALUES ('corbin268', 'password') RETURNING *;",
        )
        );
    }

    #[test]
    fn test_generic_insert_nil() {
        assert_eq!(
            insert(Account::new(
                "".to_string(),
                "".to_string(),
                "password".to_string(),
                None,
                None
            )),
            r#"INSERT INTO accounts (password) VALUES ('password') RETURNING *;"#
        );
    }

    #[test]
    fn test_generic_delete() {
        assert_eq!(
            delete(Account::new(
                "5".to_string(),
                "".to_string(),
                "".to_string(),
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
                "corbin268".to_string(),
                "gogins".to_string(),
                None,
                Some(now)
            )),
            format!("UPDATE accounts SET username = 'corbin268', password = 'gogins', updated_at = '{}' WHERE id = '5' RETURNING *;",formatted_now)
        )
    }

    #[test]
    fn test_generic_update_nil() {
        let now = Utc::now();
        let formatted_now = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();
        assert_eq!(
            update(Account::new(
                "5".to_string(),
                "".to_string(),
                "gogins".to_string(),
                None,
                Some(now)
            )),
            format!("UPDATE accounts SET password = 'gogins', updated_at = '{}' WHERE id = '5' RETURNING *;",formatted_now)
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
                    "corbin268".to_string(),
                    "password".to_string(),
                    Some(now),
                    Some(now)
                ),
                ComparisonOperator::Equal,
                ConditonalOperator::AND,
            ),
            format!("SELECT * FROM accounts WHERE id = '6' AND username = 'corbin268' AND password = 'password' AND created_at = '{}' AND updated_at = '{}';",formatted_now,formatted_now)
        )
    }

    #[test]
    fn test_generic_search_nil() {
        assert_eq!(
            search(
                Account::new("6".to_string(), "".to_string(), "".to_string(), None, None),
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
                Account::new("".to_string(), "".to_string(), "".to_string(), None, None),
                ComparisonOperator::Basic,
                ConditonalOperator::Basic,
            ),
            r#"SELECT * FROM accounts;"#
        )
    }
}
