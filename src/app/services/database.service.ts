import { Injectable } from '@angular/core';
import { Observable, defer } from 'rxjs';
import { ItemCollection } from '../models/item-collection';
import { invoke } from "@tauri-apps/api/tauri";
import { Item } from '../models/item';

@Injectable({
  providedIn: 'root'
})
export class DatabaseService {

  constructor() { }

  public createItemCollection() : Observable<ItemCollection>{
    return defer(() => {
      return invoke<ItemCollection>("create_item_collection").then((collection) => {
        return collection;
      });
    })
  }

  public createItem(id: Number) : Observable<Item>{
    return defer(() => {
      return invoke<Item>("create_item", { id: id }).then((item) => {
        return item;
      });
    })
  }

  public getItemCollections() : Observable<ItemCollection[]>{
    return defer(() => {
      return invoke<ItemCollection[]>("get_item_collections").then((collections) => {
        return collections;
      });
    })
  }
}
