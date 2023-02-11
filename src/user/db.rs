use mysql::prelude::Queryable;
use super::user::User;
pub struct Db{
    pool: mysql::Pool,
    conn: mysql::PooledConn
}

impl Db{
    pub fn new() -> Self{
        let id = "root";
        let pw = "xx1121!!!!";
        let name = "rust";

        let url = format!("mysql://{id}:{pw}@localhost:3306/{name}");
        let pool = mysql::Pool::new(url.trim()).unwrap();
        let conn = pool.get_conn().unwrap();
        Db{
            pool,
            conn
        }
    }

    pub fn get_users(&mut self) -> Vec<User>{
        self.conn.query_map("select * from users", |(id, pw)|User{id, pw}).unwrap()
    }

    pub fn get_user(&mut self, id: &str) -> Vec<User>{
        self.conn.query_map(format!("select * from users where id={id}").trim(), |(id, pw)|User{id, pw}).unwrap()
    }

    pub fn new_user(&mut self, user: User) -> bool{
        let q = format!("insert into users value {user:?}");
        match self.conn.exec_drop(q.trim(), ()){
            Ok(_) => {},
            Err(_) => {return false}
        };
        true
    }
    
}

