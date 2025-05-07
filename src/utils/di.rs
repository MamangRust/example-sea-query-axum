use std::sync::Arc;


use crate::{
    abstract_trait::{
        DynAuthService, DynCategoryRepository, DynCategoryService, DynCommentRepository,
        DynCommentService, DynFileService, DynPostsRepository, DynPostsService, DynUserRepository,
        DynUserService,
    },
    config::{ConnectionPool, Hashing, JwtConfig},
    repository::{CategoryRepository, CommentRepository, PostRepository, UserRepository},
    service::{
        AuthService, CategoryService, CommentService, FileService, PostService, UserService,
    },
};

#[derive(Clone)]
pub struct DependenciesInject {
    pub category_service: DynCategoryService,
    pub post_service: DynPostsService,
    pub comment_service: DynCommentService,
    pub user_service: DynUserService,
    pub auth_service: DynAuthService,
    pub file_service: DynFileService,
}

impl DependenciesInject {
    pub fn new(pool: ConnectionPool, hashing: Hashing, jwt_config: JwtConfig) -> Self {
        let category_repository =
            Arc::new(CategoryRepository::new(pool.clone())) as DynCategoryRepository;

        let category_service =
            Arc::new(CategoryService::new(category_repository)) as DynCategoryService;

        let post_repository = Arc::new(PostRepository::new(pool.clone())) as DynPostsRepository;

        let post_service = Arc::new(PostService::new(post_repository.clone())) as DynPostsService;

        let comment_repository =
            Arc::new(CommentRepository::new(pool.clone())) as DynCommentRepository;
        let comment_service =
            Arc::new(CommentService::new(comment_repository)) as DynCommentService;

        let user_repository = Arc::new(UserRepository::new(pool.clone())) as DynUserRepository;

        let user_service = Arc::new(UserService::new(user_repository.clone())) as DynUserService;

        let auth_service = Arc::new(AuthService::new(
            user_repository.clone(),
            hashing,
            jwt_config,
        ));

        let file_service = Arc::new(FileService::new()) as DynFileService;

        Self {
            category_service,
            post_service,
            comment_service,
            user_service,
            auth_service,
            file_service,
        }
    }
}
