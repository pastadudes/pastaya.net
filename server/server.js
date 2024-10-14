// server.js
const express = require('express');
const cors = require('cors');
const db = require('./database'); // Import database module

const app = express();
app.use(cors());
app.use(express.json());

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

// Removed the reset endpoint

// Start the server
const PORT = 3000;
app.listen(PORT, () => {
    console.log(`Server running on http://localhost:${PORT}`);
    console.log('GLOBAL NYAA RAHHHHH!!!!')
});
