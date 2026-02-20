INSERT INTO board (url, label, category)
VALUES 
    ('a', 'আনিমে এবং মাঙ্গা (Anime & Manga)', 'Hobbies'),
    ('kr', 'খাদ্য ও রান্না (Food & Cooking)', 'Hobbies'),
    ('kd', 'খেলাধুলা (Sports)', 'Hobbies'),
    ('sangeet', 'সঙ্গীত (Music)', 'Hobbies'),
    ('tv', 'ফিল্ম ও টেলিভিশন (Film & Television)', 'Hobbies'),
    ('vg', 'ভিডিও গেমস (Video Games)', 'Hobbies'),
    ('desh', 'বাংলাদেশ (Bangladesh)', 'Interests'),
    ('sahitto', 'সাহিত্য (Literature)', 'Interests'),
    ('me', 'মানবতা ও ইতিহাস (History & Humanity)', 'Interests'),
    ('bg', 'বিজ্ঞান ও গণিত (Science & Math)', 'Interests'),
    ('ghor', 'ঘরের সাজসজ্জা (Home Décor)', 'Lifestyle'),
    ('shajgoj', 'সাজগোজ (Fashion)', 'Lifestyle'),
    ('sharir', 'স্বাস্থ্য ও শরীরচর্চা (Health & Fitness)', 'Lifestyle'),
    ('manosik', 'মানসিক স্বাস্থ্য (Mental Health)', 'Lifestyle'),
    ('ghoshona', 'ঘোষণা (Announcements)', 'Misc'),
    ('site', 'সাইট আলোচনা (Site Talk)', 'Misc'),
    ('b', 'বিবিধ (Misc)', 'Misc')
ON CONFLICT (url)
DO NOTHING;
