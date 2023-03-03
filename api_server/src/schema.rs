// @generated automatically by Diesel CLI.

diesel::table! {
    encounters (id) {
        id -> Int4,
        name -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
