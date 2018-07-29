use store::app::AppState;
impl AppState {
    pub fn get_command(_command: String) {
        info!("Command Issued: {:?}", _command);
        info!("get command"); 
    }
}
