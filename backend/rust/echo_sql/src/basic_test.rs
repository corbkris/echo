#[cfg(test)]
mod tests {

    use crate::basic::{insert, search, update, ComparisonOperator, ConditonalOperator};
    use crate::tables::account::Account;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_generic_insert() {
        assert_eq!(
            insert(&Account::new(None, "corbin268".to_string(), None, None)),
            format!("INSERT INTO accounts (username) VALUES ('corbin268') RETURNING *;",)
        );
    }

    #[test]
    fn test_generic_update() {
        let now = Utc::now();
        let formatted_now = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();
        let id = Uuid::new_v4();
        assert_eq!(
            update(&Account::new(
                Some(id),
                "corbin268".to_string(),
                None,
                Some(now)
            )),
            format!("UPDATE accounts SET username = 'corbin268', updated_at = '{}' WHERE id = '{}' RETURNING *;",formatted_now,id)
        )
    }

    #[test]
    fn test_generic_update_nil() {
        let now = Utc::now();
        let formatted_now = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();
        let id = Uuid::new_v4();
        assert_eq!(
            update(&Account::new(
                Some(id),
                "wacky".to_string(),
                None,
                Some(now)
            )),
            format!("UPDATE accounts SET username = 'wacky', updated_at = '{}' WHERE id = '{}' RETURNING *;",formatted_now,id)
        )
    }

    #[test]
    fn test_generic_search() {
        let now = Utc::now();
        let formatted_now = now.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string();
        let id = Uuid::new_v4();
        assert_eq!(
            search(
                &Account::new(
                    Some(id),
                    "corbin268".to_string(),
                    Some(now),
                    Some(now)
                ),
                ComparisonOperator::Equal,
                ConditonalOperator::AND,
            ),
            format!("SELECT * FROM accounts WHERE id = '{}' AND username = 'corbin268' AND created_at = '{}' AND updated_at = '{}';",id,formatted_now,formatted_now)
        )
    }

    #[test]
    fn test_generic_search_nil() {
        let id = Uuid::new_v4();
        assert_eq!(
            search(
                &Account::new(Some(id), "".to_string(), None, None),
                ComparisonOperator::Equal,
                ConditonalOperator::Basic,
            ),
            format!("SELECT * FROM accounts WHERE id = '{}';", id)
        )
    }

    #[test]
    fn test_generic_search_basic() {
        assert_eq!(
            search(
                &Account::new(None, "".to_string(), None, None),
                ComparisonOperator::Basic,
                ConditonalOperator::Basic,
            ),
            r#"SELECT * FROM accounts;"#
        )
    }
}
