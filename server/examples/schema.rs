
use db_schema::PgSchema;
use sqlx::{PgPool, pool};

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://postgres:postgres@45.128.222.100:15432")
        .await
        .unwrap();
    generate_sql_statements_for_schema(&pool).await.unwrap();
}

async fn generate_sql_statements_for_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    let schema = PgSchema::new("public");

    let enums = schema.get_enums(pool).await?;
    let types = schema.get_types(pool).await?;
    let tables = schema.get_tables(pool).await?;
    let views = schema.get_views(pool).await?;
    let mviews = schema.get_mviews(pool).await?;
    let functions = schema.get_functions(pool).await?;
    let triggers = schema.get_triggers(pool).await?;
    let indexes = schema.get_indexes(pool).await?;

    println!("Enums: {:?}", enums);
    println!("Types: {:?}", types);
    println!("Tables: {:?}", tables);
    println!("Views: {:?}", views);
    println!("Materialized Views: {:?}", mviews);
    println!("Functions: {:?}", functions);
    println!("Triggers: {:?}", triggers);
    println!("Indexes: {:?}", indexes);

    Ok(())
}