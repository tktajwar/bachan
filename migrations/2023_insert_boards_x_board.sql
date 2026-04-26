INSERT INTO board (url, label, category)
VALUES
    ('x', 'অলৌকিক', 'Interests')
ON CONFLICT (url)
DO NOTHING;
