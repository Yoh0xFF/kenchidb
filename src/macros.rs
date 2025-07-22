use crate::{database::Collection, schema::{Document, Value}};

// Macro to define schemas with TypeScript-like syntax
#[macro_export]
macro_rules! define_schema {
    (
        $schema_name:ident {
            $($field_name:ident: $field_type:tt,)*
        }
    ) => {
        pub struct $schema_name;

        impl $schema_name {
            pub fn schema() -> crate::schema::Schema {
                crate::schema::Schema::new(
                    stringify!($schema_name).to_string(),
                    vec![
                        $(
                            define_schema!(@create_field $field_name, $field_type)
                        ),*
                    ]
                )
            }

            pub fn create() -> crate::macros::DocumentBuilder<$schema_name> {
                crate::macros::DocumentBuilder::new()
            }
        }
    };

    // Handle non-nullable fields
    (@create_field $field_name:ident, $field_type:ident) => {
        crate::schema::Field {
            name: stringify!($field_name).to_string(),
            field_type: define_schema!(@field_type $field_type),
            nullable: false,
        }
    };

    // Handle nullable fields
    (@create_field $field_name:ident, $field_type:ident?) => {
        crate::schema::Field {
            name: stringify!($field_name).to_string(),
            field_type: define_schema!(@field_type $field_type),
            nullable: true,
        }
    };

    (@field_type byte) => { crate::schema::FieldType::Byte };
    (@field_type short) => { crate::schema::FieldType::Short };
    (@field_type int) => { crate::schema::FieldType::Int };
    (@field_type long) => { crate::schema::FieldType::Long };
    (@field_type float) => { crate::schema::FieldType::Float };
    (@field_type double) => { crate::schema::FieldType::Double };
    (@field_type string) => { crate::schema::FieldType::String };
    (@field_type boolean) => { crate::schema::FieldType::Boolean };
}

// Document builder for type-safe document creation
pub struct DocumentBuilder<T> {
    document: Document,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> DocumentBuilder<T> {
    pub fn new() -> Self {
        Self {
            document: Document::new(0), // ID will be set by collection
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn set<V: Into<Value>>(mut self, field: &str, value: V) -> Self {
        self.document.set(field, value);
        self
    }

    pub fn build(self) -> Document {
        self.document
    }
}

// Implement Into<Value> for all primitive types
impl From<u8> for Value {
    fn from(val: u8) -> Self {
        Value::Byte(val)
    }
}

impl From<i16> for Value {
    fn from(val: i16) -> Self {
        Value::Short(val)
    }
}

impl From<i32> for Value {
    fn from(val: i32) -> Self {
        Value::Int(val)
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Value::Long(val)
    }
}

impl From<f32> for Value {
    fn from(val: f32) -> Self {
        Value::Float(val)
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Double(val)
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        if val.chars().count() > 255 {
            panic!(
                "String too long: {} characters (max 255)",
                val.chars().count()
            );
        }
        Value::String(val)
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Value::from(val.to_string())
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Value::Boolean(val)
    }
}

// Query builder for type-safe queries
pub struct QueryBuilder<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> QueryBuilder<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    // Simple field-based filtering (can be extended)
    pub fn where_eq(&self, field: &str, value: Value) -> SimpleQuery {
        SimpleQuery {
            field: field.to_string(),
            operation: QueryOperation::Equals,
            value,
        }
    }
}

#[derive(Debug)]
pub struct SimpleQuery {
    pub field: String,
    pub operation: QueryOperation,
    pub value: Value,
}

#[derive(Debug)]
pub enum QueryOperation {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
}

impl SimpleQuery {
    pub fn matches(&self, document: &Document) -> bool {
        if let Some(doc_value) = document.get(&self.field) {
            match self.operation {
                QueryOperation::Equals => doc_value == &self.value,
                QueryOperation::NotEquals => doc_value != &self.value,
                QueryOperation::GreaterThan => self.compare_greater(doc_value, &self.value),
                QueryOperation::LessThan => self.compare_less(doc_value, &self.value),
            }
        } else {
            false
        }
    }

    fn compare_greater(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Byte(a), Value::Byte(b)) => a > b,
            (Value::Short(a), Value::Short(b)) => a > b,
            (Value::Int(a), Value::Int(b)) => a > b,
            (Value::Long(a), Value::Long(b)) => a > b,
            (Value::Float(a), Value::Float(b)) => a > b,
            (Value::Double(a), Value::Double(b)) => a > b,
            (Value::String(a), Value::String(b)) => a > b,
            _ => false,
        }
    }

    fn compare_less(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Byte(a), Value::Byte(b)) => a < b,
            (Value::Short(a), Value::Short(b)) => a < b,
            (Value::Int(a), Value::Int(b)) => a < b,
            (Value::Long(a), Value::Long(b)) => a < b,
            (Value::Float(a), Value::Float(b)) => a < b,
            (Value::Double(a), Value::Double(b)) => a < b,
            (Value::String(a), Value::String(b)) => a < b,
            _ => false,
        }
    }
}

impl Collection {
    pub fn find_where(&self, query: &SimpleQuery) -> Vec<&Document> {
        self.documents
            .values()
            .filter(|doc| query.matches(doc))
            .collect()
    }

    pub fn find_one_where(&self, query: &SimpleQuery) -> Option<&Document> {
        self.documents.values().find(|doc| query.matches(doc))
    }
}
