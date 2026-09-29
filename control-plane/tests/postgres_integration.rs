use gitgat_control_plane::state::AppState;
use sqlx::PgPool;
use uuid::Uuid;

async fn migrated_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("connect PostgreSQL");
    sqlx::migrate!().run(&pool).await.expect("run migrations");
    pool
}

#[tokio::test]
async fn migrations_and_readiness_query_work() {
    let pool = migrated_pool().await;
    let value = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("readiness query");
    assert_eq!(value, 1);

    let state = AppState::new(pool, None);
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM organizations")
        .fetch_one(&state.pool)
        .await
        .expect("organizations table");
    assert!(count >= 0);
}

#[tokio::test]
async fn tenant_memberships_do_not_cross_organizations() {
    let pool = migrated_pool().await;
    let suffix = Uuid::now_v7().to_string();
    let org_a = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO organizations (slug, name) VALUES ($1, $2) RETURNING id",
    )
    .bind(format!("tenant-a-{suffix}"))
    .bind("Tenant A")
    .fetch_one(&pool)
    .await
    .expect("org A");

    let org_b = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO organizations (slug, name) VALUES ($1, $2) RETURNING id",
    )
    .bind(format!("tenant-b-{suffix}"))
    .bind("Tenant B")
    .fetch_one(&pool)
    .await
    .expect("org B");

    let user = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (external_subject, display_name) VALUES ($1, $2) RETURNING id",
    )
    .bind(format!("subject-{suffix}"))
    .bind("Tenant User")
    .fetch_one(&pool)
    .await
    .expect("user");

    sqlx::query(
        "INSERT INTO memberships (organization_id, user_id, role) VALUES ($1, $2, 'developer')",
    )
    .bind(org_a)
    .bind(user)
    .execute(&pool)
    .await
    .expect("membership");

    let visible_a = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM memberships WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(org_a)
    .bind(user)
    .fetch_one(&pool)
    .await
    .expect("tenant A query");

    let visible_b = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM memberships WHERE organization_id = $1 AND user_id = $2",
    )
    .bind(org_b)
    .bind(user)
    .fetch_one(&pool)
    .await
    .expect("tenant B query");

    assert_eq!(visible_a, 1);
    assert_eq!(visible_b, 0);
}
