INSERT INTO board (url, label, category)
VALUES
    ('pr', 'প্রযুক্তি (technology)', 'Interests')
ON CONFLICT (url)
DO NOTHING;
