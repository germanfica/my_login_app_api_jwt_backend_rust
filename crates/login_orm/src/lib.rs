pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .or_else(|_| env::var("DATABASE_URL"))
//         .expect("DATABASE_URL must be set");

//     println!("Database URL: {}", database_url); // Para debuggear

//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("MYSQL_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


use diesel::prelude::*;
use diesel::result::Error;
use crate::models::{User, Role, Privilege};
use crate::schema::{user::dsl as user_dsl, role::dsl as role_dsl, privileges::dsl as privileges_dsl};

// Función para obtener los roles de un usuario por su ID
pub fn get_user_roles(conn: &mut MysqlConnection, user_id: i32) -> Result<Vec<String>, Error> {
    // Consultar la tabla de privilegios para obtener los roles asociados a un usuario
    let roles = privileges_dsl::privileges
        .filter(privileges_dsl::userId.eq(user_id))  // Filtrar por ID de usuario
        .inner_join(role_dsl::role)                   // Hacer el join con la tabla `role`
        .select(role_dsl::name)                       // Seleccionar el nombre de los roles
        .load::<String>(conn)?;                       // Ejecutar la consulta y obtener los resultados

    Ok(roles)
}
