import {Entity, Column, PrimaryGeneratedColumn, BaseEntity, BeforeInsert} from "typeorm"
import {v4 as uuidv4} from  'uuid';
import {generateRandomNumber} from "../pkg";
@Entity("knight_information")
export class Knight extends BaseEntity{
    @PrimaryGeneratedColumn('uuid')
    id!: string

    @Column({type:'varchar'})
    name!: string

    @Column({type:'varchar'})
    age!: string | number

    @Column({type: 'int'})
    strength!: string | number

    @Column({type: 'int'})
    agility!: string | number

    @Column({type:'int'})
    speed!: string | number

    @BeforeInsert()
    seedEntity(){
        this.id = uuidv4(); // set the Id
        this.strength = generateRandomNumber();
        this.speed = generateRandomNumber();
        this.agility = generateRandomNumber();
    }
}