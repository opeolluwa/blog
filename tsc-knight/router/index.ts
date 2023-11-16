import express, {Request, Response, Router, Express}  from "express";
import {AddKnightDto, ApiResponse, KnightEntity, AgeType, Knight, knightsDatabase} from "../pkg";
import {isAgeType, isString} from "../validators";
import {type} from "node:os";


export let router : Router;
router = express.Router();



// get all knights
router.get('/', (req: Request, res: Response) => {
if(!knightsDatabase.length) {
    return res.json(new ApiResponse("no knights in Camelot at the moment, ", null));
}
return res.json(new ApiResponse("Knights of Camelot at your service", knightsDatabase));
})

// add new knight
router.post('/',  (req: Request, res: Response) => {
    console.log(req.body)
    return
    const payload = req.body as unknown as AddKnightDto;
    const {name, age } = payload;

    // request validation
    if (!name && !age){
     return   res.status(400).json(new ApiResponse("Knight name and  age is required", null));
    }
    // validate the request type
    else if (!isAgeType(age)){
        return   res.status(400).json(new ApiResponse("Knight age may only be number or string", null));
    }

    else if (!isString(name)){
        return   res.status(400).json(new ApiResponse("Knight names may only be  string", null));
    }

    else {
        let newKnight  = new Knight(name, age);
    }
})
