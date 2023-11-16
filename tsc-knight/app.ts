import express, { Express, Request, Response } from 'express';
import {router} from "./router";

let app : Express;
const PORT : number =  5007;
app = express();

// mount the routes
app.use(express.json());
app.use("/", router);

// start the server
app.listen(PORT, ()=>{
    console.log(`Knight marching on path http://0.0.0.0:${PORT}`)
})

