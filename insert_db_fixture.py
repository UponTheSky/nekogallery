import sqlite3

def _insert_db_fixture() -> None:
    with sqlite3.connect("./sqlite.db") as conn:
        conn.execute("""
            CREATE TABLE IF NOT EXISTS cat (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )
        """)
        conn.execute("INSERT INTO cat (id, name) VALUES (0, 'phil');")
        conn.execute("INSERT INTO cat (id, name) VALUES (1, 'stu');")
        conn.execute("INSERT INTO cat (id, name) VALUES (2, 'alan');")
        conn.execute("INSERT INTO cat (id, name) VALUES (3, 'doug');")

if __name__ == "__main__":
   _insert_db_fixture() 
   