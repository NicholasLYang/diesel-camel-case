table! {
    permissions (id) {
        id -> Int4,
        resource_name -> Nullable<Varchar>,
        action -> Nullable<Actiontype>,
        modifier -> Nullable<Actionmodifier>,
    }
}
