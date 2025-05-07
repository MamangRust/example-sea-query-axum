use serde::Serialize;
use utoipa::ToSchema;

use crate::model::category::Category;

#[derive(Debug, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
}

impl From<Category> for CategoryResponse {
    fn from(category: Category) -> Self {
        CategoryResponse {
            id: category.id,
            name: category.name,
        }
    }
}
