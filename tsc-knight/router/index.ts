import express, {Request, Response, Router, Express}  from "express";
import {AddKnightDto, ApiResponse, KnightEntity, AgeType,  knightsDatabase, KnightInterface} from "../pkg";
import {isAgeType, isString} from "../validators";
import {AppDataSource} from "../datasource/config";
import {Knight} from "../datasource/models";

const KnightDatabase = AppDataSource.getRepository(Knight)

export let router : Router;
router = express.Router();



// get all knights
router.get('/', (req: Request, res: Response) => {
const allKnight
if(!knightsDatabase.length) {
    return res.json(new ApiResponse("no knights in Camelot at the moment, ", null));
}
return res.json(new ApiResponse("Knights of Camelot at your service", knightsDatabase));
})

// // add new knight
// router.post('/',  (req: Request, res: Response) => {
//     const payload = req.body as unknown as AddKnightDto;
//     const {name, age } = payload;
//
//     // request validation
//     if (!name && !age){
//      return   res.status(400).json(new ApiResponse("Knight name and  age is required", null));
//     }
//     // validate the request type
//     else if (!isAgeType(age)){
//         return   res.status(400).json(new ApiResponse("Knight age may only be number or string", null));
//     }
//
//     else if (!isString(name)){
//         return   res.status(400).json(new ApiResponse("Knight names may only be  string", null));
//     }
//
//     else {
//         let newKnight  = new Knight(name, age).save();
//         return res.json(new ApiResponse(`${newKnight.name } has been added to the company`, newKnight));
//     }
// })
//
//
// // update knight
// router.patch("/", (req: Request, res:Response )=>{
//     const knightId = req.query.id as unknown as  Required<string>;
//     const payload = req.body as unknown as Partial<KnightInterface>
//
//     // retrieve
//   const knight =  knightsDatabase.find((knight:KnightEntity)=>{knight.id == knightId.trim()})
//     if(!knight){
//         return res.status(404).send(new ApiResponse("No kight with the Provided Id", null))
//     }
//
//     // update th rescord
// })
