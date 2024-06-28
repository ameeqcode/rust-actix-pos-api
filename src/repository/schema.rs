// @generated automatically by Diesel CLI.

diesel::table! {
    pos_items (id) {
        id -> Int4,
        #[max_length = 255]
        code -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        category -> Nullable<Varchar>,
        #[max_length = 255]
        sub_category -> Nullable<Varchar>,
    }
}
