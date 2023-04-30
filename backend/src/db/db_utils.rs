// use diesel::mysql::MysqlConnection;
// use diesel::prelude::*;
// use std::env;

use actix_web::web;

use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;

type DbPool = Pool<ConnectionManager<MysqlConnection>>;

// pub fn valid_user(pool: web::Data<DbPool>, user: common::UserData) -> Result<bool, diesel::result::Error> {
//     let conn: &MysqlConnection = &pool.get().unwrap();
//     let results = users
//         .filter(matricula.eq(user.matricula))
//         .filter(hash.eq(user.hash))
//         .load::<common::UserData>(conn)?;
//     if results.len() == 1 {
//         Ok(true)
//     } else {
//         Ok(false)
//     }
// }
