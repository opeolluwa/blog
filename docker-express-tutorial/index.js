const express = require("express");
const app = express();
const port = 3000;
const hey = require("./hey");
app.get("/", (req, res) => {
  res.send("Hello World!");
});

app.listen(port, () => {
hey()
  console.log(`Example app listening on port ${port}`);
});
