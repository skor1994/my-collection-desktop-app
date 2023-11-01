import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { ItemCollectionMenuComponent } from "./components/item-collection-menu/item-collection-menu.component";
import { ItemCollectionViewComponent } from "./components/item-collection-view/item-collection-view.component";

@NgModule({
  declarations: [
    AppComponent,
    ItemCollectionMenuComponent,
    ItemCollectionViewComponent
  ],
  imports: [BrowserModule],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
