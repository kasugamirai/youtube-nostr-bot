use crate::models::{Config, NewVideos, NewYoutubeUser, Videos, YoutubeUser};
use diesel::prelude::*;
use diesel::result::Error;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::fs::File;
use std::io::BufReader;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../data/migrations");

pub struct DbConnection {
    conn: PgConnection,
}

impl DbConnection {
    pub fn new(config_path: &str) -> Result<DbConnection, Box<dyn std::error::Error>> {
        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_yaml::from_reader(reader)?;
        let conn = PgConnection::establish(&config.dsn)?;

        Ok(DbConnection { conn })
    }
    pub fn run_migrations(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.conn.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }

    fn load_users(&mut self, ch: &str) -> Result<Vec<YoutubeUser>, Error> {
        use crate::schema::youtube_users::dsl::*;
        youtube_users
            .filter(channel.eq(ch))
            .load::<YoutubeUser>(&mut self.conn)
    }

    pub fn add_avatar(&mut self, ch: &str, av: &str) -> Result<(), Error> {
        use crate::schema::youtube_users::dsl::*;

        diesel::update(youtube_users.filter(channel.eq(ch)))
            .set(avatar.eq(av))
            .execute(&mut self.conn)
            .map_err(|err| {
                log::error!("Error adding avatar: {}", err);
                err
            })
            .map(|_| ())
    }

    pub fn query_channel_id(&mut self, ch: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(ch)?;
        Ok(results.first().map(|user| user.channel_id.to_string()))
    }

    pub fn avatar_exists(&mut self, ch: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(ch)?;
        Ok(results.first().and_then(|user| user.avatar.clone()))
    }

    pub fn video_exists(&mut self, lk: &str) -> Result<bool, Error> {
        use crate::schema::videos::dsl::*;
        let results = videos.filter(link.eq(lk)).load::<Videos>(&mut self.conn)?;
        Ok(results.len() > 0)
    }

    pub fn channel_exists(&mut self, ch: &str) -> Result<bool, Error> {
        let results = self.load_users(ch)?;
        Ok(!results.is_empty())
    }

    pub fn add_user(
        &mut self,
        un: String,
        av: String,
        pk: String,
        prk: String,
        ch: String,
        chid: String,
    ) -> Result<(), Error> {
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
                log::error!("Error adding user: {}", err);
                err
            })
            .map(|_| ())
    }

    pub fn query_user_id(&mut self, ch: &str) -> Result<Option<i32>, Error> {
        let results = self.load_users(ch)?;
        Ok(results.first().map(|user| user.id))
    }

    pub fn add_video(
        &mut self,
        au: String,
        ch: String,
        ti: String,
        lk: String,
        pu: bool,
    ) -> Result<(), Error> {
        use crate::schema::videos::dsl::*;

        let u = self
            .query_user_id(&ch)?
            .expect("User should exist at this point");

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
                log::error!("Error adding video: {}", err);
                err
            })
            .map(|_| ())
    }

    pub fn find_user_private_key(&mut self, ch: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(ch)?;
        Ok(results.first().map(|user| user.privatekey.to_string()))
    }

    pub fn find_user_public_key(&mut self, ch: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(ch)?;
        Ok(results.first().map(|user| user.publickey.to_string()))
    }
}
