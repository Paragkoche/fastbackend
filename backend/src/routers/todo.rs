use crate::db::db::connection;
use crate::db::model::tobo::Todo;
use crate::db::schema::todos;
use diesel::RunQueryDsl;
use rocket::response::Debug;
use rocket_contrib::json;
use rocket_contrib::json::Json;
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[get("/todo")]
pub fn getAllTodo() {
    let db = &mut connection();
    Json(json!(todos::dsl::todos.load::<Todo>(db).expect("msg")));
}
