CREATE TABLE articles (
  id serial,
  title VARCHAR NOT NULL,
  content text NOT NULL,
  created_by int NOT NULL,
  created_on timestamptz,
  PRIMARY KEY (id),
  FOREIGN KEY (created_by) REFERENCES users(id)
);