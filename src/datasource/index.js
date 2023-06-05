import Database from "tauri-plugin-sql-api";

const db = await Database.load("sqlite:data.db");

async function isTable() {
  const result = await db.select("SELECT name FROM sqlite_master WHERE type='table' AND name='table_name'");
  console.log(result);
}
