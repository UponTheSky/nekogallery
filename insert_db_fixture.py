import sqlite3
from uuid import uuid4

def _insert_db_fixture() -> None:
    with sqlite3.connect("./sqlite.db") as conn:
        conn.execute("""
            CREATE TABLE IF NOT EXISTS cat (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL
            )
        """)
        conn.execute(f"INSERT INTO cat (id, name) VALUES ('{uuid4()}', 'phil');")
        conn.execute(f"INSERT INTO cat (id, name) VALUES ('{uuid4()}', 'stu');")
        conn.execute(f"INSERT INTO cat (id, name) VALUES ('{uuid4()}', 'alan');")
        conn.execute(f"INSERT INTO cat (id, name) VALUES ('{uuid4()}', 'doug');")

if __name__ == "__main__":
   _insert_db_fixture() 
   