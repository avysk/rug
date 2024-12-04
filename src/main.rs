use futures::executor::block_on;
use platform_dirs::UserDirs;
use sea_orm::{Database, DbErr};
use std::path::PathBuf;

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("Error connecting to database: {}", err);
    }
}

fn create_db_path() -> PathBuf {
    let mut db = UserDirs::new().unwrap().document_dir;
    db.push("rug.sqlite");
    db
}

fn create_db_url() -> String {
    let mut uri = PathBuf::from("sqlite:");
    let db = create_db_path();
    uri.push(db);
    uri.to_str().unwrap().to_string()
}

async fn run() -> Result<(), DbErr> {
    let db_url = create_db_url();
    let conn = Database::connect(db_url);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_path_ends_with_database_name() {
        let db = create_db_path();
        assert!(db.ends_with("rug.sqlite"));
    }
}
