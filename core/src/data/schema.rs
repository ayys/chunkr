diesel::table! {
    tasks (task_id) {
        task_id -> Text,
        file_name -> Nullable<Text>,
        file_size -> Nullable<Int8>,
        page_count -> Nullable<Int4>,
        segment_count -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
        expires_at -> Nullable<Timestamptz>,
        finished_at -> Nullable<Timestamptz>,
        status -> Nullable<Text>,
        task_url -> Nullable<Text>,
        input_location -> Nullable<Text>,
        output_location -> Nullable<Text>,
        configuration -> Nullable<Text>,
        message -> Nullable<Text>,
        pdf_location -> Nullable<Text>,
        input_file_type -> Nullable<Text>,
        #[max_length = 255]
        mime_type -> Nullable<Varchar>,
        started_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        image_folder_location -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(tasks);
