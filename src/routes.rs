use db::Conn as DbConn;
use rocket_contrib::{Json, Value};
use super::robot::{Robot, NewRobot};

#[get("/robots", format = "application/json")]
fn index(conn: DbConn) -> Json {
    Json(json!({
        "status": 200,
        "result": Robot::all(&conn)
    }))
}

#[post("/robots", format = "application/json", data = "<new_robot>")]
fn new(conn: DbConn, new_robot: Json<NewRobot>) -> Json<Value> {
    Json(json!({
        "status": Robot::insert(new_robot.into_inner(), &conn),
        "result": Robot::all(&conn).first()
    }))
}

#[get("/robots/<id>", format = "application/json")]
fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = Robot::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };
    Json(json!({
        "status": status,
        "result": result.get(0)
    }))
}

/// TODO: finish UPDATE func
// #[put("/robots/<id>", data = "<new_robot>")]
// fn update(conn: DbConn, id: i32, new_robot: Json<NewRobot>) -> Result<Redirect, ()> {
//     Ok(Redirect::to("/robots/<id>"))
// }

#[delete("/robots/<id>")]
fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if Robot::delete_with_id(id, &conn) { 204 } else { 404 };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/robots/departments/<department>", format = "application/json")]
fn department(department: String, conn: DbConn) -> Json {
    Json(json!(
        Robot::all_in_department(department, &conn)
    ))
}

#[error(404)]
fn not_found() -> Json {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}
