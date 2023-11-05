import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";
import { MatSidenavModule } from '@angular/material/sidenav';
import { AppComponent } from "./app.component";
import { ItemCollectionMenuComponent } from "./components/item-collection-menu/item-collection-menu.component";
import { ItemCollectionViewComponent } from "./components/item-collection-view/item-collection-view.component";
import { NoopAnimationsModule } from '@angular/platform-browser/animations';

@NgModule({
  declarations: [
    AppComponent,
    ItemCollectionMenuComponent,
    ItemCollectionViewComponent
  ],
  imports: [
    BrowserModule,
    MatSidenavModule,
    NoopAnimationsModule
  ],
  providers: [],
  bootstrap: [AppComponent],
})
export class AppModule {}
