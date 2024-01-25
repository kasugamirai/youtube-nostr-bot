use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::videos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Videos {
    pub id: i32,
    pub author: String,
    pub channel: String,
    pub title: String,
    pub link: String,
    pub published: bool,
    pub userid: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::videos)]
pub struct NewVideos {
    pub author: String,
    pub channel: String,
    pub title: String,
    pub link: String,
    pub published: bool,
    pub userid: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::youtube_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct YoutubeUser {
    pub id: i32,
    pub username: String,
    pub publickey: String,
    pub privatekey: String,
    pub channel: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::youtube_users)]
pub struct NewYoutubeUser {
    pub username: String,
    pub publickey: String,
    pub privatekey: String,
    pub channel: String,
}
