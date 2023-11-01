import { Item } from "./item";

export interface ItemCollection {
    id: Number;
    created_at: String;
    updated_on: String;
    items: Item[];
}
