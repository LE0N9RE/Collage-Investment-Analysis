use rusqlite::{Connection, Result, params};

fn main() -> Result<()> {
    let conn = connect_db()?;

    create_table(&conn)?;

    let mit = insert_colleges(&conn, "MIT")?;
    let stanford = insert_colleges(&conn, "Stanford")?;
    let georgia_tech =insert_colleges(&conn, "Georgia Tech")?;
    let purdue = insert_colleges(&conn, "Purdue University")?;

    insert_ranking(&conn, "QS", mit, 1, 2026)?;

    insert_academic_offerings(
        &conn,
        mit,
        "Aerospace Engineering",
        Some("Bachelor of Science"),
    )?;
    insert_academic_offerings(
        &conn,
        stanford,
        "Aerospace Engineering",
        Some("Bachelor of Science"),
    )?;
    insert_academic_offerings(
        &conn,
        georgia_tech,
        "Aerospace Engineering",
        Some("Bachelor of Science"),
    )?;

    insert_academic_offerings(
        &conn,
        purdue,
        "Aeronautical and Astronautical Engineering",
        Some("Bachelor of Science"),
    )?;
    print_colleges(&conn)?;
    Ok(())
}
// connect to db
fn connect_db() -> Result<Connection> {
    let conn = Connection::open("college.database")?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    println!("nconnected to detabase");
    Ok(conn)
}

fn create_table(conn: &Connection) -> Result<()>{
    conn.execute(
        "CREATE TABLE IF NOT EXISTS colleges (
            id INTEGER PRIMARY KEY,
            college_name TEXT NOT NULL UNIQUE
        );",
        [],
    )?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS ranking (
        id INTEGER PRIMARY KEY,
        college_id INTEGER NOT NULL,
        source TEXT NOT NULL,
        rank INTEGER,
        year INTEGER,
        FOREIGN KEY(college_id) REFERENCES colleges(id)
        );",[],
        )?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS academic_offerings (
            id INTEGER PRIMARY KEY,
            college_id INTEGER NOT NULL,
            field TEXT NOT NULL,
            degree_name TEXT,
            FOREIGN KEY (college_id)REFERENCES colleges(id)
        );",[],
        )?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS college_features(
        id INTEGER PRIMARY KEY,
        college_id INTEGER NOT NULL,
        category TEXT NOT NULL,
        feature TEXT NOT NULL,
        available BOOLEAN NOT NULL,
        FOREIGN KEY (college_id) REFERENCES colleges(id),
        UNIQUE (college_id, category, feature)
        );",[],
        )?;

    Ok(())
}

fn insert_colleges(conn: &Connection, college_name: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO colleges (college_name) VALUES (?1)",
        params![college_name],
    )?;

    Ok(conn.last_insert_rowid())
}

fn insert_ranking(conn: &Connection,
    source: &str,
    college_id: i64,
    rank: u8,
    year: u16) -> Result<()> {

    conn.execute(
        "INSERT INTO ranking (source, college_id, rank, year)
         VALUES (?1, ?2, ?3, ?4)",
        params![source, college_id, rank, year],
    )?;

    Ok(())
}

fn insert_academic_offerings(conn: &Connection,
    college_id: i64,
    field: &str,
    degree_name: Option<&str>) -> Result<()> {

    conn.execute(
        "INSERT INTO academic_offerings
        (college_id, field, degree_name)
        VALUES (?1, ?2, ?3)",
        params![college_id, field, degree_name],
    )?;

    Ok(())
}

fn insert_college_features(
    conn: &Connection,
    college_id: i64,
    category: &str,
    feature: &str,
    available: bool) -> Result<()> {

    conn.execute(
        "INSERT INTO college_features
        (college_id, category, feature, available)
        VALUES (?1, ?2, ?3, ?4)",
        params![college_id, category, feature, available],
    )?;

    Ok(())


}

fn print_colleges(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, college_name FROM colleges"
    )?;

    let colleges = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
        ))
    })?;

    for college in colleges {
        let (id, name) = college?;
        println!("ID: {} | College: {}", id, name);
    }

    Ok(())
}
