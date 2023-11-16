import {AgeType} from "../pkg";

export function isAgeType(value: any): value is AgeType {
    // return true or false
    return typeof value === 'number' || typeof value === 'string';
}

export function  isString(value:any): value is string {
    return typeof value  === 'string';
}