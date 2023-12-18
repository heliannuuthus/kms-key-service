use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use serde_json::json;

use crate::{
    common::{
        axum::{Json, Query},
        datasource::Paginator,
        errors::Result,
    },
    pojo::form::key::{KeyCreateForm, KeyImportForm, KeyImportParamsQuery},
    service::key_service,
    States,
};

#[utoipa::path(
    post,
    path="",
    operation_id = "创建密钥",
    context_path= "/keys",
    responses(
        (status = 200, description = "密钥创建结果", body = KeyCreateResult, content_type="application/json"),
        (status = 400, description = "illegal params")
    ),
    request_body = KeyCreateForm
)]
pub async fn create_key(
    State(States { db, rd, extra, .. }): State<States>,
    Json(form): Json<KeyCreateForm>,
) -> Result<impl IntoResponse> {
    tracing::info!("create master key, data: {:?}", form);

    key_service::create_key(&rd, &db, extra.re, &form)
        .await
        .map(axum::Json)
}

#[utoipa::path(
    get,
    path="/import/params",
    operation_id = "导入密钥材料所需的参数",
    context_path= "/keys",
    params(KeyImportParamsQuery),
    responses(
        (status = 200, description = "", body = KeyMaterialImportParamsResult),
        (status = 400, description = "illegal params")
    ),
)]
pub async fn import_key_params(
    State(States { db, rd, .. }): State<States>,
    Query(form): Query<KeyImportParamsQuery>,
) -> Result<impl IntoResponse> {
    tracing::info!("create import key material, data: {:?}", form);
    key_service::generate_key_import_params(&rd, &db, &form)
        .await
        .map(axum::Json)
}

#[utoipa::path(
    post,
    path="/import",
    operation_id = "导入密钥材料",
    context_path= "/keys",
    request_body = KeyImportForm,
    responses(
        (status = 200, description = "", body = String),
        (status = 400, description = "illegal params")
    ),
)]
#[axum::debug_handler]
pub async fn import_key(
    State(States { db, rd, .. }): State<States>,
    Json(form): Json<KeyImportForm>,
) -> Result<impl IntoResponse> {
    tracing::info!("import key material, data: {:?}", form);
    key_service::import_key_material(&rd, &db, &form)
        .await
        .map(|_| axum::Json(json!({"key_id": form.key_id})))
}

#[utoipa::path(
    get,
    path="",
    operation_id = "分页查询 kms 实例所有密钥列表",
    context_path= "/kms/{kms_id}/keys",
    params(
        ("kms_id" = String, Path, description="kms 标识"),
        Paginator
    ),
    responses(
        (status = 200, description = "", body = ()),
        (status = 400, description = "illegal params")
    ),
  )]
pub async fn list_kms_keys(
    State(States { db, .. }): State<States>,
    Path(kms_id): Path<String>,
    Query(paginator): Query<Paginator>,
) -> Result<impl IntoResponse> {
    tracing::info!(
        "pagin kms key, kms_id: {}, paginator: {:?}",
        kms_id,
        paginator
    );

    key_service::list_kms_keys(&db, &kms_id, &paginator)
        .await
        .map(axum::Json)
}

#[utoipa::path(
    post,
    path="/versions",
    operation_id = "新增密钥版本",
    context_path= "/keys/{key_id}",
    params(
        ("key_id" = String, Path, description="密钥标识"),
    ),
    responses(
        (status = 200, description = "密钥新版本信息", body = KeyVersionResult),
        (status = 400, description = "illegal params")
    ),
  )]
pub async fn create_key_version(
    State(States { db, rd, extra, .. }): State<States>,
    Path(key_id): Path<String>,
) -> Result<impl IntoResponse> {
    tracing::info!("create key version, key_id: {}", key_id);
    key_service::create_key_version(&rd, &db, &extra.re, &key_id)
        .await
        .map(axum::Json)
}

#[utoipa::path(
    get,
    path="",
    operation_id = "分页查询密钥版本信息",
    context_path= "/keys/{key_id}/versions",
    params(
        ("key_id" = String, Path, description="kms 标识"),
        Paginator
      ),
    responses(
        (status = 200, description = "", body = ()),
        (status = 400, description = "illegal params")
    ),
  )]
pub async fn list_key_version(
    State(States { db, .. }): State<States>,
    Path(key_id): Path<String>,
    Query(paginator): Query<Paginator>,
) -> Result<impl IntoResponse> {
    tracing::info!(
        "pagin key meta, key_id: {}, paginator: {:?}",
        key_id,
        paginator
    );

    key_service::list_key_versions(&db, &key_id, &paginator)
        .await
        .map(axum::Json)
}
