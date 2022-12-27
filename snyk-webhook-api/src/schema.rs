// @generated automatically by Diesel CLI.

diesel::table! {
    snyk_events (id) {
        id -> Int4,
        ts -> Timestamp,
        org -> Jsonb,
        project -> Jsonb,
        new_issues -> Array<Nullable<Jsonb>>,
        removed_issues -> Array<Nullable<Jsonb>>,
    }
}
