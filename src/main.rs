use platform_dirs::UserDirs;
use std::path::PathBuf;

fn main() {
    let db = create_db_path();
    println!("{:?}", db);
}

fn create_db_path() -> PathBuf {
    let mut db = UserDirs::new().unwrap().document_dir;
    db.push("rug.sqlite");
    db
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
