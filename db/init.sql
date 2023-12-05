CREATE TABLE IF NOT EXISTS articles (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255),
    content TEXT,
    published_date VARCHAR(255)
);
