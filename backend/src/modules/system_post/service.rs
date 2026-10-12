use sqlx::{FromRow, Postgres, Transaction};

use crate::{
    common::error::{AppError, AppResult},
    modules::system_post::dto::{
        CreatePostRequest, PaginationMeta, PostDetailData, PostListData, PostListItem,
        PostListQuery, PostMutationData, PostStatusMutationData, UpdatePostRequest,
        UpdatePostStatusRequest,
    },
    state::AppState,
};

const SUPER_ADMIN_POST_CODE: &str = "SUPER_ADMIN";

#[derive(Debug, Clone, FromRow)]
struct PostListRow {
    id: i64,
    code: String,
    name: String,
    sort: i32,
    status: i16,
    user_count: i64,
    remark: Option<String>,
    created_at: String,
}

#[derive(Debug, Clone, FromRow)]
struct PostDetailRow {
    id: i64,
    code: String,
    name: String,
    sort: i32,
    status: i16,
    user_count: i64,
    remark: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, FromRow)]
struct PostIdentityRow {
    id: i64,
    code: String,
    name: String,
}

#[derive(Debug, Clone, FromRow)]
struct CountRow {
    total: i64,
}

#[derive(Debug, Clone, FromRow)]
struct NextIdRow {
    next_id: i64,
}

#[derive(Debug, Clone, FromRow)]
struct ExistsRow {
    exists_flag: bool,
}

pub async fn list_posts(state: &AppState, query: PostListQuery) -> AppResult<PostListData> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).clamp(1, 100);
    let offset = ((page - 1) * page_size) as i64;
    let keyword = query
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("%{value}%"));
    let status = query.status.map(validate_status).transpose()?;

    let total_row = sqlx::query_as::<_, CountRow>(
        r#"
        SELECT COUNT(*)::BIGINT AS total
        FROM sys_post p
        WHERE p.is_deleted = FALSE
          AND ($1::TEXT IS NULL
                OR p.post_name ILIKE $1
                OR p.post_code ILIKE $1
                OR COALESCE(p.remark, '') ILIKE $1)
          AND ($2::SMALLINT IS NULL OR p.status = $2)
        "#,
    )
    .bind(keyword.as_deref())
    .bind(status)
    .fetch_one(&state.db)
    .await?;

    let rows = sqlx::query_as::<_, PostListRow>(
        r#"
        SELECT
            p.id,
            p.post_code AS code,
            p.post_name AS name,
            p.sort_no AS sort,
            p.status,
            COALESCE(up.user_count, 0) AS user_count,
            p.remark,
            TO_CHAR(p.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at
        FROM sys_post p
        LEFT JOIN (
            SELECT up.post_id, COUNT(*)::BIGINT AS user_count
            FROM sys_user_post up
            INNER JOIN sys_user u ON u.id = up.user_id
            WHERE u.is_deleted = FALSE
            GROUP BY up.post_id
        ) up ON up.post_id = p.id
        WHERE p.is_deleted = FALSE
          AND ($1::TEXT IS NULL
                OR p.post_name ILIKE $1
                OR p.post_code ILIKE $1
                OR COALESCE(p.remark, '') ILIKE $1)
          AND ($2::SMALLINT IS NULL OR p.status = $2)
        ORDER BY p.sort_no, p.id
        OFFSET $3
        LIMIT $4
        "#,
    )
    .bind(keyword.as_deref())
    .bind(status)
    .bind(offset)
    .bind(page_size as i64)
    .fetch_all(&state.db)
    .await?;

    Ok(PostListData {
        items: rows.into_iter().map(map_post_list_item).collect(),
        pagination: PaginationMeta {
            page,
            page_size,
            total: total_row.total.max(0) as u64,
        },
    })
}

pub async fn get_post_detail(state: &AppState, post_id: i64) -> AppResult<PostDetailData> {
    let row = find_post_detail_row(state, post_id).await?;
    Ok(map_post_detail(row))
}

