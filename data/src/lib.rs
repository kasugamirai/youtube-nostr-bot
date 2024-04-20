mod models;
mod schema;

use crate::models::{NewVideos, NewYoutubeUser, Videos, YoutubeUser};

use diesel::RunQueryDsl;
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../data/migrations");

#[derive(Debug)]
pub enum Error {
    Diesel(diesel::result::Error),
    Connection(diesel::ConnectionError),
    IoError(std::io::Error),
    SerdeError(serde_yaml::Error),
}

impl From<diesel::ConnectionError> for Error {
    fn from(err: diesel::ConnectionError) -> Self {
        Error::Connection(err)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::Diesel(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::SerdeError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Diesel(err) => write!(f, "Diesel error: {}", err),
            Error::Connection(err) => write!(f, "Connection error: {}", err),
            Error::IoError(err) => write!(f, "IO error: {}", err),
            Error::SerdeError(err) => write!(f, "Serde error: {}", err),
        }
    }
}

pub struct DbConnection {
    conn: PgConnection,
}

impl DbConnection {
    pub fn new(dsn: &str) -> Result<DbConnection, Error> {
        let conn = PgConnection::establish(dsn)?;
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
        Ok(youtube_users
            .filter(channel.eq(ch))
            .load::<YoutubeUser>(&mut self.conn)?)
    }

    pub fn add_avatar(&mut self, ch: &str, av: &str) -> Result<(), Error> {
        use crate::schema::youtube_users::dsl::*;

        Ok(diesel::update(youtube_users.filter(channel.eq(ch)))
            .set(avatar.eq(av))
            .execute(&mut self.conn)
            .map_err(|err| {
                log::error!("Error adding avatar: {}", err);
                err
            })
            .map(|_| ())?)
    }

    pub async fn query_channel_id(&mut self, name: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(name)?;
        Ok(results.first().map(|user| user.channel_id.to_string()))
    }

    pub async fn avatar_exists(&mut self, name: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(name)?;
        Ok(results.first().and_then(|user| user.avatar.clone()))
    }

    pub async fn video_exists(&mut self, lk: &str) -> Result<bool, Error> {
        use crate::schema::videos::dsl::*;
        let results = videos.filter(link.eq(lk)).load::<Videos>(&mut self.conn)?;
        Ok(results.len() > 0)
    }

    pub async fn channel_exists(&mut self, ch: &str) -> Result<bool, Error> {
        let results = self.load_users(ch)?;
        Ok(!results.is_empty())
    }

    pub async fn user_exists(&mut self, name: &str) -> Result<bool, Error> {
        let results = self.load_users(name)?;
        Ok(!results.is_empty())
    }

    pub async fn query_avatar(&mut self, name: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(name)?;
        Ok(results.first().and_then(|user| user.avatar.clone()))
    }

    pub async fn query_user_name(&mut self, ch: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(ch)?;
        Ok(results.first().map(|user| user.username.to_string()))
    }

    pub async fn add_user(
        &mut self,
        un: &str,
        av: &str,
        pk: &str,
        prk: &str,
        ch: &str,
        chid: &str,
    ) -> Result<(), Error> {
        use crate::schema::youtube_users::dsl::*;

        let new_user = NewYoutubeUser {
            username: un.to_string(),
            avatar: Some(av.to_string()),
            publickey: pk.to_string(),
            privatekey: prk.to_string(),
            channel: ch.to_string(),
            channel_id: chid.to_string(),
        };

        Ok(diesel::insert_into(youtube_users)
            .values(&new_user)
            .execute(&mut self.conn)
            .map_err(|err| {
                log::error!("Error adding user: {}", err);
                err
            })
            .map(|_| ())?)
    }

    pub async fn query_user_id(&mut self, ch: &str) -> Result<Option<i32>, Error> {
        let results = self.load_users(ch)?;
        Ok(results.first().map(|user| user.id))
    }

    pub async fn add_video(
        &mut self,
        au: &str,
        ch: &str,
        ti: &str,
        lk: &str,
        pu: bool,
    ) -> Result<(), Error> {
        use crate::schema::videos::dsl::*;

        let u_id = self.query_user_id(ch).await?.unwrap();

        let new_video = NewVideos {
            author: au.to_string(),
            channel: ch.to_string(),
            title: ti.to_string(),
            link: lk.to_string(),
            published: pu,
            userid: u_id,
        };

        Ok(diesel::insert_into(videos)
            .values(&new_video)
            .execute(&mut self.conn)
            .map_err(|err| {
                log::error!("Error adding video: {}", err);
                err
            })
            .map(|_| ())?)
    }

    pub async fn find_user_private_key(&mut self, ch: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(ch)?;
        Ok(results.first().map(|user| user.privatekey.to_string()))
    }

    pub async fn find_user_public_key(&mut self, ch: &str) -> Result<Option<String>, Error> {
        let results = self.load_users(ch)?;
        Ok(results.first().map(|user| user.publickey.to_string()))
    }
}
