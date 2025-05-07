use crate::{
    abstract_trait::{CategoryServiceTrait, DynCategoryRepository},
    domain::{
        ApiResponse, ApiResponsePagination, CategoryResponse, CreateCategoryRequest, ErrorResponse,
        FindAllCategoryRequest, Pagination, UpdateCategoryRequest,
    },
    utils::AppError,
};
use async_trait::async_trait;
use tracing::info;

pub struct CategoryService {
    repository: DynCategoryRepository,
}

impl CategoryService {
    pub fn new(repository: DynCategoryRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CategoryServiceTrait for CategoryService {
    async fn get_categories(
        &self,
        req: FindAllCategoryRequest,
    ) -> Result<ApiResponsePagination<Vec<CategoryResponse>>, ErrorResponse> {
        let page = if req.page > 0 { req.page } else { 1 };
        let page_size = if req.page_size > 0 { req.page_size } else { 10 };
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let (categories, total_items) = self
            .repository
            .find_all(page, page_size, search)
            .await
            .map_err(|e| {
                tracing::error!("Repository error: {}", e);
                AppError::from(e)
            })
            .map_err(|e| {
                tracing::error!("Repository error: {}", e);
                ErrorResponse::from(e)
            })?;

        info!("Found {} categories", categories.len());

        let total_pages = (total_items as f64 / page_size as f64).ceil() as i32;

        let category_responses: Vec<CategoryResponse> =
            categories.into_iter().map(CategoryResponse::from).collect();

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Categories retrieved successfully".to_string(),
            data: category_responses,
            pagination: Pagination {
                page,
                page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn get_category(
        &self,
        id: i32,
    ) -> Result<Option<ApiResponse<CategoryResponse>>, ErrorResponse> {
        let category = self
            .repository
            .find_by_id(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        if let Some(category) = category {
            Ok(Some(ApiResponse {
                status: "success".to_string(),
                message: "Category retrieved successfully".to_string(),
                data: CategoryResponse::from(category),
            }))
        } else {
            Err(ErrorResponse::from(AppError::NotFound(format!(
                "Category with id {} not found",
                id
            ))))
        }
    }

    async fn create_category(
        &self,
        input: &CreateCategoryRequest,
    ) -> Result<ApiResponse<CategoryResponse>, ErrorResponse> {
        let category = self
            .repository
            .create(input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        info!("Category created: {:#?}", category);

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Category created successfully".to_string(),
            data: CategoryResponse::from(category),
        })
    }

    async fn update_category(
        &self,
        input: &UpdateCategoryRequest,
    ) -> Result<Option<ApiResponse<CategoryResponse>>, ErrorResponse> {
        let category = self
            .repository
            .update(input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        Ok(Some(ApiResponse {
            status: "success".to_string(),
            message: "Category updated successfully".to_string(),
            data: CategoryResponse::from(category),
        }))
    }

    async fn delete_category(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        self.repository
            .delete(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Category deleted successfully".to_string(),
            data: (),
        })
    }
}
