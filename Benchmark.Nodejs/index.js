import express from 'express';
import { v4 as uuidv4 } from 'uuid';
import User from './models/user.js';

const app = express();
const port = 5200;

app.get('/api/user', (req, res) => {
    let user = new User(uuidv4(), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com");
    res.json(user);
});

app.listen(port, () => {
  console.log(`app listening on port ${port}`)
});
