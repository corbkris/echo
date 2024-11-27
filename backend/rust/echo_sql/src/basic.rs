use serde_json::Value;

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
impl ConditonalOperator {
    pub fn to_str(&self) -> &'static str {
        match self {
            ConditonalOperator::AND => "AND",
            ConditonalOperator::OR => "OR",
            ConditonalOperator::Basic => "",
        }
    }
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
impl ComparisonOperator {
    pub fn to_str(&self) -> &'static str {
        match self {
            ComparisonOperator::Equal => "=",
            ComparisonOperator::NotEqual => "!=",
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::GreaterThanEqual => ">=",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::LessThanEqual => "<=",
            ComparisonOperator::Basic => "",
        }
    }
}

pub fn delete<T>(model: &T) -> String
where
    T: ModelBuilder,
{
    format!(
        "DELETE FROM {} WHERE id = {}",
        model.table_name(),
        model.id()
    )
}

macro_rules! insert_statement {
    ($table:expr, $fields:expr, $values:expr) => {{
        // Join fields and values to form the SQL INSERT statement
        let fields_str = $fields.join(", ");
        let values_str = $values.join(", ");
        format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING *;",
            $table, fields_str, values_str
        )
    }};
}

pub fn insert<T>(model: &T) -> String
where
    T: ModelBuilder,
{
    let json_data = model.to_json();

    let fields: Vec<String> = json_data
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(key, value)| match value {
            Value::Null => None,
            Value::String(s) if !s.is_empty() => Some(key.to_string()),
            Value::Number(_) => Some(key.to_string()),
            Value::Bool(_) => Some(key.to_string()),
            _ => None,
        })
        .collect();

    let values: Vec<String> = json_data
        .as_object()
        .unwrap()
        .values()
        .filter_map(|value| match value {
            Value::Null => None,
            Value::String(s) if !s.is_empty() => Some(format!("'{}'", s)),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        })
        .collect();

    insert_statement!(model.table_name(), fields, values)
}

macro_rules! update_statement {
    ($table:expr, $updates:expr, $id:expr) => {{
        let set_clause_str = $updates.join(", ");

        format!(
            "UPDATE {} SET {} WHERE id = {} RETURNING *;",
            $table, set_clause_str, $id
        )
    }};
}

pub fn update<T>(model: &T) -> String
where
    T: ModelBuilder,
{
    let json_data = model.to_json();

    let updates: Vec<String> = json_data
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(key, value)| {
            if key == "id" {
                return None;
            }

            match value {
                Value::Null => None,
                Value::String(s) if !s.is_empty() => Some(format!("{} = '{}'", key, s)),
                Value::Number(n) => Some(format!("{} = {}", key, n)),
                Value::Bool(b) => Some(format!("{} = {}", key, b)),
                _ => None,
            }
        })
        .collect();

    update_statement!(model.table_name(), updates, model.id())
}

macro_rules! search_statement {
    ($table:expr, $conditions:expr, $conditional_operator:expr) => {{
        let where_clause_str = if !$conditions.is_empty() {
            format!(
                " WHERE {}",
                $conditions.join(&format!(" {} ", $conditional_operator.to_str()))
            )
        } else {
            String::new()
        };
        format!("SELECT * FROM {}{};", $table, where_clause_str)
    }};
}

pub fn search<T>(
    model: &T,
    comparison: ComparisonOperator,
    conditional: ConditonalOperator,
) -> String
where
    T: ModelBuilder,
{
    let json_data = model.to_json();
    let comparison = comparison.to_str();

    let searches: Vec<String> = json_data
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(key, value)| match value {
            Value::Null => None,
            Value::String(s) if !s.is_empty() => Some(format!("{} {} '{}'", key, comparison, s)),
            Value::Number(n) => Some(format!("{} {} {}", key, comparison, n)),
            Value::Bool(b) => Some(format!("{} {} {}", key, comparison, b)),
            _ => None,
        })
        .collect();

    search_statement!(model.table_name(), searches, conditional)
}
