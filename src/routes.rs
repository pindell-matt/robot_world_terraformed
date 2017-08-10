use db::Conn as DbConn;
use rocket::response::{Flash, Redirect};
use rocket_contrib::Json;
use super::robot::{Robot, NewRobot};

#[get("/")]
fn index(conn: DbConn) -> Json {
    Json(json!(Robot::all(&conn)))
}

#[post("/", data = "<new_robot>")]
fn new(conn: DbConn, new_robot: Json<NewRobot>) -> Flash<Redirect> {
    Robot::insert(new_robot.into_inner(), &conn);
    Flash::success(Redirect::to("/robots"), "Robot successfully added")
}

#[get("/<id>")]
fn show(conn: DbConn, id: i32) -> Json {
    Json(json!(Robot::show(id, &conn)))
}

/// TODO: finish UPDATE func
// #[put("/robots/<id>", data = "<new_robot>")]
// fn update(conn: DbConn, id: i32, new_robot: Json<NewRobot>) -> Result<Redirect, ()> {
//     Ok(Redirect::to("/robots/<id>"))
// }

#[delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> Result<Redirect, ()> {
    Robot::delete_with_id(id, &conn);
    Ok(Redirect::to("/robots"))
}

#[get("/departments/<department>")]
fn department(department: String, conn: DbConn) -> Json {
    Json(json!(
        Robot::all_in_department(department, &conn)
    ))
}
