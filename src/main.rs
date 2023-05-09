use eyre::Result;
use std::{cell::RefCell, rc::Rc};
use tiger_spacetraders::{start_ui, App};

fn main() -> Result<()> {
    // let mut config = Configuration {
    //     user_agent: Some("TigerSpacetraders/1.0.0/rust".to_owned()),
    //     ..Default::default()
    // };

    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;
    Ok(())
}
