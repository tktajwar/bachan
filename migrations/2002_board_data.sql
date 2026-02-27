INSERT INTO board (url, label, category)
VALUES 
    ('a', 'আনিমে এবং মাঙ্গা (Anime & Manga)', 'Hobbies'),
    ('kr', 'খাদ্য ও রান্না (Food & Cooking)', 'Hobbies'),
    ('kd', 'খেলাধুলা (Sports)', 'Hobbies'),
    ('s', 'সঙ্গীত (Music)', 'Hobbies'),
    ('tv', 'ফিল্ম ও টেলিভিশন (Film & Television)', 'Hobbies'),
    ('vg', 'ভিডিও গেমস (Video Games)', 'Hobbies'),
    ('desh', 'বাংলাদেশ (Bangladesh)', 'Interests'),
    ('sh', 'সাহিত্য (Literature)', 'Interests'),
    ('me', 'মানবতা ও ইতিহাস (History & Humanity)', 'Interests'),
    ('bg', 'বিজ্ঞান ও গণিত (Science & Math)', 'Interests'),
    ('gh', 'ঘরের সাজসজ্জা (Home Décor)', 'Lifestyle'),
    ('shaj', 'সাজগোজ (Fashion)', 'Lifestyle'),
    ('ms', 'স্বাস্থ্য ও শরীরচর্চা (Health & Fitness)', 'Lifestyle'),
    ('mh', 'মানসিক স্বাস্থ্য (Mental Health)', 'Lifestyle'),
    ('g', 'ঘোষণা (Announcements)', 'Misc'),
    ('site', 'সাইট আলোচনা (Site Talk)', 'Misc'),
    ('b', 'বিবিধ (Misc)', 'Misc')
ON CONFLICT (url)
DO NOTHING;
