use super::prelude::*;
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user)
        .service(show_users)
        .service(update_user)
        .service(delete_user)
        .service(login_user)
        .service(logout_user)
        .service(check_user);
}

#[post("/api/user/create")]
async fn create_user(
    web::Json(user): web::Json<model::QueryNewUser>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::{user, user_auth};

    let validate_result: Result<(), ValidationErrors> = user.validate();
    if validate_result.is_err() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<()>>::into(validate_result));
    }

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<()>>::into(
            web::block::<_, _, diesel::result::Error>(move || {
                conn.transaction(|| {
                    diesel::insert_into(user::table)
                        .values(&(Into::<model::NewUser>::into(&user)))
                        .execute(&conn)?;

                    let last_id = diesel::select(db::last_insert_id).get_result::<u64>(&conn)?;

                    let user_auths: model::NewUserAuths =
                        From::<(model::QueryNewUser, u64)>::from((user, last_id));

                    diesel::insert_into(user_auth::table)
                        .values(&user_auths.0[..])
                        .execute(&conn)?;
                    Ok(())
                })
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/user/list")]
async fn show_users(
    web::Json(query_params): web::Json<model::QueryParamsUser>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(
            Into::<UnifiedResponseMessages<PaginateData<model::User>>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    let query_builder = model::UserQueryBuilder::query_all_columns();

                    let users = query_params
                        .clone()
                        .query_filter(query_builder)
                        .paginate(query_params.page)
                        .set_per_page(query_params.per_page)
                        .load::<model::User>(&conn)?;

                    let per_page = query_params.per_page;
                    let count_builder = model::UserQueryBuilder::query_count();
                    let count = query_params
                        .query_filter(count_builder)
                        .get_result::<i64>(&conn)?;

                    Ok(PaginateData::<model::User>::default()
                        .set_data_source(users)
                        .set_page_size(per_page)
                        .set_total(count))
                })
                .await,
            ),
        );
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<PaginateData<model::User>>::error())
}

#[post("/api/user/update")]
async fn update_user(
    web::Json(user_value): web::Json<model::UpdateUser>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || diesel::update(&user_value).set(&user_value).execute(&conn)).await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}

#[post("/api/user/delete")]
async fn delete_user(
    web::Json(model::UserId { user_id }): web::Json<model::UserId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::{user, user_auth};

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<()>>::into(
            web::block::<_, _, diesel::result::Error>(move || {
                conn.transaction(|| {
                    diesel::delete(user::table.filter(user::id.eq(user_id))).execute(&conn)?;
                    diesel::delete(user_auth::table.filter(user_auth::user_id.eq(user_id)))
                        .execute(&conn)?;
                    Ok(())
                })
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/user/login")]
async fn login_user(
    web::Json(user_login): web::Json<model::UserAuthLogin>,
    session: Session,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let login_result: UnifiedResponseMessages<()> =
        pre_login_user(user_login, session, pool).await.into();

    HttpResponse::Ok().json(login_result)
}

async fn pre_login_user(
    model::UserAuthLogin {
        login_type,
        account,
        password,
    }: model::UserAuthLogin,
    session: Session,
    pool: ShareData<db::ConnectionPool>,
) -> Result<(), CommonError> {
    use model::schema::{user, user_auth};
    use model::user::get_encrypted_certificate_by_raw_certificate;

    let conn = pool.get()?;
    let user_package: (model::UserAuth, model::User) =
        web::block::<_, _, diesel::result::Error>(move || {
            user_auth::table
                .inner_join(user::table)
                .select((user_auth::all_columns, user::all_columns))
                .filter(user_auth::identity_type.eq(login_type))
                .filter(user_auth::identifier.eq(account))
                .filter(
                    user_auth::certificate
                        .eq(get_encrypted_certificate_by_raw_certificate(&password)),
                )
                .first::<(model::UserAuth, model::User)>(&conn)
        })
        .await?;
    save_session(session, user_package)
}

fn save_session(
    session: Session,
    (_, user): (model::UserAuth, model::User),
) -> Result<(), CommonError> {
    session.set("login_time", get_timestamp())?;
    session.set("user_id", user.id)?;
    session.set("user_name", user.user_name)?;
    session.set("nick_name", user.nick_name)?;
    Ok(())
}

#[post("/api/user/check")]
async fn check_user(session: Session, pool: ShareData<db::ConnectionPool>) -> HttpResponse {
    let check_result = pre_check_user(session, pool).await;
    if let Ok(user) = check_result {
        return HttpResponse::Ok().json(UnifiedResponseMessages::<model::User>::success_with_data(
            user,
        ));
    };

    HttpResponse::Ok().json(
        UnifiedResponseMessages::<()>::error()
            .customized_error_msg(check_result.expect_err("").to_string()),
    )
}

async fn pre_check_user(
    session: Session,
    pool: ShareData<db::ConnectionPool>,
) -> Result<model::User, CommonError> {
    use model::schema::user;

    let conn = pool.get()?;
    let user_id = session
        .get::<u64>("user_id")?
        .ok_or_else(|| CommonError::DisPass("Without set `user_id` .".into()))?;

    let user = web::block::<_, _, diesel::result::Error>(move || {
        let user = user::table
            .select(user::all_columns)
            .find(user_id)
            .first::<model::User>(&conn)?;

        Ok(user)
    })
    .await?;

    Ok(user)
}

#[post("/api/user/logout")]
async fn logout_user(session: Session) -> HttpResponse {
    HttpResponse::Ok().json({
        session.clear();
        UnifiedResponseMessages::<()>::success()
    })
}
