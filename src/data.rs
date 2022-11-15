use crate::Res;
use chrono::{Datelike, NaiveDateTime};
use rusqlite;
use rusqlite::{types::ToSql, Connection};
use serde::Serialize;

pub fn connect() -> Res<Connection> {
    Connection::open("data.db").map_err(|e| e.into())
}

pub fn create() -> Res<()> {
    let conn = connect()?;
    let sql = include_str!("../sql_files/create.sql");

    conn.execute_batch(sql)?;
    println!("Tables created.");
    Ok(())
}

pub fn seed() -> Res<()> {
    let conn = connect()?;

    let drop_sql = include_str!("../sql_files/drop.sql");
    conn.execute_batch(drop_sql)?;

    create()?;

    let reg1 = Registration::new(chrono::Local::now().naive_local(), 1, 100);
    reg1.save(&conn)?;
    let reg2 = Registration::new(chrono::Local::now().naive_local(), 2, 440);
    reg2.save(&conn)?;

    Ok(())
}

#[allow(dead_code)]
pub fn drop() -> Res<()> {
    let conn = connect()?;
    let sql = include_str!("../sql_files/drop.sql");

    conn.execute_batch(sql)?;
    println!("Tables dropped.");
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct Registration {
    id: i32,
    date: String,
    dateint: i32,
    item_id: i32,
    quantity: u32,
}

impl Registration {
    pub fn new(date: NaiveDateTime, item_id: i32, quantity: u32) -> Self {
        let date_str = date.format("%Y-%m-%d %H:%M:%S.%3f").to_string();
        let dateint = date.year() as i32 * 10000 + date.month() as i32 * 100 + date.day() as i32;

        Registration {
            id: 0,
            date: date_str,
            dateint,
            item_id,
            quantity,
        }
    }

    pub fn date(&self) -> Res<NaiveDateTime> {
        NaiveDateTime::parse_from_str(self.date.as_str(), "%Y-%m-%d %H:%M:%S.%3f")
            .map_err(|e| e.into())
    }

    pub fn save(&self, conn: &Connection) -> Res<usize> {
        let sql =
            "INSERT INTO registrations (date, dateint, item_id, quantity) VALUES (?1, ?2, ?3, ?4)";

        let rows_affected = conn.execute(
            sql,
            &[
                &self.date,
                &self.dateint as &dyn ToSql,
                &self.item_id,
                &self.quantity,
            ],
        )?;

        println!("Registration saved. Rows affected: {}", rows_affected);

        Ok(rows_affected)
    }

    pub fn find_registrations(
        conn: &Connection,
        where_stmt: &str,
        params: &[&dyn ToSql],
    ) -> Res<Vec<Registration>> {
        let select_sql = "SELECT * FROM registrations WHERE ".to_string();
        let sql = select_sql + where_stmt;

        let mut stmt = conn.prepare(&sql)?;
        let res = stmt.query_map(params, |row: &rusqlite::Row| {
            let id: i32 = row.get("id").unwrap();
            let date = row.get("date").unwrap();
            let dateint: i32 = row.get("dateint").unwrap();
            let item_id: i32 = row.get("item_id").unwrap();
            let quantity: u32 = row.get("quantity").unwrap();

            Ok(Registration {
                id,
                date,
                dateint,
                item_id,
                quantity,
            })
        })?;

        Ok(res
            .enumerate()
            .filter_map(|(i, r)| match r {
                Ok(res) => Some(res),
                Err(e) => {
                    println!("Parse error on result #{}: {}", i, e);
                    None
                }
            })
            .collect())
    }
}
