// @generated automatically by Diesel CLI.

diesel::table! {
    article_menus (article_id, menu_id) {
        article_id -> Int4,
        menu_id -> Int4,
    }
}

diesel::table! {
    article_metas (article_id) {
        article_id -> Int4,
        #[max_length = 255]
        key -> Nullable<Varchar>,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    articles (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        published_at -> Timestamp,
        author_id -> Nullable<Int4>,
    }
}


diesel::table! {
    menus (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    menu_items (id) {
        id -> Int4,
        menu_id -> Nullable<Int4>,
        #[max_length = 255]
        label -> Varchar,
        #[max_length = 255]
        link -> Varchar,
        position -> Int4,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    role_permissions (role_id, permission_id) {
        role_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        role_id -> Nullable<Int4>,
        session_token -> Nullable<Varchar>,
    }
}

diesel::joinable!(article_menus -> articles (article_id));
diesel::joinable!(article_menus -> menus (menu_id));
diesel::joinable!(article_metas -> articles (article_id));
diesel::joinable!(articles -> users (author_id));
diesel::joinable!(menu_items -> menus (menu_id));
diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    article_menus,
    article_metas,
    articles,
    menu_items,
    menus,
    permissions,
    role_permissions,
    roles,
    users,
);
