use diesel::prelude::*;
use crate::models::{Videos, NewVideos, YoutubeUser, NewYoutubeUser,Config};
use diesel::result::Error;
use std::fs::File;
use std::io::BufReader;


pub struct DbConnection {
    conn: PgConnection,
    videos: Vec<NewVideos>,
    youtube_users: Vec<NewYoutubeUser>,
}

impl DbConnection {
    pub fn new() -> DbConnection {
        let file = File::open("./conf/test/config.yaml").expect("Failed to open config file");
        let reader = BufReader::new(file);
        let config: Config = serde_yaml::from_reader(reader).expect("Failed to read config");
        let conn = PgConnection::establish(&config.dsn).expect(&format!("Error connecting to {}", config.dsn));

        DbConnection {
            conn: conn,
            videos: Vec::new(),
            youtube_users: Vec::new(),
        }
    }

    pub fn query_channel_id(&mut self, ch: &str) -> Option<String> {
        use crate::schema::youtube_users::dsl::*;
        let results = youtube_users.filter(channel.eq(ch)).load::<YoutubeUser>(&mut self.conn).expect("Error loading users");
        results.first().map(|user| user.channel_id.to_string())
    }
    
    pub fn video_exists(&mut self, lk: &str) -> bool {
        use crate::schema::videos::dsl::*;
        let results = videos.filter(link.eq(lk)).load::<Videos>(&mut self.conn).expect("Error loading videos");
        results.len() > 0
    }

    pub fn channel_exists(&mut self, ch: &str) -> bool {
        use crate::schema::youtube_users::dsl::*;
        let results = youtube_users.filter(channel.eq(ch)).load::<YoutubeUser>(&mut self.conn).expect("Error loading users");
        !results.is_empty()
    }

    pub fn add_user(&mut self, un: String, av:String, pk: String, prk: String, ch: String, chid: String) -> Result<(), Error> {
        use crate::schema::youtube_users::dsl::*;
    
        let new_user = NewYoutubeUser {
            username: un,
            avatar: Some(av),
            publickey: pk,
            privatekey: prk,
            channel: ch,
            channel_id: chid,
        };
    
        diesel::insert_into(youtube_users)
            .values(&new_user)
            .execute(&mut self.conn)
            .map_err(|err| {
                eprintln!("Error adding user: {}", err);
                err
            })
            .map(|_| ())
    }
        
    pub fn query_user_id(&mut self, ch: &str) -> Option<i32> {
        use crate::schema::youtube_users::dsl::*;
        let results = youtube_users.filter(channel.eq(ch)).load::<YoutubeUser>(&mut self.conn).expect("Error loading users");
        results.first().map(|user| user.id)
    }

    pub fn add_video(&mut self, au: String, ch: String, ti: String, lk: String, pu: bool) -> Result<(), Error> {
        use crate::schema::videos::dsl::*;
    
        let u = self.query_user_id(&ch).expect("User should exist at this point");
    
        let new_video = NewVideos {
            author: au,
            channel: ch,
            title: ti,
            link: lk,
            published: pu,
            userid: u,
        };
    
        diesel::insert_into(videos)
            .values(&new_video)
            .execute(&mut self.conn)
            .map_err(|err| {
                eprintln!("Error adding video: {}", err);
                err
            })
            .map(|_| ())
    }

    pub fn find_user_private_key(&mut self, ch: &str) -> Option<String> {
        use crate::schema::youtube_users::dsl::*;
        let results = youtube_users.filter(channel.eq(ch)).load::<YoutubeUser>(&mut self.conn).expect("Error loading users");
        results.first().map(|user| user.privatekey.to_string())
    }

    pub fn find_user_public_key(&mut self, ch: &str) -> Option<String> {
        use crate::schema::youtube_users::dsl::*;
        let results = youtube_users.filter(channel.eq(ch)).load::<YoutubeUser>(&mut self.conn).expect("Error loading users");
        results.first().map(|user| user.publickey.to_string())
    }

    
}