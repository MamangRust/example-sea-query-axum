use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Clone, ToSchema)]
pub struct Pagination {
    pub page: i32,
    pub page_size: i32,
    pub total_items: i64,
    pub total_pages: i32,
}
