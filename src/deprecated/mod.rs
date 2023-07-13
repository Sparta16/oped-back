// use crate::models::User;
// use async_trait::async_trait;
// use sqlx::{FromRow, Pool, Postgres};
// use std::sync::Arc;
//
// #[derive(FromRow)]
// struct ReturnedId {
//     id: i32,
// }
//
// pub struct UserRepositoryImp {
//     pub(crate) pool: Arc<Pool<Postgres>>,
// }
//
// #[async_trait]
// impl UserRepository for UserRepositoryImp {
//     async fn select_all(&self) -> Result<Vec<User>, UserRepositoryError> {
//         let users = sqlx::query_as::<_, User>("SELECT * FROM public.user")
//             .fetch_all(&*self.pool)
//             .await;
//
//         match users {
//             Ok(users) => Ok(users),
//             _ => Err(UserRepositoryError::Message(
//                 "Failed to select all users".to_owned(),
//             )),
//         }
//     }
//
//     async fn select_one(&self, id: i32) -> Result<User, UserRepositoryError> {
//         let user = sqlx::query_as::<_, User>("SELECT * FROM public.user WHERE id = $1")
//             .bind(id)
//             .fetch_one(&*self.pool)
//             .await;
//
//         match user {
//             Ok(user) => Ok(user),
//             _ => Err(UserRepositoryError::Message(
//                 "Failed to select user".to_owned(),
//             )),
//         }
//     }
//
//     async fn insert(
//         &self,
//         login: String,
//         hash: String,
//         salt: String,
//     ) -> Result<i32, UserRepositoryError> {
//         let result = sqlx::query_as::<_, ReturnedId>(
//             "INSERT INTO public.user VALUES (DEFAULT, $1, $2, $3) RETURNING id",
//         )
//         .bind(login)
//         .bind(hash)
//         .bind(salt)
//         .fetch_one(&*self.pool)
//         .await;
//
//         match result {
//             Ok(returned_id) => Ok(returned_id.id),
//             _ => Err(UserRepositoryError::Message(
//                 "Failed to insert user".to_owned(),
//             )),
//         }
//     }
//
//     async fn select_one_by_login(&self, login: String) -> Result<User, UserRepositoryError> {
//         let user = sqlx::query_as::<_, User>("SELECT * FROM public.user WHERE login = $1")
//             .bind(login)
//             .fetch_one(&*self.pool)
//             .await;
//
//         match user {
//             Ok(user) => Ok(user),
//             _ => Err(UserRepositoryError::Message(
//                 "Failed to select user".to_owned(),
//             )),
//         }
//     }
// }
