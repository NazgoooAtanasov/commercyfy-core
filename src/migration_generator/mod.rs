use std::io::Write;

use crate::schemas::base_extensions::{
    CreateExtensionField, CreateExtensionFieldVariant, ExtensionType, Field,
};

#[derive(Default)]
pub struct MigrationGenerator {
    pub kind: Option<ExtensionType>,
    pub fields: Vec<CreateExtensionField>,
}

impl MigrationGenerator {
    pub fn new() -> Self {
        return Self {
            kind: None,
            fields: Vec::new(),
        };
    }

    pub fn set_what_to_extend(&mut self, kind: ExtensionType) -> &Self {
        self.kind = Some(kind);
        return self;
    }

    pub fn set_field(&mut self, field: CreateExtensionField) -> &Self {
        self.fields.push(field);
        return self;
    }

    fn gen_string_extend(
        &self,
        string_field: &Field<String>,
        query: &mut String,
        table_to_extend: &str,
    ) {
        query.push_str(
            format!(
                "INSERT INTO {} (name, description, value_type, default_value, mandatory) VALUES ('{}', {}, '{}', {}, {});\n",
                table_to_extend,
                string_field.name,
                string_field.description.clone().unwrap_or("NULL".to_string()),
                "string",
                string_field.default_value.clone().unwrap_or("NULL".to_string()),
                string_field.mandatory
            ).as_str()
        );
    }

    fn gen_boolean_extend(
        &self,
        bool_field: &Field<bool>,
        query: &mut String,
        table_to_extend: &str,
    ) {
        query.push_str(
            format!(
                "INSERT INTO {} (name, description, value_type, default_value, mandatory) VALUES ('{}', {}, '{}', {}, {});\n",
                table_to_extend,
                bool_field.name,
                bool_field.description.clone().unwrap_or("NULL".to_string()),
                "bool",
                bool_field.default_value.clone().unwrap_or(false),
                bool_field.mandatory
            ).as_str()
        );
    }

    fn generate_query(&self) -> Option<String> {
        return match self.kind {
            Some(ExtensionType::Product) => {
                let table_to_extend = "__meta_product_custom_fields";
                let mut query = String::new();

                for field in &self.fields {
                    match &field.variant {
                        CreateExtensionFieldVariant::STRING(strng_field) => {
                            self.gen_string_extend(strng_field, &mut query, table_to_extend);
                        }

                        CreateExtensionFieldVariant::BOOLEAN(bool_field) => {
                            self.gen_boolean_extend(bool_field, &mut query, table_to_extend);
                        }
                    }
                }

                Some(query)
            }
            None => None,
        };
    }

    pub fn generate(self) -> Result<String, String> {
        let migration_query = self.generate_query();

        if let None = migration_query {
            return Err(String::from(
                "Invalid data provided. Migration could not be generated.",
            ));
        }

        let generated_migrations_path = std::path::Path::new("generated_migrations");
        if !generated_migrations_path.exists() {
            let create_dir = std::fs::create_dir(generated_migrations_path);
            if let Err(e) = create_dir {
                return Err(e.to_string());
            }
        }

        let now = std::time::SystemTime::now();
        let elapsed = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let mut generated_migration_path_builder = generated_migrations_path.to_path_buf();
        generated_migration_path_builder
            .push(format!("generated-migration-{}.sql", elapsed.as_millis()));
        let generated_migration = generated_migration_path_builder.as_path();
        let create_file = std::fs::File::create(generated_migration);
        if let Err(e) = create_file {
            return Err(e.to_string());
        }

        let mut file = create_file.unwrap();
        let write_result = file.write_all(migration_query.unwrap().as_bytes());
        if let Err(e) = write_result {
            return Err(e.to_string());
        }

        return Ok(generated_migration.to_str().unwrap().to_string());
    }
}
