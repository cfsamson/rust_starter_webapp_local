use crate::Res;
use chrono::{Datelike, NaiveDateTime};
use rusqlite;
use rusqlite::{types::ToSql, Connection};
use serde::Serialize;
use repository::Repo;
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

    let repo = repository::Repository::new()?;
    let reg_repo = repo.registrations();
    reg_repo.add(repository::Registration::new(chrono::Local::now().naive_local(), 1, 100));
    reg_repo.add(repository::Registration::new(chrono::Local::now().naive_local(), 2, 440));
    repo.commit()?;

    Ok(())
}

pub fn drop() -> Res<()> {
    let conn = connect()?;
    let sql = include_str!("../sql_files/drop.sql");

    conn.execute_batch(sql)?;
    println!("Tables dropped.");
    Ok(())
}

pub mod repository {
    use crate::Res;
    use chrono::{Datelike, NaiveDateTime};
    use rusqlite;
    use rusqlite::{types::ToSql, Connection};
    use serde::Serialize;

    pub trait Repo {
        type Item;
        fn commit(&mut self) -> Res<usize>;
        fn delete(&mut self, id: i32);
        fn add(&mut self, item: Self::Item);
        fn find(&self, id: i32) -> Res<Option<Self::Item>>;
        fn get_all(&self) -> Res<Vec<Self::Item>>;
    }
    enum Ops<T> {
        Add(T),
        Delete(i32),
    }

    pub struct Repository<'a> {
        registrations: Registrations<'a>,
        // ADD NEW REPOSITORIES HERE
        conn: Connection,
    }

    impl<'a> Repository<'a> {
        pub fn new() -> Res<Self> {
            let conn = Connection::open("data.db")?;
            Ok(Repository {
                registrations: Registrations::new(&conn),
                conn: conn,
            })
        }

        
        pub fn commit(&mut self) -> Res<usize> {
            let mut affected_rows = 0;
            affected_rows += self.registrations.commit()?;
            // ADD NEW REPOSITORIES HERE
            Ok(affected_rows)
        }

        pub fn registrations(&mut self) -> &'a mut Registrations {
            &mut self.registrations
        }
    }


    pub struct Registrations<'a> {
        actions: Vec<Ops<Registration>>,
        conn: &'a Connection,
    }

    impl<'a> Registrations<'a> {
        fn new(conn: &'a Connection) -> Self {
            Registrations {
                actions: vec![],
                conn,
            }
        }
    }

    impl<'a> Repo for Registrations<'a> {
        type Item = Registration;
        fn commit(&mut self) -> Res<usize> {
            let mut rows_affected = 0;
            while let Some(action) = self.actions.pop() {
                match action {
                    Ops::Add(reg) => {
                        let sql =
            "INSERT INTO registrations (date, dateint, item_id, quantity) VALUES (?1, ?2, ?3, ?4)";

                        let affected = self.conn.execute(
                            sql,
                            &[
                                &reg.date,
                                &reg.dateint as &ToSql,
                                &reg.item_id,
                                &reg.quantity,
                            ],
                        )?;

                        rows_affected += affected;
                    }
                    Ops::Delete(id) => {
                        let sql = "DELETE FROM registrations WHERE id = ?1";
                        let affected = self.conn.execute(sql, &[&id])?;

                        rows_affected += affected;
                    }
                    _ => (),
                }
            }

            Ok(rows_affected)
        }

        fn delete(&mut self, id: i32) {
            self.actions.push(Ops::Delete(id));
        }

        fn add(&mut self, item: Self::Item) {
            self.actions.push(Ops::Add(item))
        }

        fn find(&self, id: i32) -> Res<Option<Registration>> {
            // go get from db here
            let sql = "SELECT * FROM registrations WHERE id = ?1";

            let mut stmt = self.conn.prepare(&sql)?;
            let res = stmt.query_map(&[&id], |row: &rusqlite::Row| {
                let id: i32 = row.get("id");
                let date = row.get("date");
                let dateint: i32 = row.get("dateint");
                let item_id: i32 = row.get("item_id");
                let quantity: u32 = row.get("quantity");

                Registration {
                    id,
                    date,
                    dateint,
                    item_id,
                    quantity,
                }
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
                .collect::<Vec<Registration>>()
                .pop())
        }

        fn get_all(&self) -> Res<Vec<Registration>> {
            let sql = "SELECT * FROM registrations";

            let mut stmt = self.conn.prepare(&sql)?;
            let res = stmt.query_map(rusqlite::NO_PARAMS, |row: &rusqlite::Row| {
                let id: i32 = row.get("id");
                let date = row.get("date");
                let dateint: i32 = row.get("dateint");
                let item_id: i32 = row.get("item_id");
                let quantity: u32 = row.get("quantity");

                Registration {
                    id,
                    date,
                    dateint,
                    item_id,
                    quantity,
                }
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
            let dateint =
                date.year() as i32 * 10000 + date.month() as i32 * 100 + date.day() as i32;

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
    }

}
