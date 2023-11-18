import {Knight} from "./models"
import { DataSource } from "typeorm"
export const AppDataSource = new DataSource({
    type: "sqlite",
    database: "knights",
    synchronize: true,
    logging: true,
    entities: [Knight],
    subscribers: [],
    migrations: [],
})