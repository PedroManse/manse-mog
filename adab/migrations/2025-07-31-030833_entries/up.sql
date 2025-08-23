-- Your SQL goes here
CREATE TABLE entries (
	source_id INTEGER NOT NULL,
  song TEXT NOT NULL,
  band TEXT NOT NULL,
  date DATE NOT NULL,
	link TEXT,
	album_250px TEXT,
	album_56px TEXT,
	FOREIGN KEY(source_id) REFERENCES sources(source_id)
);

CREATE TABLE sources (
	source_id INTEGER PRIMARY KEY,
	site TEXT NOT NULL
);

