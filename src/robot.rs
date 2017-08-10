use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::robots;
use schema::robots::dsl::{robots as all_robots};

// Struct for pulling Robots out of the database.
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Robot {
    pub id: i32,
    pub name: String,
    pub avatar: String,
    pub department: String
}

/// investigate FromData option via Rocket
// Struct for creating / inserting new Robots into the database.
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "robots"]
pub struct NewRobot {
    pub name: String,
    pub avatar: String,
    pub department: String
}

impl Robot {
    pub fn show(id: i32, conn: &PgConnection) -> Vec<Robot> {
        all_robots.find(id).load::<Robot>(conn).unwrap()
    }

    pub fn all(conn: &PgConnection) -> Vec<Robot> {
        all_robots.order(robots::id.desc()).load::<Robot>(conn).unwrap()
    }

    pub fn insert(robot: NewRobot, conn: &PgConnection) -> bool {
        let NewRobot { name, avatar, department } = robot;
        diesel::insert(&NewRobot { name, avatar, department })
            .into(robots::table)
            .execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(all_robots.find(id)).execute(conn).is_ok()
    }

    pub fn all_in_department(department: String, conn: &PgConnection) -> Vec<Robot> {
        all_robots
            .filter(robots::department.eq(department))
            .load::<Robot>(conn)
            .unwrap()
    }

}
