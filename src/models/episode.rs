use actix_web::web;
use serde_json::Value;
use sqlx::{sqlite::{SqlitePool, SqliteRow}, Error, query, Row};
use chrono::{DateTime, NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    id: i64,
    channel_id: i64,
    title: String,
    description: String,
    yt_id: String,
    link: String,
    published_at: NaiveDateTime,
    image: String,
    listen: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEpisode {
    channel_id: i64,
    title: String,
    description: String,
    yt_id: String,
    link: String,
    published_at: NaiveDateTime,
    image: String,
    listen: bool,
}

impl Episode{
    pub async fn create(pool: &web::Data<SqlitePool>, new: &NewEpisode) -> Result<Episode, Error>{
        let sql = "INSERT INTO episodes (channel_id, title, description, yt_id, link, published_at, image, listen) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);";
        let id = query(sql)
            .bind(&new.channel_id)
            .bind(&new.title)
            .bind(&new.description)
            .bind(&new.yt_id)
            .bind(&new.link)
            .bind(&new.published_at)
            .bind(&new.image)
            .bind(&new.listen)
            .execute(pool.get_ref())
            .await?
        .last_insert_rowid();
        Self::read(pool, id).await
    }

    pub async fn read(pool: &web::Data<SqlitePool>, id: i64) -> Result<Episode, Error>{
        let sql = "SELECT id, channel_id, title, description, yt_id, link,
            published_at, image, listen FROM episodes WHERE id = $1";
        query(sql)
            .bind(id)
            .map(|row: SqliteRow| Episode{
                id: row.get("id"),
                channel_id: row.get("channel_id"),
                title: row.get("title"),
                description: row.get("description"),
                yt_id: row.get("yt_id"),
                link: row.get("link"),
                published_at: row.get("published_at"),
                image: row.get("image"),
                listen: row.get("listen"),

            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn read_all(pool: &web::Data<SqlitePool>) -> Result<Vec<Episode>, Error>{
        let sql = "SELECT id, channel_id, title, description, yt_id, link,
            published_at, image, listen FROM episodes";
        
        query(sql)
            .map(|row: SqliteRow| Episode{
                id: row.get("id"),
                channel_id: row.get("channel_id"),
                title: row.get("title"),
                description: row.get("description"),
                yt_id: row.get("yt_id"),
                link: row.get("link"),
                published_at: row.get("published_at"),
                image: row.get("image"),
                listen: row.get("listen"),

            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn update(pool: &web::Data<SqlitePool>, episode: Episode) -> Result<Episode, Error>{
        let sql = "UPDATE episodes SET channel_id = $2, title = $3,
            description = $4, yt_id = $5, link = $6,
            published_at = $7, image = $8, listen = $9 FROM episodes WHERE
            id = $1";
        query(sql)
            .bind(episode.id)
            .bind(episode.channel_id)
            .bind(episode.title)
            .bind(episode.description)
            .bind(episode.yt_id)
            .bind(episode.link)
            .bind(episode.published_at)
            .bind(episode.image)
            .bind(episode.listen)
            .execute(pool.get_ref())
            .await?;
        Self::read(pool, episode.id).await
    }

    pub async fn delete(pool: web::Data<SqlitePool>, id: i64) -> Result<Episode, Error>{
        let episode = Self::read(&pool, id).await?;
        let sql = "DELETE from episodes WHERE id = $1";
        query(sql)
            .bind(id)
            .execute(pool.get_ref())
            .await?;
        Ok(episode)
    }
}
