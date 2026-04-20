INSERT INTO board (url, label, category)
VALUES
    ('mu', 'মিউজিক', 'Hobbies'),
    ('sci', 'বিজ্ঞান ও গণিত', 'Interests'),
    ('his', 'মানবতা ও ইতিহাস', 'Interests'),
    ('lit', 'সাহিত্য', 'Interests')
ON CONFLICT (url)
DO NOTHING;

UPDATE thread SET board = 'mu' WHERE board = 's';
UPDATE thread SET board = 'sci' WHERE board = 'bg';
UPDATE thread SET board = 'his' WHERE board = 'me';
UPDATE thread SET board = 'lit' WHERE board = 'sh';
UPDATE PendingPost SET board = 'mu' WHERE board = 's';
UPDATE PendingPost SET board = 'sci' WHERE board = 'bg';
UPDATE PendingPost SET board = 'his' WHERE board = 'me';
UPDATE PendingPost SET board = 'lit' WHERE board = 'sh';

DELETE FROM board WHERE url IN ('s', 'bg', 'me', 'sh');
