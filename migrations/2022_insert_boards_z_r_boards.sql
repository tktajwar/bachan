INSERT INTO board (url, label, category)
VALUES
    ('z', 'অন্যান্য সাইট', 'Misc'),
    ('r', 'অপসারিত', 'Misc')
ON CONFLICT (url)
DO NOTHING;
