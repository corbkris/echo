pub trait ModelBuilder {
    fn id(&self) -> String;
    fn to_json(&self) -> serde_json::Value;
    fn table_name(&self) -> String;
}

pub enum ConditonalOperator {
    AND,
    OR,
    Basic,
}

pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Basic,
}

//SELECT * FROM your_table WHERE condition1 AND condition2 AND condition3;

pub fn search<T>(
    model: T,
    comparison: ComparisonOperator,
    conditional: ConditonalOperator,
) -> String
where
    T: ModelBuilder,
{
    let mut comparison_operator = String::from("");
    match comparison {
        ComparisonOperator::Equal => comparison_operator.push_str(" = "),
        ComparisonOperator::NotEqual => comparison_operator.push_str(" != "),
        ComparisonOperator::GreaterThan => comparison_operator.push_str(" > "),
        ComparisonOperator::GreaterThanEqual => comparison_operator.push_str(" >= "),
        ComparisonOperator::LessThan => comparison_operator.push_str(" < "),
        ComparisonOperator::LessThanEqual => comparison_operator.push_str(" <= "),
        ComparisonOperator::Basic => comparison_operator.push_str(" "),
    }

    let mut conditional_operator = String::from("");
    match conditional {
        ConditonalOperator::AND => conditional_operator.push_str(" AND "),
        ConditonalOperator::OR => conditional_operator.push_str(" OR "),
        ConditonalOperator::Basic => conditional_operator.push_str(" "),
    }

    let mut query = format!("SELECT * FROM {}", model.table_name());

    if model.to_json().as_object().unwrap().values().all(|value| {
        if value == "" {
            true
        } else if value.is_null() {
            true
        } else {
            false
        }
    }) {
        return query;
    }

    query.push_str(" WHERE ");

    for (key, value) in model.to_json().as_object().unwrap() {
        if value != "" && !value.is_null() {
            query.push_str(&key);
            query.push_str(&comparison_operator);
            if value.is_string() {
                query.push_str(&value.to_string().replace('"', "'"));
            }
            if value.is_i64() {
                query.push_str(&value.to_string().replace('"', ""));
            }
            if value.is_boolean() {
                query.push_str(&value.to_string().replace('"', ""));
            }
            query.push_str(&conditional_operator);
        }
    }

    query = query.trim_end_matches(&conditional_operator).to_string();

    query
}

//INSERT INTO MyStruct (id, name, age) VALUES ($1, $2, $3)
//"UPDATE your_table SET username = $1, password = $2 WHERE id = $3";

pub fn insert<T>(model: T) -> String
where
    T: ModelBuilder,
{
    let mut query = format!("INSERT INTO {} (", model.table_name());
    let mut values = String::from("VALUES (");

    for (key, value) in model.to_json().as_object().unwrap() {
        if key == "id" {
            query.push_str(&key);
            //values.push_str("uuid_generate_v4()::text");
            values.push_str("uuid_generate_v4()");
            query.push_str(", ");
            values.push_str(", ");
        }
        if value != "" && key != "id" && !value.is_null() {
            query.push_str(&key);
            if value.is_string() {
                values.push_str(&value.to_string().replace('"', "'"));
            }
            if value.is_i64() {
                values.push_str(&value.to_string().replace('"', ""));
            }
            if value.is_boolean() {
                values.push_str(&value.to_string().replace('"', ""));
            }

            query.push_str(", ");
            values.push_str(", ");
        }
    }
    query = query.trim_end_matches(", ").to_string();
    values = values.trim_end_matches(", ").to_string();
    query.push_str(") ");
    values.push_str(")");
    query.push_str(&values);
    query.push_str(" RETURNING *;");

    query
}

pub fn update<T>(model: T) -> String
where
    T: ModelBuilder,
{
    let mut query = format!("UPDATE {} SET ", model.table_name());
    let where_statement = format!(" WHERE id = {}", model.id());
    for (key, value) in model.to_json().as_object().unwrap() {
        if value != "" && !value.is_null() {
            if key.to_string() != "id" {
                query.push_str(&key);
                query.push_str(" = ");
                if value.is_string() {
                    query.push_str(&value.to_string().replace('"', "'"));
                }
                if value.is_i64() {
                    query.push_str(&value.to_string().replace('"', ""));
                }
                if value.is_boolean() {
                    query.push_str(&value.to_string().replace('"', ""));
                }
                query.push_str(", ");
            }
        }
    }
    query = query.trim_end_matches(", ").to_string();
    query.push_str(&where_statement);
    query.push_str(" RETURNING *;");

    query
}

pub fn delete<T>(model: T) -> String
where
    T: ModelBuilder,
{
    format!(
        "DELETE FROM {} WHERE id = {}",
        model.table_name(),
        model.id()
    )
}
