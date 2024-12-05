use redis::{Commands, RedisResult};
use std::{env, fmt::Result};

fn connect() -> redis::Connection {
    //format - host:port
    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

    //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };

    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    //println!("{}", redis_conn_url);

    redis::Client::open(&*redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

pub fn  store_key_and_token(key:String, token: String) -> RedisResult<bool> {
    let mut conn = connect();
    println!("******* Running SET, GET, INCR commands *******");

    let _: () = redis::cmd("SET")
        .arg(&key)
        .arg(&token)
        .query(&mut conn)?;

    let found_token: String = redis::cmd("GET")
        .arg(key)
        .query(&mut conn)?;
    if found_token != token {
        return Ok(false);
    }
    Ok(true)
}