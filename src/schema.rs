// @generated automatically by Diesel CLI.

diesel::table! {
    users (uuid) {
        uuid -> Int4,
        name -> Varchar,
        email -> Varchar,
        removed -> Bool,
    }
}
