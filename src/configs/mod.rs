use std::env;

pub struct CONFIGS {
    pub secret_key: String,
}

pub fn configs() -> CONFIGS {
    CONFIGS {
        secret_key: env::var("SECRET_KEY").expect("Missing SECRET_KEY enviroment variable"),
    }
}
