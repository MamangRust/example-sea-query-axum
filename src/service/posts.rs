use crate::{
    abstract_trait::{DynPostsRepository, PostsServiceTrait},
    domain::{
        ApiResponse, ApiResponsePagination, CreatePostRequest, ErrorResponse, FindAllPostRequest,
        Pagination, PostRelationResponse, PostResponse, UpdatePostRequest,
    },
    utils::AppError,
};
use async_trait::async_trait;
use tracing::{info, error};

pub struct PostService {
    repository: DynPostsRepository,
}

impl PostService {
    pub fn new(repository: DynPostsRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl PostsServiceTrait for PostService {
    async fn get_all_posts(
        &self,
        req: FindAllPostRequest,
    ) -> Result<ApiResponsePagination<Vec<PostResponse>>, ErrorResponse> {
        let page = req.page.max(1);
        let page_size = req.page_size.max(1);
        let search = if req.search.is_empty() {
            None
        } else {
            Some(req.search.clone())
        };

        let (posts, total_items) = self
            .repository
            .get_all_posts(page, page_size, search)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let responses: Vec<PostResponse> = posts.into_iter().map(PostResponse::from).collect();

        let total_pages = (total_items as f64 / req.page_size as f64).ceil() as i32;

        Ok(ApiResponsePagination {
            status: "success".to_string(),
            message: "Posts retrieved successfully".to_string(),
            data: responses,
            pagination: Pagination {
                page: req.page,
                page_size: req.page_size,
                total_items,
                total_pages,
            },
        })
    }

    async fn get_post(
        &self,
        post_id: i32,
    ) -> Result<Option<ApiResponse<PostResponse>>, ErrorResponse> {
        let post = self
            .repository
            .get_post(post_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        if let Some(post) = post {
            Ok(Some(ApiResponse {
                status: "success".to_string(),
                message: "Post retrieved successfully".to_string(),
                data: PostResponse::from(post),
            }))
        } else {
            Err(ErrorResponse::from(AppError::NotFound(format!(
                "Posts with id {} not found",
                post_id
            ))))
        }
    }

    async fn get_post_relation(
        &self,
        post_id: i32,
    ) -> Result<ApiResponse<PostRelationResponse>, ErrorResponse> {
        let relations = self
            .repository
            .get_post_relation(post_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let first_relation = relations
            .into_iter()
            .next()
            .ok_or_else(|| AppError::NotFound("Post relation not found".to_string()))?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Post relation retrieved successfully".to_string(),
            data: first_relation,
        })
    }

    async fn create_post(
        &self,
        input: &CreatePostRequest,
    ) -> Result<ApiResponse<PostResponse>, ErrorResponse> {
        let post = self
            .repository
            .create_post(input)
            .await
            .map_err(|e| {
                error!("Failed to create post: {}", e);
                AppError::from(e)
            })
            .map_err(|e| ErrorResponse::from(e))?;
    
        info!("Post created successfully with title: {}", input.title);
    
        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Post created successfully".to_string(),
            data: PostResponse::from(post),
        })
    }
    

    async fn update_post(
        &self,
        input: &UpdatePostRequest,
    ) -> Result<ApiResponse<PostResponse>, ErrorResponse> {
        let post = self
            .repository
            .update_post(input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Post updated successfully".to_string(),
            data: PostResponse::from(post),
        })
    }

    async fn delete_post(&self, post_id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        self.repository
            .delete_post(post_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Post deleted successfully".to_string(),
            data: (),
        })
    }
}
