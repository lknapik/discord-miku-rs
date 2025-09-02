use core::str;

use poise::{
    serenity_prelude::{Colour, CreateEmbed},
    CreateReply,
};
use rusqlite::{Connection, Result};

use crate::{Context, Error};

/*
Need to set to full path (ie no ~) 
*/
const DB_FILE_NAME: &'static str = "silksong.db";

#[derive(Debug)]
struct Table {
    id: i32,
    username: String,
    secret_type: String,
    secret_name: String,
}

#[derive(Debug, poise::ChoiceParameter)]
enum SecretType {
    Location,
    Boss,
    Item,
}

#[poise::command(prefix_command, slash_command)]
pub async fn temp(_ctx: Context<'_>) -> Result<(), Error> {
    let sql = Connection::open(DB_FILE_NAME).unwrap();
    println!("temp called");

    sql.execute("CREATE TABLE silksong (id INTEGER PRIMARY KEY, username TEXT, secret_type TEXT, secret_name TEXT);", ()).unwrap();
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Secret Type"] secret_type: SecretType,
    #[description = "Secret Name"] name: String,
) -> Result<(), Error> {
    let sql = Connection::open(DB_FILE_NAME).unwrap();

    sql.execute(
        "INSERT INTO silksong (username, secret_type, secret_name) VALUES (?1, ?2, ?3);",
        (
            ctx.author().name.clone(),
            format!("{:?}", secret_type),
            name.clone().to_ascii_lowercase(),
        ),
    )
    .unwrap();

    let _ = ctx
        .reply(format!("Added new {:?}: \"{}\"", secret_type, name))
        .await;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let mut locations_vec = Vec::new();
    let mut boss_vec = Vec::new();
    let mut item_vec = Vec::new();

    {
        let sql = Connection::open(DB_FILE_NAME).unwrap();

        let mut stmt = sql
            .prepare(
                "
            SELECT id, username, secret_type, secret_name FROM silksong WHERE username LIKE (?1)",
            )
            .unwrap();

        let list_iter = stmt
            .query_map([&format!("{}", ctx.author().name.clone())], |row| {
                Ok(Table {
                    id: row.get(0).unwrap(),
                    username: row.get(1).unwrap(),
                    secret_type: row.get(2).unwrap(),
                    secret_name: row.get(3).unwrap(),
                })
            })
            .unwrap();

        for row in list_iter {
            let row = row.unwrap();
            match row.secret_type.as_str() {
                "Location" => locations_vec.push(row.secret_name),
                "Boss" => boss_vec.push(row.secret_name),
                "Item" => item_vec.push(row.secret_name),
                _ => (),
            }
        }
    }

    let embed = CreateEmbed::default()
        .title(format!("{}'s Discoveries", ctx.author().name.clone()))
        .color(Colour::new(0xe92337))
        .field("Locations", locations_vec.join("\n"), true)
        .field("Bosses", boss_vec.join("\n"), true)
        .field("Items", item_vec.join("\n"), true);

    let mut msg = CreateReply::default();

    msg = msg.embed(embed);

    let _ = ctx.send(msg).await;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn delete(
    ctx: Context<'_>,
    #[description = "Secret Name"] name: String,
) -> Result<(), Error> {
    let sql = Connection::open(DB_FILE_NAME).unwrap();

    sql.execute(
        "DELETE FROM silksong WHERE username = ?1 AND secret_name = ?2;",
        (ctx.author().name.clone(), name.clone().to_ascii_lowercase()),
    )
    .unwrap();

    let _ = ctx.reply(format!("Removed \"{}\"", name)).await;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn compare(
    ctx: Context<'_>,
    #[description = "Other username"] other_username: String,
) -> Result<(), Error> {
    let mut locations_vec = Vec::new();
    let mut boss_vec = Vec::new();
    let mut item_vec = Vec::new();

    {
        let sql = Connection::open(DB_FILE_NAME).unwrap();

        let mut stmt = sql.prepare("SELECT person1.secret_type, person1.secret_name FROM silksong AS person1 JOIN silksong AS person2 ON person1.secret_name = person2.secret_name WHERE person1.username LIKE (?1) AND person2.username LIKE (?2);").unwrap();

        let list_iter = stmt
            .query_map(
                [
                    &format!("{}", ctx.author().name.clone()),
                    &format!("{}", other_username.clone()),
                ],
                |row| {
                    Ok(Table {
                        id: 0,
                        username: "".to_string(),
                        secret_type: row.get(0).unwrap(),
                        secret_name: row.get(1).unwrap(),
                    })
                },
            )
            .unwrap();

        for row in list_iter {
            let row = row.unwrap();
            match row.secret_type.as_str() {
                "Location" => locations_vec.push(row.secret_name),
                "Boss" => boss_vec.push(row.secret_name),
                "Item" => item_vec.push(row.secret_name),
                _ => (),
            }
        }
    }

    let embed = CreateEmbed::default()
        .title(format!("{}'s Shared Discoveries", other_username.clone()))
        .color(Colour::new(0xe92337))
        .field("Locations", locations_vec.join("\n"), true)
        .field("Bosses", boss_vec.join("\n"), true)
        .field("Items", item_vec.join("\n"), true);

    let mut msg = CreateReply::default();

    msg = msg.embed(embed);

    let _ = ctx.send(msg).await;
    Ok(())
}
