use actix_web::web::ServiceConfig;
use to_do::get_time_factory;

mod to_do;

pub fn views_factory(app: &mut ServiceConfig) {
    // println!("views_factory()-call");
    get_time_factory(app);
}
