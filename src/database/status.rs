use rusqlite::{Connection, Result};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct StatusRow {
    #[tabled(rename = "Status")]
    status: String,
}

pub fn insert_status(conn: &Connection) {
    conn.execute("insert into statuss(statuss) values(?1)", &[&"inactive"])
        .expect("Err: failed to add the status");
}

pub fn view_status(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("select statuss from statuss")?;

    let rows = stmt.query_map([], |row| {
        Ok(StatusRow {
            status: row.get(0)?,
        })
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    let table = Table::new(results);
    println!("{}", table);

    Ok(())
}

pub fn update_status(conn: &Connection, status: &str) -> Result<()> {
    conn.execute("update statuss set statuss=?", &[&status])
        .expect("Err: failed to update the status");
    Ok(())
}
