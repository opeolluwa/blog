// create a union type to hold knight age type
// the age maybe accepted as string or number then stored as number
export type AgeType = number | string;
import {v4 as uuidv4} from 'uuid';

export interface KnightInterface {
    name: string;
    age: AgeType;
    strength: number;
    agility: number;
    speed: number;
}

export interface KnightEntity extends KnightInterface {
    id:string;
}

// a pseudo database initially set to empty
export const knightsDatabase : Array<KnightEntity> =[]



// helper types
export interface  AddKnightDto {
    name: string
    age: AgeType
}


// The API response builder
export class ApiResponse<T>{
    message: string
    data: T;
    constructor(message:string, data:T){
        this.message = message;
        this. data = data;

        return {
            data, message
        }
    }
}

export function  generateRandomNumber(){
    return  Math.floor(Math.random() * 101)
}