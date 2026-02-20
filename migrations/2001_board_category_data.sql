INSERT INTO BoardCategory (categoryname)
VALUES 
    ('Hobbies'),
    ('Interests'),
    ('Lifestyle'),
    ('Misc')
ON CONFLICT (categoryname)
DO NOTHING;
