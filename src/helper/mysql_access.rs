// use sqlx::postgres::PgPoolOptions;
use sqlx::{mysql::MySqlPoolOptions, Executor, MySql, Pool};

pub async fn connect_mysql() -> sqlx::Result<Pool<MySql>> {
    let pool = MySqlPoolOptions::new()
        .connect("mysql://root:@localhost/test")
        .await?;
    // pool.execute("SHOW DATABASES").await?;
    let res = sqlx::query("SHOW DATABASES").fetch_all(&pool).await?;
    println!("Qry Res {:#?}", res);
    Ok(pool)
}