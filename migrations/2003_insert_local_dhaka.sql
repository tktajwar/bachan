INSERT INTO BoardCategory (categoryname)
VALUES
    ('Local')
ON CONFLICT (categoryname)
DO NOTHING;

INSERT INTO board (url, label, category)
VALUES
    ('dhk', 'ঢাকা (Dhaka)', 'Local')
ON CONFLICT (url)
DO NOTHING;
