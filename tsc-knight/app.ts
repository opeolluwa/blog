import express, { Express,} from 'express';
import {router} from "./router";
import "reflect-metadata"
import {AppDataSource} from "./datasource/config"
let app : Express;
const PORT : number =  5007;
app = express();

// mount the routes
app.use(express.json())
app.use("/", router);

// start the server
app.listen(PORT, async ()=>{
    AppDataSource.initialize()
        .then(() => {
            console.log(`Knights marching to war on the dreaded on path http://0.0.0.0:${PORT}`)
        })
        .catch((error:Error)=>{
            console.log(error)
        })
})


