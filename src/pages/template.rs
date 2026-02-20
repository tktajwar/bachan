use lazy_static::lazy_static;

lazy_static! {

    // Tera templating engine

    pub static ref TERA: tera::Tera = {
	tera::Tera::new("templates/**/*")
	    .expect("Failed to initiate Tera")
    };
}
