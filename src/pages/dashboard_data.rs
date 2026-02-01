use serde::Serialize;
use lazy_static::lazy_static;

#[derive(Serialize)]
struct Board {
    url: &'static str,
    label: &'static str,
}

#[derive(Serialize)]
pub struct Boards {
    hobbies: Vec<Board>,
    interests: Vec<Board>,
    lifestyle: Vec<Board>,
    misc: Vec<Board>,
}

#[derive(Serialize)]
pub struct Context {
    pub boards: Boards,
}

lazy_static! {
    pub static ref TERA: tera::Tera = {
	tera::Tera::new("templates/**/*")
	    .expect("Failed to initiate Tera")
    };

    pub static ref CTX: Context = Context {
	boards: Boards {
	    hobbies: vec![
		Board { url: "/a", label: "আনিমে এবং মাঙ্গা (Anime & Manga)" },
		Board { url: "/khaddo", label: "খাদ্য ও রান্না (Food & Cooking)" },
		Board { url: "/kheladhula", label: "খেলাধুলা (Sports)" },
		Board { url: "/sangeet", label: "সঙ্গীত (Music)" },
		Board { url: "/tv", label: "ফিল্ম ও টেলিভিশন (Film & Television)" },
		Board { url: "/vg", label: "ভিডিও গেমস (Video Games)" },
	    ],
	    interests: vec![
		Board { url: "/desh", label: "বাংলাদেশ (Bangladesh)" },
		Board { url: "/sahitto", label: "সাহিত্য (Literature)" },
		Board { url: "/manobota", label: "মানবতা ও ইতিহাস (History & Humanity)" },
		Board { url: "/biggan", label: "বিজ্ঞান ও গণিত (Science & Math)" },
	    ],
	    lifestyle: vec![
		Board { url: "/ghor", label: "ঘরের সাজসজ্জা (Home Décor)" },
		Board { url: "/shajgoj", label: "সাজগোজ (Fashion)" },
		Board { url: "/sastho", label: "স্বাস্থ্য ও শরীরচর্চা (Health & Fitness)" },
		Board { url: "/manosik", label: "মানসিক স্বাস্থ্য (Mental Health)" },
	    ],
	    misc: vec![
		Board { url: "/ghoshona", label: "ঘোষণা (Announcements)" },
		Board { url: "/site", label: "সাইট আলোচনা (Site Talk)" },
		Board { url: "/b", label: "বিষয়বহির্ভূত (Random)" },
	    ],
	}
    };
}
