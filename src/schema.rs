table! {
    heading (id) {
        id -> Int4,
        post -> Int4,
        section_index -> Int4,
        heading_text -> Text,
        heading_size -> Int4,
    }
}

table! {
    image (id) {
        id -> Int4,
        post -> Int4,
        section_index -> Int4,
        image_name -> Varchar,
        caption -> Varchar,
        file_name -> Varchar,
    }
}

table! {
    post (id) {
        id -> Int4,
        title -> Varchar,
        published -> Bool,
        blog -> Varchar,
    }
}

table! {
    text_section (id) {
        id -> Int4,
        post -> Int4,
        section_index -> Int4,
        section_text -> Text,
    }
}

joinable!(heading -> post (post));
joinable!(image -> post (post));
joinable!(text_section -> post (post));

allow_tables_to_appear_in_same_query!(
    heading,
    image,
    post,
    text_section,
);