pub async fn create_post(
    state: &AppState,
    operator_user_id: i64,
    payload: CreatePostRequest,
) -> AppResult<PostMutationData> {
    let code = require_non_empty(payload.code, "code")?;
    let name = require_non_empty(payload.name, "name")?;
    let sort = validate_sort(payload.sort.unwrap_or(100))?;
    let status = validate_status(payload.status.unwrap_or(1))?;
    let remark = normalize_optional_field(payload.remark);

    let mut tx = state.db.begin().await?;
    ensure_unique_post_code(&mut tx, &code, None).await?;
    let new_post_id = next_post_id(&mut tx).await?;

    sqlx::query(
        r#"
        INSERT INTO sys_post (
            id,
            post_name,
            post_code,
            sort_no,
            status,
            created_at,
            updated_at,
            created_by,
            updated_by,
            is_deleted,
            remark
        ) VALUES (
            $1, $2, $3, $4, $5, NOW(), NOW(), $6, $6, FALSE, $7
        )
        "#,
    )
    .bind(new_post_id)
    .bind(&name)
    .bind(&code)
    .bind(sort)
    .bind(status)
    .bind(operator_user_id)
    .bind(remark)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(PostMutationData {
        id: new_post_id,
        code,
        name,
    })
}

pub async fn update_post(
    state: &AppState,
    operator_user_id: i64,
    post_id: i64,
    payload: UpdatePostRequest,
) -> AppResult<PostMutationData> {
    let mut tx = state.db.begin().await?;
    let current_post = find_post_identity(&mut tx, post_id)
        .await?
        .ok_or_else(|| AppError::not_found("post not found"))?;

    let name = payload
        .name
        .map(|value| require_non_empty(value, "name"))
        .transpose()?;
    let sort = payload.sort.map(validate_sort).transpose()?;
    let status = payload.status.map(validate_status).transpose()?;
    let remark = payload
        .remark
        .map(|value| normalize_optional_field(Some(value)));

    if name.is_none() && sort.is_none() && status.is_none() && remark.is_none() {
        return Err(AppError::bad_request(
            "at least one updatable field is required",
        ));
    }

    if current_post.code == SUPER_ADMIN_POST_CODE && status == Some(0) {
        return Err(AppError::forbidden("super admin post cannot be disabled"));
    }

    sqlx::query(
        r#"
        UPDATE sys_post
        SET post_name = COALESCE($2, post_name),
            sort_no = COALESCE($3, sort_no),
            status = COALESCE($4, status),
            remark = COALESCE($5, remark),
            updated_at = NOW(),
            updated_by = $6
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(post_id)
    .bind(name.as_deref())
    .bind(sort)
    .bind(status)
    .bind(remark.flatten())
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(PostMutationData {
        id: current_post.id,
        code: current_post.code,
        name: name.unwrap_or(current_post.name),
    })
}

pub async fn update_post_status(
    state: &AppState,
    operator_user_id: i64,
    post_id: i64,
    payload: UpdatePostStatusRequest,
) -> AppResult<PostStatusMutationData> {
    let status = validate_status(payload.status)?;
    let mut tx = state.db.begin().await?;
    let current_post = find_post_identity(&mut tx, post_id)
        .await?
        .ok_or_else(|| AppError::not_found("post not found"))?;

    if current_post.code == SUPER_ADMIN_POST_CODE && status == 0 {
        return Err(AppError::forbidden("super admin post cannot be disabled"));
    }

    sqlx::query(
        r#"
        UPDATE sys_post
        SET status = $2,
            updated_at = NOW(),
            updated_by = $3
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(post_id)
    .bind(status)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(PostStatusMutationData {
        id: current_post.id,
        code: current_post.code,
        name: current_post.name,
        status,
    })
}

pub async fn delete_post(
    state: &AppState,
    operator_user_id: i64,
    post_id: i64,
) -> AppResult<PostMutationData> {
    let mut tx = state.db.begin().await?;
    let current_post = find_post_identity(&mut tx, post_id)
        .await?
        .ok_or_else(|| AppError::not_found("post not found"))?;

    if current_post.code == SUPER_ADMIN_POST_CODE {
        return Err(AppError::forbidden("super admin post cannot be deleted"));
    }

    ensure_post_not_used(&mut tx, post_id).await?;

    sqlx::query(
        r#"
        UPDATE sys_post
        SET is_deleted = TRUE,
            updated_at = NOW(),
            updated_by = $2
        WHERE id = $1
          AND is_deleted = FALSE
        "#,
    )
    .bind(post_id)
    .bind(operator_user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(PostMutationData {
        id: current_post.id,
        code: current_post.code,
        name: current_post.name,
    })
}

async fn find_post_detail_row(state: &AppState, post_id: i64) -> AppResult<PostDetailRow> {
    sqlx::query_as::<_, PostDetailRow>(
        r#"
        SELECT
            p.id,
            p.post_code AS code,
            p.post_name AS name,
            p.sort_no AS sort,
            p.status,
            COALESCE((
                SELECT COUNT(*)::BIGINT
                FROM sys_user_post up
                INNER JOIN sys_user u ON u.id = up.user_id
                WHERE up.post_id = p.id
                  AND u.is_deleted = FALSE
            ), 0) AS user_count,
            p.remark,
            TO_CHAR(p.created_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS created_at,
            TO_CHAR(p.updated_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS updated_at
        FROM sys_post p
        WHERE p.id = $1
          AND p.is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(post_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::not_found("post not found"))
}

fn map_post_list_item(row: PostListRow) -> PostListItem {
    PostListItem {
        id: row.id,
        code: row.code,
        name: row.name,
        sort: row.sort,
        status: row.status,
        user_count: row.user_count,
        remark: row.remark,
        created_at: row.created_at,
    }
}

fn map_post_detail(row: PostDetailRow) -> PostDetailData {
    PostDetailData {
        id: row.id,
        code: row.code,
        name: row.name,
        sort: row.sort,
        status: row.status,
        user_count: row.user_count,
        remark: row.remark,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

async fn find_post_identity(
    tx: &mut Transaction<'_, Postgres>,
    post_id: i64,
) -> AppResult<Option<PostIdentityRow>> {
    sqlx::query_as::<_, PostIdentityRow>(
        r#"
        SELECT
            id,
            post_code AS code,
            post_name AS name
        FROM sys_post
        WHERE id = $1
          AND is_deleted = FALSE
        LIMIT 1
        "#,
    )
    .bind(post_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(Into::into)
}

async fn ensure_unique_post_code(
    tx: &mut Transaction<'_, Postgres>,
    code: &str,
    exclude_post_id: Option<i64>,
) -> AppResult<()> {
    let row = sqlx::query_as::<_, ExistsRow>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM sys_post
            WHERE post_code = $1
              AND ($2::BIGINT IS NULL OR id <> $2)
        ) AS exists_flag
        "#,
    )
    .bind(code)
    .bind(exclude_post_id)
    .fetch_one(&mut **tx)
    .await?;

    if row.exists_flag {
        return Err(AppError::conflict("post code already exists"));
    }

    Ok(())
}

async fn ensure_post_not_used(tx: &mut Transaction<'_, Postgres>, post_id: i64) -> AppResult<()> {
    let row = sqlx::query_as::<_, CountRow>(
        r#"
        SELECT COUNT(*)::BIGINT AS total
        FROM sys_user_post up
        INNER JOIN sys_user u ON u.id = up.user_id
        WHERE up.post_id = $1
          AND u.is_deleted = FALSE
        "#,
    )
    .bind(post_id)
    .fetch_one(&mut **tx)
    .await?;

    if row.total > 0 {
        return Err(AppError::conflict("post is assigned to users"));
    }

    Ok(())
}

async fn next_post_id(tx: &mut Transaction<'_, Postgres>) -> AppResult<i64> {
    let row = sqlx::query_as::<_, NextIdRow>(
        r#"
        SELECT COALESCE(MAX(id), 200) + 10 AS next_id
        FROM sys_post
        "#,
    )
    .fetch_one(&mut **tx)
    .await?;

    Ok(row.next_id)
}

fn require_non_empty(value: String, field_name: &str) -> AppResult<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::bad_request(format!("{field_name} is required")));
    }

    Ok(trimmed.to_string())
}

fn normalize_optional_field(value: Option<String>) -> Option<String> {
    value.and_then(|item| {
        let trimmed = item.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn validate_status(status: i16) -> AppResult<i16> {
    if status != 0 && status != 1 {
        return Err(AppError::bad_request("status must be 0 or 1"));
    }

    Ok(status)
}

fn validate_sort(sort: i32) -> AppResult<i32> {
    if sort < 0 {
        return Err(AppError::bad_request(
            "sort must be greater than or equal to 0",
        ));
    }

    Ok(sort)
}
