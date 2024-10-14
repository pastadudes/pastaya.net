// server.js
const express = require('express');
const sqlite3 = require('sqlite3').verbose();
const cors = require('cors');

const app = express();
app.use(cors());
app.use(express.json());

// Connect to SQLite database at the specified path
const db = new sqlite3.Database('../counter.db', (err) => {
    if (err) {
        console.error('Error opening database ' + err.message);
    } else {
        console.log('Connected to the SQLite database.');
    }
});

// Create table if it doesn't exist
db.serialize(() => {
    db.run(`CREATE TABLE IF NOT EXISTS counter (
        id INTEGER PRIMARY KEY,
        count INTEGER DEFAULT 0
    )`);
    db.run(`INSERT INTO counter (id, count) VALUES (1, 0) ON CONFLICT(id) DO NOTHING`); // Ensure there's a row to update
});

// Endpoint to get the current count
app.get('/count', (req, res) => {
    db.get('SELECT count FROM counter WHERE id = 1', (err, row) => {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        res.json({ count: row.count });
    });
});

// Endpoint to increment the counter
app.post('/increment', (req, res) => {
    db.run('UPDATE counter SET count = count + 1 WHERE id = 1', function (err) {
        if (err) {
            return res.status(500).json({ error: err.message });
        }
        db.get('SELECT count FROM counter WHERE id = 1', (err, row) => {
            if (err) {
                return res.status(500).json({ error: err.message });
            }
            res.json({ count: row.count });
        });
    });
});

// Start the server
const PORT = 3000;
app.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
    console.log('GLOBAL COUNTER RAHHH!!!!!')
});
