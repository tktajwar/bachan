INSERT INTO board (url, label, category)
VALUES
    ('pol', 'রাজনীতি (Politics)', 'Interests')
ON CONFLICT (url)
DO NOTHING;
