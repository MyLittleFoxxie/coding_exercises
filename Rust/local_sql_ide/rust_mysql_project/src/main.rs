use sqlx::{MySql, Column, Row, Pool, TypeInfo}; // Import TypeInfo
use dotenvy::dotenv;
use std::env;
use serde_json::{Value, Number};
use tokio::io::{self, AsyncBufReadExt, BufReader};

async fn run_query(pool: &Pool<MySql>, query: &str) -> Result<Vec<Value>, sqlx::Error> {
    let rows = sqlx::query(query).fetch_all(pool).await?;

    let json_rows: Vec<Value> = rows.into_iter()
        .map(|row| {
            let mut obj = serde_json::Map::new();
            for col in row.columns() {
                let col_name = col.name();
                let col_type = col.type_info().name();
                let value: Value = match col_type {
                    "VARCHAR" | "CHAR" | "TEXT" => match row.try_get::<&str, _>(col_name) {
                        Ok(v) => Value::String(v.to_string()),
                        Err(_) => Value::Null,
                    },
                    "INT" | "TINYINT" | "SMALLINT" | "MEDIUMINT" | "BIGINT" => row.try_get::<i64, _>(col_name).map_or(Value::Null, |v| Value::Number(Number::from(v))),
                    "DECIMAL" | "FLOAT" | "DOUBLE" => row.try_get::<f64, _>(col_name).map_or(Value::Null, |v| serde_json::to_value(v).unwrap_or(Value::Null)),
                    _ => Value::Null,
                };
                obj.insert(col_name.to_owned(), value);
            }
            Value::Object(obj)
        })
        .collect();

    Ok(json_rows)
}

async fn interactive_mode(pool: Pool<MySql>) -> Result<(), Box<dyn std::error::Error>> {
    let mut line = String::new();
    let mut stdin = BufReader::new(io::stdin());

    println!("Enter SQL queries to execute or type 'exit' to stop:");

    while {
        line.clear();
        stdin.read_line(&mut line).await?; // Use BufReader's read_line
        !line.trim().eq_ignore_ascii_case("exit") && !line.trim().is_empty()
    } {
        match run_query(&pool, line.trim()).await {
            Ok(json_rows) => {
                let json_str = serde_json::to_string_pretty(&json_rows).unwrap();
                println!("JSON: {}", json_str);
            },
            Err(e) => eprintln!("Error executing query: {}", e),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::<MySql>::connect(&database_url).await.expect("Failed to connect to database");
    println!("Connected to database successfully.");
    
    if let Err(e) = interactive_mode(pool).await {
        eprintln!("An error occurred: {}", e);
    }
}
