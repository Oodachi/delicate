use super::prelude::*;
use super::schema::{task_bind};

#[derive(Queryable, AsChangeset, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "task_bind"]

pub struct TaskBind {
    id: i64,
    task_id: i64,
    bind_id: i64,
    created_time: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "task_bind"]
pub struct NewTaskBind {
    task_id: i64,
    bind_id: i64,
    created_time: NaiveDateTime,
}