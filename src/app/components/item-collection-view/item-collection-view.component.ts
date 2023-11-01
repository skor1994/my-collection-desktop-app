import { Component, Input, OnInit } from '@angular/core';
import { take } from 'rxjs';
import { ItemCollection } from 'src/app/models/item-collection';
import { DatabaseService } from 'src/app/services/database.service';

@Component({
  selector: 'app-item-collection-view',
  templateUrl: './item-collection-view.component.html',
  styleUrls: ['./item-collection-view.component.css']
})
export class ItemCollectionViewComponent implements OnInit {
  @Input() selectedCollection!: ItemCollection;

  constructor(private databaseService: DatabaseService) { }

  ngOnInit() {
  }

  public create() {
    this.databaseService.createItem(this.selectedCollection.id)
      .pipe(take(1))
      .subscribe(res => this.selectedCollection?.items.push(res));
  }

}
