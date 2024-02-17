// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        due_date -> Nullable<Date>,
        completed -> Bool,
    }
}
