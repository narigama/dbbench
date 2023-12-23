use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};

pub async fn example(database_url: &str) -> eyre::Result<()> {
    // connect to the database
    let conn = sea_orm::Database::connect(
        sea_orm::ConnectOptions::new(database_url)
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info)
            .to_owned(),
    )
    .await?;

    // start a transaction.
    // if you don't call `conn.commit().await?` and it falls out of scope,
    // `conn.drop()` will rollback any changes made.
    let db = conn.begin().await?;

    // create a user
    let user = user_create(&db, "seaorm@narigama.dev", "tescovalue").await?;

    // create a new post and save it
    let post = post_create(
        &db,
        &user.clone(),
        "Hello World!",
        "Welcome to my new Blog!",
    )
    .await?;

    // commit these changes
    db.commit().await?;

    println!("SEAORM POST-COMMIT, {user:#?}");
    println!("SEAORM POST-COMMIT, {post:#?}");

    Ok(())
}

pub async fn user_create(
    db: &sea_orm::DatabaseTransaction,
    email: &str,
    password: &str,
) -> eyre::Result<crate::orm::user::ActiveModel> {
    let user = crate::orm::user::ActiveModel {
        email: sea_orm::ActiveValue::Set(email.into()),
        password: sea_orm::ActiveValue::Set(password.into()),
        ..Default::default()
    }
    .save(db)
    .await?;

    Ok(user)
}

pub async fn user_get_by_email(
    db: &sea_orm::DatabaseTransaction,
    email: &str,
) -> eyre::Result<Option<crate::orm::user::Model>> {
    Ok(crate::orm::user::Entity::find()
        .filter(crate::orm::user::Column::Email.eq(email))
        .one(db)
        .await?)
}

pub async fn post_create(
    db: &sea_orm::DatabaseTransaction,
    user: &crate::orm::user::ActiveModel,
    title: &str,
    body: &str,
) -> eyre::Result<crate::orm::post::ActiveModel> {
    // TODO: is there a more idiomatic method of unpacking the user.id safely?
    eyre::ensure!(user.id.is_unchanged());
    let user_id = user.id.clone().unwrap();

    let post = crate::orm::post::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(user_id),
        title: sea_orm::ActiveValue::Set(title.into()),
        body: sea_orm::ActiveValue::Set(body.into()),
        ..Default::default()
    }
    .save(db)
    .await?;

    Ok(post)
}
