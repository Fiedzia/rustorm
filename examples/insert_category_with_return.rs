extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;

use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

use rustorm::query::Query;
use rustorm::dao::{Dao,IsDao};
use rustorm::pool::ManagedPool;
use rustorm::table::{IsTable,Table};

#[derive(Debug, Clone)]
struct Category {
    pub category_id:Uuid,
    pub name:Option<String>,
    pub active:bool,
    pub client_id:Option<Uuid>,
    pub created:DateTime<UTC>,
    pub created_by:Option<Uuid>,
    pub description:Option<String>,
    pub help:Option<String>,
    pub organization_id:Option<Uuid>,
    pub priority:Option<f64>,
    pub updated:DateTime<UTC>,
    pub updated_by:Option<Uuid>,
}

impl IsDao for Category{
    fn from_dao(dao:&Dao)->Self{
        Category{
            organization_id: dao.get_opt("organization_id"),
            client_id: dao.get_opt("client_id"),
            created: dao.get("created"),
            created_by: dao.get_opt("created_by"),
            updated: dao.get("updated"),
            updated_by: dao.get_opt("updated_by"),
            priority: dao.get_opt("priority"),
            name: dao.get_opt("name"),
            description: dao.get_opt("description"),
            help: dao.get_opt("help"),
            active: dao.get("active"),
            category_id: dao.get("category_id"),
        }
    }
}


impl IsTable for Category{

    fn table()->Table{
    
        Table{
            schema:"bazaar".to_string(),
            name:"category".to_string(),
            parent_table:Some("record".to_string()),
            sub_table:vec![],
            comment:None,
            columns:vec![],
        }
    }
}

fn main(){
    let url = "postgres://postgres:p0stgr3s@localhost/bazaar_v6";
    let mut pool = ManagedPool::init(&url, 1);
    let db = pool.connect().unwrap();
        
    let category: Category = Query::insert()
            .set("name", &"Test Category12121")
        .into_table(&"bazaar.category")
            .return_all()
            .collect_one(db.as_ref()).unwrap();
    println!("category: {}", category.name.unwrap());
}
