use rusqlite::{Connection, named_params};
use serde::{Deserialize, Serialize};
use chrono::Utc;

const CURRENT_DB_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: i16,
    name: String
}

#[derive(Serialize, Deserialize)]
pub struct ItemCollection {
    id: i16,
    created_at: String,
    updated_on: String,
    items: Option<Vec<Item>>
}

pub fn initialize_database() -> Result<Connection, rusqlite::Error> {
    let mut db = Connection::open("dev.sqlite")?;

    let mut user_pragma = db.prepare("PRAGMA user_version")?;
    let existing_user_version: u32 = user_pragma.query_row([], |row| { Ok(row.get(0)?) })?;
    drop(user_pragma);

    create_database(&mut db, existing_user_version)?;

    Ok(db)
}

/// Upgrades the database to the current version.
pub fn create_database(db: &mut Connection, existing_version: u32) -> Result<(), rusqlite::Error> {
  if existing_version < CURRENT_DB_VERSION {
    db.pragma_update(None, "journal_mode", "WAL")?;

    let tx = db.transaction()?;

    tx.pragma_update(None, "user_version", CURRENT_DB_VERSION)?;

    tx.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS item_collections (id INTEGER PRIMARY KEY, created_at TEXT NOT NULL, updated_on TEXT NOT NULL);
        CREATE TABLE IF NOT EXISTS item (id INTEGER PRIMARY KEY, name TEXT NOT NULL, item_collection_id INTEGER, FOREIGN KEY(item_collection_id) REFERENCES item_collections (id));
        "
    )?;

    tx.commit()?;
  }

  Ok(())
}

pub fn add_new_item_to_col(db: &Connection, id: &i16) -> Result<(), rusqlite:: Error> {
    let mut statement = db.prepare("INSERT INTO item (name, item_collection_id) VALUES (@name, @item_collection_id)")?;
    statement.execute(named_params! { "@name": "New item", "@item_collection_id": id })?;

    Ok(())
}

pub fn add_new_item_col(db: &Connection) -> Result<(), rusqlite::Error> {
    let timestamp = Utc::now().to_string();
    let mut statement = db.prepare("INSERT INTO item_collections (created_at, updated_on) VALUES (@created_at, @updated_on)")?;
    statement.execute(named_params! { "@created_at": timestamp, "@updated_on": timestamp })?;

    Ok(())
}

pub fn get_last_collection(db: &Connection) -> Result<ItemCollection, rusqlite::Error> {
    let id = db.last_insert_rowid().to_string();
    let mut statement = db.prepare("SELECT id, created_at, updated_on FROM item_collections WHERE id == ?1")?;
    statement.query_row([&id], |row| {
        Ok(ItemCollection {
            id: row.get(0)?,
            created_at: row.get(1)?,
            updated_on: row.get(2)?,
            items: Some(vec![])
        })
    })
}

pub fn get_last_item(db: &Connection) -> Result<Item, rusqlite::Error> {
    let id = db.last_insert_rowid().to_string();
    let mut statement = db.prepare("SELECT id, name FROM item WHERE id == ?1")?;
    statement.query_row([&id], |row| {
        Ok(Item {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })
}

pub fn get_all(db: &Connection) -> Result<Vec<ItemCollection>, rusqlite::Error> {
    let mut statement = db.prepare("SELECT id, created_at, updated_on FROM item_collections")?;
    let mut rows = statement.query([])?;
    let mut items = Vec::new();
    while let Some(row) = rows.next()? {
        items.push(ItemCollection {
        id: row.get(0)?,
        created_at: row.get(1)?,
        updated_on: row.get(2)?,
        items: Some(vec![])
        });
    }
  
    Ok(items)
}