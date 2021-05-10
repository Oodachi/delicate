use super::prelude::*;
use super::schema::{task};


#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]

pub struct Task {
    id: i64,
    name: String,
    description: String,
    command: String,
    frequency: String,
    cron_expression: String,
    timeout: i16,
    retry_times: i16,
    retry_interval: i16,
    maximun_parallel_runable_num: i16,
    tag: String,
    status: i16,
    created_time: NaiveDateTime,
    deleted_time: Option<NaiveDateTime>,
}

#[derive(Insertable, Identifiable, AsChangeset, Debug, Default, Serialize, Deserialize)]
#[table_name = "task"]
pub struct NewTask {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) command: String,
    pub(crate) frequency: String,
    pub(crate) cron_expression: String,
    pub(crate) timeout: i16,
    pub(crate) retry_times: i16,
    pub(crate) retry_interval: i16,
    pub(crate) maximun_parallel_runable_num: i16,
    pub(crate) tag: String,
    pub(crate) status: i16,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsTask {
    id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    frequency: Option<String>,
    cron_expression: Option<String>,
    tag: Option<String>,
    status: Option<i16>,
    pub(crate) per_page : i64,
    pub(crate) page : i64,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct PaginateTask{
    tasks : Vec<Task>,
    per_page : i64,
    total_page : i64
}

impl PaginateTask{
    pub(crate) fn set_tasks(mut self, tasks:Vec<Task>) ->Self{
        self.tasks = tasks;
        self
    }

    pub(crate) fn set_per_page(mut self, per_page:i64) ->Self{
        self.per_page = per_page;
        self
    }

    pub(crate) fn set_total_page(mut self, total:i64) ->Self{
        self.total_page = (total as f64 / self.per_page as f64).ceil() as i64;
        self
    }
}

pub(crate) struct TaskQueryBuilder;
impl TaskQueryBuilder{
    pub(crate) fn query_all_columns() -> task::BoxedQuery<'static, Mysql>{
        task::table
        .into_boxed()
        .select(task::all_columns)
    }

    pub(crate) fn query_count()-> task::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint>{
        task::table
        .into_boxed()
        .count()
    }
}


impl QueryParamsTask {

    pub(crate) fn query_filter<ST>(self, mut statement_builder : task::BoxedQuery<'static, Mysql, ST>) -> task::BoxedQuery<'static, Mysql, ST> {
        statement_builder = statement_builder
            .filter(task::status.ne(2));
            // Maybe status 2 eq task-deleted status.

        if let Some(task_id) = self.id {
            statement_builder = statement_builder.filter(task::id.eq(task_id));
        }

        if let Some(task_status) = self.status {
            statement_builder = statement_builder.filter(task::status.eq(task_status));
        } else {
            //TODO: Addtion state in future.
            statement_builder = statement_builder.filter(task::status.ne(2));
        }

        if let Some(task_name) = self.name {
            statement_builder = statement_builder.filter(task::name.like(task_name));
        }

        if let Some(task_description) = self.description {
            statement_builder = statement_builder.filter(task::description.like(task_description));
        }

        if let Some(task_command) = self.command {
            statement_builder = statement_builder.filter(task::command.like(task_command));
        }

        if let Some(task_frequency) = self.frequency {
            statement_builder = statement_builder.filter(task::frequency.like(task_frequency));
        }

        if let Some(task_cron_expression) = self.cron_expression {
            statement_builder =
                statement_builder.filter(task::cron_expression.like(task_cron_expression));
        }

        if let Some(task_tag) = self.tag {
            statement_builder = statement_builder.filter(task::tag.like(task_tag));
        }

        statement_builder.order(task::id.desc())

    }
}