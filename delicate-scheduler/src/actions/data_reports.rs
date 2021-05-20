use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_one_day_tasks_state);
}

#[get("/api/tasks_state/one_day")]
async fn show_one_day_tasks_state(pool: ShareData<db::ConnectionPool>) -> HttpResponse {
    use db::schema::task_log;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(
            Into::<UnifiedResponseMessages<model::PaginateTask>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    let query_builder = model::TaskQueryBuilder::query_all_columns();

                    let now = NaiveDateTime::from_timestamp(get_timestamp() as i64, 0);
                    let past_day = now - ChronoDuration::days(1);

                    // the number of tasks started in a given hour, the
                    // the number of tasks that ended normally at a certain time
                    // the number of tasks that ended abnormally at a certain time
                    // the number of tasks that timed out at a certain time
                    // Number of manually cancelled tasks at a certain time
                    // Update to add a limit so that only tasks with Running status can be modified.

                    // That's is ok.
                    // task_log::table
                    //     .select(
                    //         diesel::dsl::count(task_log::id),
                    //     )
                    //     .filter(task_log::created_time.between(past_day, now));

                    // Fixby: https://github.com/diesel-rs/diesel/issues/1781.
                    let task_state = task_log::table
                        .select(&(
                            (task_log::created_time, task_log::status),
                            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(task_log.id)")
                        ))
                        .filter(task_log::created_time.between(past_day, now));
                        // .group_by((task_log::created_time, task_log::status));

                    todo!();
                })
                .await,
            ),
        );
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<model::PaginateTask>::error())
}
