use rusqlite::Connection;
use tauri::{AppHandle, State, Manager};

pub struct DbState {
    pub conn: std::sync::Mutex<Option<Connection>>,
}

pub trait DatabaseAccess {
    fn conn<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult;

    fn conn_mut<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult;
}

impl DatabaseAccess for AppHandle {
    fn conn<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&Connection) -> TResult {
        let app_state: State<DbState> = self.state();
        let db_connection_guard = app_state.conn.lock().unwrap();
        let db = db_connection_guard.as_ref().unwrap();
      
        operation(db)
      }
    
      fn conn_mut<F, TResult>(&self, operation: F) -> TResult where F: FnOnce(&mut Connection) -> TResult {
        let app_state: State<DbState> = self.state();
        let mut db_connection_guard = app_state.conn.lock().unwrap();
        let conn = db_connection_guard.as_mut().unwrap();
      
        operation(conn)
      }
}