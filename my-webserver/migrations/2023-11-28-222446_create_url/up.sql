-- Your SQL goes here
CREATE TABLE IF NOT EXISTS shortenedurl (
  id CHARACTER(36) NOT NULL PRIMARY KEY,
  url CHARACTER(200),
  shorturl CHARACTER(20),
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);
CREATE TRIGGER IF NOT EXISTS UpdateTimestamps AFTER UPDATE ON shortenedurl
  FOR EACH ROW WHEN NEW.updated_at <= OLD.updated_at 
BEGIN 
  update users set updated_at=CURRENT_TIMESTAMP where id=OLD.id;  
END;