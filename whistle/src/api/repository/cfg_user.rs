use crate::api::entity::{NewUnit, Unit};

use crate::api::entity::{NewUser, User};
use crate::config::database::get_connection;
use crate::schema::users;
use crate::schema::users::dsl::{user_name, user_unit_id, users as all_users};

use diesel::expression_methods::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub fn add_user(name: &str, unit_id: i32) -> i32 {
    let conn = &mut *get_connection();

    let new_user = NewUser {
        user_name: name.to_string(),
        user_unit_id: unit_id,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    users::table
        .order(users::id.desc())
        .select(users::id)
        .first(conn)
        .expect("Error getting last inserted id")
}

pub fn get_user(user_id: i32) -> Option<User> {
    let conn = &mut *get_connection();

    all_users
        .find(user_id)
        .first(conn)
        .optional()
        .expect(&format!("Unable to find user {}", user_id))
}

pub fn get_users() -> Vec<User> {
    let conn = &mut *get_connection();

    all_users.load::<User>(conn).expect("Error loading users")
}

pub fn update_user(user_id: i32, new_name: &str, unit_id: i32) -> usize {
    let conn = &mut *get_connection();

    diesel::update(all_users.find(user_id))
        .set((user_name.eq(new_name), user_unit_id.eq(unit_id)))
        .execute(conn)
        .expect(&format!("Unable to find user {}", user_id))
}

pub fn delete_user(user_id: i32) -> usize {
    let conn = &mut *get_connection();

    diesel::delete(all_users.find(user_id))
        .execute(conn)
        .expect(&format!("Unable to find user {}", user_id))
}

pub fn delete_users() -> usize {
    let conn = &mut *get_connection();

    diesel::delete(all_users)
        .execute(conn)
        .expect("Unable to delete users")
}

pub fn show_users() {
    let results = get_users();

    println!("[ {} elements in users table ]", results.len());
    for user in results {
        println!("{:?}", user);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::api::{
        entity::NewUnit,
        repository::unit::{delete_unit, insert_unit},
    };

    #[test]
    #[ignore]
    fn test_user() {
        let new_unit = NewUnit {
            unit_name: "test_unit_name".to_string(),
            unit_code: "test_unit_code".to_string(),
        };

        // Arrange
        let unit_id = insert_unit(new_unit);

        let new_unit_2 = NewUnit {
            unit_name: "test_unit_name_updated".to_string(),
            unit_code: "test_unit_code_updated".to_string(),
        };

        let unit_id_2 = insert_unit(new_unit_2);

        let test_user_name = "hsoh";
        let updated_user_name = "hsoh_updated";

        // Act
        let insert_id = add_user(test_user_name, unit_id);
        let saved_user = get_user(insert_id);

        update_user(insert_id, updated_user_name, unit_id_2);
        let updated_user = get_user(insert_id);

        delete_user(insert_id);
        let deleted_user = get_user(insert_id);

        // Assert
        match saved_user {
            Some(user) => {
                assert_eq!(user.user_name, test_user_name);
                assert_eq!(user.user_unit_id, unit_id);
            }
            None => panic!("Saved user not found"),
        };

        match updated_user {
            Some(user) => {
                assert_eq!(user.user_name, updated_user_name);
                assert_eq!(user.user_unit_id, unit_id_2);
            }
            None => panic!("Updated user not found"),
        };

        assert!(deleted_user.is_none());

        // Clean up
        delete_unit(unit_id);
        delete_unit(unit_id_2);
    }
}
