use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Serialize)]
pub struct Board {
    pub url: &'static str,
    pub label: &'static str,
}

#[derive(Serialize)]
pub struct Boards {
    hobbies: Vec<&'static Board>,
    interests: Vec<&'static Board>,
    lifestyle: Vec<&'static Board>,
    misc: Vec<&'static Board>,
}

#[derive(Serialize)]
pub struct DashboardContext {
    pub boards: Boards,
}

#[derive(Serialize)]
pub struct BoardContext {
    pub board: Board,
}

lazy_static! {

    // Tera templating engine

    pub static ref TERA: tera::Tera = {
	tera::Tera::new("templates/**/*")
	    .expect("Failed to initiate Tera")
    };

    // Dashboard context

    pub static ref Dashboard_CTX: DashboardContext = DashboardContext {
	boards: Boards {
	    hobbies: vec![
		&Board_a_CTX.board,
		&Board_kr_CTX.board,
		&Board_kd_CTX.board,
		&Board_sangeet_CTX.board,
		&Board_tv_CTX.board,
		&Board_vg_CTX.board,
	    ],
	    interests: vec![
		&Board_desh_CTX.board,
		&Board_sahitto_CTX.board,
		&Board_me_CTX.board,
		&Board_bg_CTX.board,
	    ],
	    lifestyle: vec![
		&Board_ghor_CTX.board,
		&Board_shajgoj_CTX.board,
		&Board_sharir_CTX.board,
		&Board_manoshik_CTX.board,
	    ],
	    misc: vec![
		&Board_ghoshona_CTX.board,
		&Board_site_CTX.board,
		&Board_b_CTX.board,
	    ],
	}
    };

    // Hobbies boards contexts

    pub static ref Board_a_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/a/",
	    label: "আনিমে এবং মাঙ্গা (Anime & Manga)",
	}
    };

    pub static ref Board_kr_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/kr/",
	    label: "খাদ্য ও রান্না (Food & Cooking)",
	}
    };

    pub static ref Board_kd_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/kd/",
	    label: "খেলাধুলা (Sports)",
	}
    };

    pub static ref Board_sangeet_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/sangeet/",
	    label: "সঙ্গীত (Music)",
	}
    };

    pub static ref Board_tv_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/tv/",
	    label: "ফিল্ম ও টেলিভিশন (Film & Television)",
	}
    };

    pub static ref Board_vg_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/vg/",
	    label: "ভিডিও গেমস (Video Games)",
	}
    };

    // Interests boards contexts

    pub static ref Board_desh_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/desh/",
	    label: "বাংলাদেশ (Bangladesh)",
	}
    };

    pub static ref Board_sahitto_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/sahitto/",
	    label: "সাহিত্য (Literature)",
	}
    };

    pub static ref Board_me_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/me/",
	    label: "মানবতা ও ইতিহাস (History & Humanity)",
	}
    };

    pub static ref Board_bg_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/bg/",
	    label: "বিজ্ঞান ও গণিত (Science & Math)",
	}
    };

    // Lifestyle boards contexts

    pub static ref Board_ghor_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/ghor/",
	    label: "ঘরের সাজসজ্জা (Home Décor)",
	}
    };

    pub static ref Board_shajgoj_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/shajgoj/",
	    label: "সাজগোজ (Fashion)",
	}
    };

    pub static ref Board_sharir_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/sharir/",
	    label: "স্বাস্থ্য ও শরীরচর্চা (Health & Fitness)",
	}
    };

    pub static ref Board_manoshik_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/manosik/",
	    label: "মানসিক স্বাস্থ্য (Mental Health)",
	}
    };

    // Misc boards contexts

    pub static ref Board_ghoshona_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/ghoshona/",
	    label: "ঘোষণা (Announcements)",
	}
    };

    pub static ref Board_site_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/site/",
	    label: "সাইট আলোচনা (Site Talk)",
	}
    };

    pub static ref Board_b_CTX: BoardContext = BoardContext {
	board: Board {
	    url: "/b/",
	    label: "বিষয়বহির্ভূত (Random)",
	}
    };
}
