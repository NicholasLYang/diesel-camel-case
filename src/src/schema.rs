table! {
    permissions (id) {
        id -> Int4,
        resource_name -> Nullable<Varchar>,
        action -> Nullable<Action_type>,
        modifier -> Nullable<Action_modifier>,
    }
}
