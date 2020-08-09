use tokio_postgres::{Client, NoTls, Row};
use tokio_postgres::types::ToSql;
use tokio_postgres::error::Error;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/migrations");
}

fn make_conn_string(config: &config::Config) -> String {
    let dbhost: String = config.get("dbhost").ok().unwrap();
    let dbport: u32 = config.get("dbport").ok().unwrap();
    let dbuser: String = config.get("dbuser").ok().unwrap();
    let dbpass: String = config.get("dbpassword").ok().unwrap();
    let dbname: String = config.get("dbname").ok().unwrap();
    format!("host={} port={} user={} password={} dbname={}", dbhost, dbport, dbuser, dbpass, dbname)
}

pub async fn connect(config: &config::Config) -> Result<Client, Error> {
    let connection_string = make_conn_string(&config);
    println!("{}", connection_string);
    let (client, conn) = tokio_postgres::connect(&connection_string, NoTls).await.ok().unwrap();
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(client)
}

pub async fn execute_action(client: &Client, query: &str) -> Result<(), Error> {
    client.batch_execute(&query).await
}

pub async fn execute(client: &Client, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, Error> {
    client.query(query, params).await
}

pub async fn migrate(client: &mut Client) -> Result<refinery::Report, refinery::Error> {
    embedded::migrations::runner().run_async(client).await
}
