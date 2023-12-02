use std::io::Write;

use crate::schemas::base_extensions::{CreateExtensionField, ExtensionType, CreateExtensionFieldVariant};

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

    fn generate_query(&self) -> Option<String> {
        return match self.kind {
            Some(ExtensionType::Product) => {
                let table_to_extend = "products";
                let mut query = String::new();

                for field in &self.fields {
                    match &field.variant {
                        CreateExtensionFieldVariant::STRING(
                            string_field,
                        ) => {
                            query.push_str(
                                format!(
                                    "ALTER TABLE {table_to_extend} ADD \"{}\" VARCHAR;\n",
                                    string_field.name
                                )
                                .as_str(),
                            );

                            if let Some(default_value) = &string_field.default_value {
                                query.push_str(format!("ALTER TABLE {table_to_extend} ALTER COLUMN \"{}\" SET DEFAULT '{}';\n", string_field.name, default_value).as_str());
                                query.push_str(
                                    format!(
                                        "UPDATE {table_to_extend} SET \"{}\" = '{}';\n",
                                        string_field.name, default_value
                                    )
                                    .as_str(),
                                );
                            }

                            if string_field.mandatory {
                                query.push_str(format!("ALTER TABLE {table_to_extend} ALTER COLUMN \"{}\" SET NOT NULL;\n", string_field.name).as_str());
                            }

                            query.push_str("\n");
                        }

                        CreateExtensionFieldVariant::BOOLEAN(
                            bool_field,
                        ) => {
                            query.push_str(
                                format!(
                                    "ALTER TABLE {table_to_extend} ADD \"{}\" BIT;\n",
                                    bool_field.name
                                )
                                .as_str(),
                            );

                            if let Some(default_value) = bool_field.default_value {
                                query.push_str(format!("ALTER TABLE {table_to_extend} ALTER COLUMN \"{}\" SET DEFAULT {}::bit(1);\n", bool_field.name, default_value as i32).as_str());
                                query.push_str(
                                    format!(
                                        "UPDATE {table_to_extend} SET \"{}\" = {}::bit(1);\n",
                                        bool_field.name, default_value as i32
                                    )
                                    .as_str(),
                                );
                            }

                            if bool_field.mandatory {
                                query.push_str(format!("ALTER TABLE {table_to_extend} ALTER COLUMN \"{}\" SET NOT NULL;\n", bool_field.name).as_str());
                            }

                            query.push_str("\n");
                        }
                    }
                }

                Some(query)
            }
            None => None,
        }
    }

    pub fn generate(self) -> Result<String, String> {
        let migration_query = self.generate_query();

        if let None = migration_query {
            return Err(String::from("Invalid data provided. Migration could not be generated."));
        }

        let generated_migrations_path = std::path::Path::new("generated_migrations");
        if !generated_migrations_path.exists() {
            let create_dir = std::fs::create_dir(generated_migrations_path);
            if let Err(e) = create_dir {
                return Err(e.to_string());
            }
        }

        let now = std::time::SystemTime::now();
        let elapsed = now .duration_since(std::time::UNIX_EPOCH).unwrap();
        let mut generated_migration_path_builder = generated_migrations_path.to_path_buf();
        generated_migration_path_builder.push(format!("generated-migration-{}.sql", elapsed.as_millis()));
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
