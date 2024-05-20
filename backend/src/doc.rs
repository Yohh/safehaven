use crate::{
    api::{
        admin::{self, users::ChangePassword, LoginRequest, LoginResponse},
        map::{
            self, FetchedEntity, NewCommentRequest, NewEntityRequest, SearchRequest, ViewRequest,
        },
        root::{self, BootstrapResponse, SafeMode, StatusResponse},
        ErrorResponse,
    },
    models::{
        access_token::{AccessToken, NewOrUpdateAccessToken, PermissionPolicy, Permissions},
        category::{Category, NewCategory, UpdateCategory},
        comment::{Comment, ListedComment, NewComment, PublicComment, UpdateComment},
        entity::{
            Entity, ListedEntity, NewEntity, PublicEntity, UnprocessedLocation, UpdateEntity,
        },
        entity_cache::{CachedEntity, Cluster, EntitiesAndClusters},
        family::{Family, Field, FieldType, Form, NewOrUpdateFamily},
        options::{
            CartographyClusterConfig, CartographyInitConfig, ConfigurationOption, GeneralOptions,
            SafeHavenOptions, SafeModeConfig,
        },
        tag::{NewOrUpdateTag, Tag},
        user::{NewUser, User},
    },
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        root::status,
        root::boostrap,
        // map
        map::viewer_view_request,
        map::viewer_search_request,
        map::viewer_fetch_entity,
        map::viewer_new_comment,
        map::viewer_new_entity,
        // admin
        admin::admin_login,
        admin::admin_logout,
        // admin::options
        admin::options::admin_options_get,
        admin::options::admin_options_update,
        // admin::users
        admin::users::admin_users_list,
        admin::users::admin_user_new,
        admin::users::admin_user_get,
        admin::users::admin_user_change_self_password,
        admin::users::admin_user_change_password,
        admin::users::admin_user_delete,
        // admin::access_tokens
        admin::access_tokens::admin_access_tokens_list,
        admin::access_tokens::admin_access_token_new,
        admin::access_tokens::admin_access_token_get,
        admin::access_tokens::admin_access_token_update,
        admin::access_tokens::admin_access_token_delete,
        // admin::families
        admin::families::admin_families_list,
        admin::families::admin_family_new,
        admin::families::admin_family_get,
        admin::families::admin_family_update,
        admin::families::admin_family_delete,
        // admin::categories
        admin::categories::admin_categories_list,
        admin::categories::admin_category_new,
        admin::categories::admin_category_get,
        admin::categories::admin_category_update,
        admin::categories::admin_category_delete,
        // admin::tags
        admin::tags::admin_tags_list,
        admin::tags::admin_tag_new,
        admin::tags::admin_tag_get,
        admin::tags::admin_tag_update,
        admin::tags::admin_tag_delete,
        // admin::entities
        admin::entities::admin_entities_pending,
        admin::entities::admin_entities_search,
        admin::entities::admin_entity_new,
        admin::entities::admin_entity_get,
        admin::entities::admin_entity_update,
        admin::entities::admin_entity_delete,
        admin::entities::admin_entity_get_comments,
        admin::entities::admin_entity_register_parent,
        admin::entities::admin_entity_remove_parent,
        // admin::comments
        admin::comments::admin_comments_pending,
        admin::comments::admin_comment_new,
        admin::comments::admin_comment_get,
        admin::comments::admin_comment_update,
        admin::comments::admin_comment_delete,    ),
    components(schemas(
        // general
        ErrorResponse,
        // root
        StatusResponse,
        SafeMode,
        BootstrapResponse,
        // options
        SafeHavenOptions,
        ConfigurationOption,
        GeneralOptions,
        SafeModeConfig,
        CartographyInitConfig,
        CartographyClusterConfig,
        // families
        Family,
        NewOrUpdateFamily,
        Form,
        Field,
        FieldType,
        // categories
        Category,
        NewCategory,
        UpdateCategory,
        // tags
        Tag,
        NewOrUpdateTag,
        // entities
        Entity,
        NewEntity,
        UpdateEntity,
        ListedEntity,
        PublicEntity,
        CachedEntity,
        Cluster,
        EntitiesAndClusters,
        UnprocessedLocation,
        // comments
        Comment,
        NewComment,
        UpdateComment,
        ListedComment,
        PublicComment,
        // access_tokens
        AccessToken,
        NewOrUpdateAccessToken,
        Permissions,
        PermissionPolicy,
        // users
        NewUser,
        User,
        ChangePassword,
        LoginRequest,
        LoginResponse,
        // map
        ViewRequest,
        SearchRequest,
        NewCommentRequest,
        NewEntityRequest,
        FetchedEntity,
    ))
)]
pub struct ApiDoc {}
