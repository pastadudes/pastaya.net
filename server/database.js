// database.js
const sqlite3 = require('sqlite3').verbose();
const db = new sqlite3.Database('./counter.db');

// Create table if it doesn't exist
db.serialize(() => {
    db.run(`CREATE TABLE IF NOT EXISTS counter (
        id INTEGER PRIMARY KEY,
        count INTEGER NOT NULL
    )`);

    // Insert initial value if the table is empty
    db.get('SELECT count FROM counter WHERE id = 1', (err, row) => {
        if (!row) {
            db.run('INSERT INTO counter (id, count) VALUES (?, ?)', [1, 0]);
        }
    });
});

module.exports = db;
