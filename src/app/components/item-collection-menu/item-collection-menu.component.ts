import { Component, OnInit } from '@angular/core';
import { take } from 'rxjs';
import { ItemCollection } from 'src/app/models/item-collection';
import { DatabaseService } from 'src/app/services/database.service';

@Component({
  selector: 'app-item-collection-menu',
  templateUrl: './item-collection-menu.component.html',
  styleUrls: ['./item-collection-menu.component.css']
})
export class ItemCollectionMenuComponent implements OnInit {
  public collections?: ItemCollection[];
  public selectedCollection?: ItemCollection;

  constructor(private databaseService: DatabaseService) { }

  ngOnInit() {
    this.databaseService.getItemCollections()
      .pipe(take(1))
      .subscribe(res => this.collections = res);
  }

  public select(itemcol: ItemCollection) {
    this.selectedCollection = itemcol;
  }

  public create() {
    this.databaseService.createItemCollection()
      .pipe(take(1))
      .subscribe(res => this.collections?.push(res));
  }
}
