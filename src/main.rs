use rusqlite::{Connection, Result, params};

// connect to db
fn connect_db() -> Result<Connection> {
    let conn = Connection::open("college.database")?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    println!("connected to detabase");
    Ok(conn)
}

fn main() -> Result<()> {
    let conn = connect_db()?;
    //_create_table(&conn)?;

    let colleges = [
        "Massachusetts Institute of Technology",
        "Georgia Institute of Technology",
        "Stanford University",
        "Purdue University",
        "University of Michigan",
        "California Institute of Technology",
        "University of Illinois Urbana-Champaign",
        "University of Texas at Austin",
        "Texas A&M University",
        "University of Colorado Boulder",
        "University of Maryland",
        "Virginia Tech",
        "Princeton University",
        "Cornell University",
        "Ohio State University",
        "Pennsylvania State University",
        "University of Florida",
        "University of California, Los Angeles",
        "University of California, San Diego",
        "University of California, Irvine",
        "University of California, Davis",
        "University of California, Berkeley",
        "University of Southern California",
        "University of Washington",
        "Embry-Riddle Aeronautical University",
        "Arizona State University",
        "University of Arizona",
        "Auburn University",
        "University of Alabama",
        "University of Alabama in Huntsville",
        "Mississippi State University",
        "Iowa State University",
        "Oklahoma State University",
        "University of Oklahoma",
        "University of Kansas",
        "Wichita State University",
        "North Carolina State University",
        "University of Central Florida",
        "Florida Institute of Technology",
        "New Mexico State University",
        "West Virginia University",
        "University of Notre Dame",
        "University of Hartford",
        "San Jose State University",
        "San Diego State University",
        "California Polytechnic State University",
        "California State Polytechnic University, Pomona",
        "California State University, Long Beach",
        "Missouri University of Science and Technology",
        "Rensselaer Polytechnic Institute",
        "Clarkson University",
        "Syracuse University",
        "University at Buffalo",
        "University of Cincinnati",
        "Rutgers University",
        "Saint Louis University",
        "Worcester Polytechnic Institute",
        "Case Western Reserve University",
        "Western Michigan University",
        "University of Minnesota",
        "University of Texas at Arlington",
        "University of South Carolina",
        "University of Tennessee, Knoxville",
        "University of Virginia",
        "University of Miami",
        "Illinois Institute of Technology",
        "Capitol Technology University",
        "Tuskegee University",
        "Southern New Hampshire University",
        "University of Dayton",
        "Washington University in St. Louis",
        "University of Houston",
        "University of Nevada, Las Vegas",
        "Air Force Institute of Technology",
        "United States Air Force Academy",
        "University of Akron",
        "Morehead State University",
        "University of Alaska Fairbanks",
        "University of North Dakota",
        "Wright State University",
        "Inter American University of Puerto Rico",
        "West Virginia University Institute of Technology",
        "George Washington University",
        "Naval Postgraduate School",
        "Johns Hopkins University",
        "University of Pennsylvania",
        "Duke University",
        "University of Utah",
        "Utah State University",
        "Oregon State University",
        "University of Nevada, Reno",
        "University of New Orleans",
        "University of Massachusetts Lowell",
        "University of Delaware",
        "University of New Hampshire",
        "Rice University",
        "University of Connecticut",
        "University of Colorado Denver",
    ];
    for college in colleges {
        insert_colleges(&conn,college)?;
    }

    print_colleges(&conn)?;
Ok(())
}


fn _create_table(conn: &Connection) -> Result<()>{
    conn.execute(
        "CREATE TABLE IF NOT EXISTS colleges (
            id INTEGER PRIMARY KEY,
            college_name TEXT NOT NULL UNIQUE
        );",
        [],
    )?;

Ok(())
}

fn insert_colleges(conn: &Connection, college_name: &str) -> Result<i64> {
    conn.execute(
        "INSERT OR IGNORE INTO colleges (college_name) VALUES (?1)",
        params![college_name],
    )?;
    let college_id = conn.query_row(
        "SELECT id FROM colleges WHERE college_name = ?1",
        params![college_name],
        |row| row.get(0),
    )?;
    Ok(college_id)
}


fn print_colleges(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT * FROM colleges"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
        ))
    })?;

    for row in rows {
        let (id, name) = row?;
        println!("ID: {} | College: {}", id, name);
    }

    Ok(())
}
