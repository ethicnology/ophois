# osmtograph

![](https://github.com/ethicnology/osmtograph/blob/main/assets/osmtograph-polylines.png?raw=true)

## prerequisites

- [Osmium Tool](https://osmcode.org/osmium-tool/) or a filtered by highways
  OpenStreetMap file
- Bash Command Line (cat)

## installation

### download executable

Download the
[**lastest release**](https://github.com/ethicnology/osmtograph/releases) for
your system (**Linux, MacOS or Windows**)

### build from sources

clone this repository and then

```sh
cargo build --release
```

## procedure

#### export a map from [OpenStreetMap](https://www.openstreetmap.org)

![](https://github.com/ethicnology/osmtograph/blob/main/assets/osmtograph-input.png?raw=true)

#### extract ways of type [highway](https://wiki.openstreetmap.org/wiki/Key:highway) from your map

```sh
osmium tags-filter map.osm w/highway -o highways-ways.osm
```

#### extract nodes

```sh
cat highways-ways.osm | ./osmtograph --format | ./osmtograph --nodes > nodes
```

#### extract links

```sh
cat highways-ways.osm | ./osmtograph --format | ./osmtograph --links > links
```

#### extract ways

```sh
cat highways-ways.osm | ./osmtograph --format | ./osmtograph --ways > ways
```

#### combine nodes and links into a graph

```sh
cat nodes.txt links > graph
```

## output

### graph file

```sh
node_id␟lat␟48.8279975␟lon␟2.3518307␟key␟value… #represents a node and his data
node_id␟lat␟48.8279975␟lon␟2.3518307
node_id␟lat␟48.8279975␟lon␟2.3518307␟key␟value␟key␟value…
node_id␟node_id␟way_id #represents a link/edge
node_id␟node_id␟way_id
```

### ways file

```sh
way_id␟key␟value␟key␟value…
way_id
way_id␟key␟value
way_id␟key␟value␟key␟value…
way_id
```

> **_NOTE:_** "␟" ASCII 31 (0x1F) Unit Separator - Used to indicate separation
> between units within a record. I choose this one because OSM data contains
> various special characters.

### example with real world data

#### graph

```sh
21658501␟lat␟48.8279975␟lon␟2.3518307
21658502␟lat␟48.8279276␟lon␟2.3513732
92192237␟lat␟48.8275872␟lon␟2.3490245
1829061602␟lat␟48.8275089␟lon␟2.3484223
1829061607␟lat␟48.8278868␟lon␟2.347252
1829061610␟lat␟48.8260051␟lon␟2.3474783
1829061614␟lat␟48.8273732␟lon␟2.3487375
1829061640␟lat␟48.827773␟lon␟2.3503086
1829061642␟lat␟48.8278201␟lon␟2.3506517
1829061648␟lat␟48.8277624␟lon␟2.3502336
1829061667␟lat␟48.8265177␟lon␟2.3501273
1829061676␟lat␟48.8269249␟lon␟2.348167
1852590201␟lat␟48.8276523␟lon␟2.3494784
2268836829␟lat␟48.8276001␟lon␟2.3486802␟addr:housenumber␟38a␟source␟cadastre-dgi-fr source : Direction Générale des Impôts - Cadastre. Mise à jour : 2013
2286779145␟lat␟48.8260569␟lon␟2.3475149␟crossing␟zebra␟highway␟crossing␟kerb␟lowered␟tactile_paving␟yes
2286779154␟lat␟48.8276739␟lon␟2.3496385␟crossing␟zebra␟crossing_ref␟zebra␟highway␟crossing␟kerb␟lowered␟tactile_paving␟yes
2576426847␟lat␟48.8273391␟lon␟2.3487858␟crossing␟zebra␟crossing_ref␟zebra␟highway␟crossing␟kerb␟lowered␟tactile_paving␟yes
2576426850␟lat␟48.8274242␟lon␟2.3486471
2576426851␟lat␟48.8274323␟lon␟2.3487423
2576426852␟lat␟48.8274347␟lon␟2.3487671
2576426853␟lat␟48.8274352␟lon␟2.348721
2576426854␟lat␟48.8274412␟lon␟2.3487844
2576426855␟lat␟48.827493␟lon␟2.3485442
2576426856␟lat␟48.8275026␟lon␟2.3485468
2576426858␟lat␟48.8275464␟lon␟2.3489207
2576426859␟lat␟48.8275541␟lon␟2.3489099
2597215157␟lat␟48.8265578␟lon␟2.3500902␟crossing␟uncontrolled␟highway␟crossing␟tactile_paving␟yes
2598270008␟lat␟48.8276879␟lon␟2.349736
3758221284␟lat␟48.8273411␟lon␟2.3486982␟crossing␟zebra␟crossing_ref␟zebra␟highway␟crossing␟kerb␟lowered␟tactile_paving␟yes
3758221292␟lat␟48.8274025␟lon␟2.3486929␟crossing␟zebra␟crossing_ref␟zebra␟highway␟crossing␟kerb␟lowered␟tactile_paving␟yes
3758221295␟lat␟48.8275185␟lon␟2.3484976␟barrier␟chain␟emergency␟yes␟motor_vehicle␟no
3758221301␟lat␟48.8275751␟lon␟2.3489308␟barrier␟chain␟emergency␟yes␟motor_vehicle␟no
3761637482␟lat␟48.8274512␟lon␟2.3486719
3761637486␟lat␟48.8275249␟lon␟2.348704
3761637488␟lat␟48.8275416␟lon␟2.3486683␟barrier␟bollard␟bicycle␟yes␟foot␟yes␟motor_vehicle␟no
3761637489␟lat␟48.8275453␟lon␟2.348698
3761637490␟lat␟48.8275499␟lon␟2.348735␟barrier␟bollard␟bicycle␟yes␟foot␟yes␟motor_vehicle␟no
3761637496␟lat␟48.8278544␟lon␟2.3473522␟crossing␟uncontrolled␟crossing_ref␟zebra␟highway␟crossing␟tactile_paving␟yes
6400885441␟lat␟48.8274338␟lon␟2.3488187␟crossing␟zebra␟highway␟crossing␟kerb␟lowered␟tactile_paving␟yes
6400933176␟lat␟48.8268914␟lon␟2.3481419␟crossing␟zebra␟highway␟crossing␟kerb␟lowered␟tactile_paving␟yes
1829061610␟2286779145␟10588201
2286779145␟6400933176␟10588201
6400933176␟1829061676␟10588201
1829061676␟3758221284␟10588201
3758221284␟1829061614␟10588201
1829061614␟6400885441␟10588201
6400885441␟92192237␟10588201
92192237␟3758221301␟171948831
3758221301␟3761637490␟171948831
92192237␟1852590201␟171948832
1852590201␟2286779154␟171948832
2286779154␟2598270008␟171948832
2598270008␟1829061648␟171948832
1829061648␟1829061640␟171948832
1829061640␟1829061642␟171948832
1829061642␟21658502␟171948832
21658502␟21658501␟171948832
1829061614␟3758221292␟171948835
3758221292␟2576426850␟171948835
2576426850␟1829061602␟171948835
1829061602␟3761637496␟171948835
3761637496␟1829061607␟171948835
1829061667␟2597215157␟171948849
2597215157␟2576426847␟171948849
2576426847␟1829061614␟171948849
2576426854␟2576426852␟251421704
2576426852␟2576426851␟251421704
2576426851␟2576426853␟251421704
2576426853␟3761637482␟251421704
3761637482␟2576426855␟251421704
2576426855␟2576426856␟251421704
2576426856␟3761637486␟251421704
3761637486␟2576426859␟251421704
2576426859␟2576426858␟251421704
2576426858␟2576426854␟251421704
3761637490␟3761637489␟372609520
3761637489␟3761637488␟372609520
3761637488␟3758221295␟372609521
3758221295␟1829061602␟372609521
2268836829␟3761637489␟372609522
3761637489␟3761637486␟372609522
3761637486␟3761637482␟372609522
3761637482␟2576426850␟372609522
3758221292␟2576426853␟965882503
```

#### ways data

```sh
10588201␟bicycle␟designated␟cycleway:left␟opposite␟foot␟designated␟highway␟living_street␟lit␟yes␟maxspeed␟20␟name␟Rue de l'Espérance␟oneway␟yes␟oneway:bicycle␟no␟source:maxspeed␟BMO arrêté n°2013P0819␟surface␟sett␟zone:maxspeed␟FR:20
171948831␟emergency␟yes␟highway␟pedestrian␟lit␟yes␟motor_vehicle␟no␟name␟Rue de la Butte aux Cailles␟source:maxspeed␟BMO arrêté n°2013P0819␟surface␟sett
171948832␟bicycle␟designated␟cycleway:left␟opposite_lane␟foot␟designated␟highway␟living_street␟lit␟yes␟maxspeed␟20␟name␟Rue de la Butte aux Cailles␟oneway␟yes␟oneway:bicycle␟no␟source:maxspeed␟BMO arrêté n°2013P0819␟surface␟sett␟zone:maxspeed␟FR:20
171948835␟bicycle␟designated␟cycleway␟opposite␟foot␟designated␟highway␟living_street␟lit␟yes␟maxspeed␟20␟name␟Rue de la Butte aux Cailles␟oneway␟yes␟oneway:bicycle␟no␟parking:condition:both␟ticket␟parking:lane:both␟parallel␟source:maxspeed␟BMO arrêté n°2013P0819␟surface␟sett␟zone:maxspeed␟FR:20
171948849␟bicycle␟designated␟cycleway␟opposite␟foot␟designated␟highway␟living_street␟lit␟yes␟maxspeed␟20␟name␟Rue Buot␟oneway␟yes␟oneway:bicycle␟no␟source:maxspeed␟BMO arrêté n°2013P0819␟surface␟sett␟zone:maxspeed␟FR:20
251421704␟area␟yes␟highway␟pedestrian␟lit␟yes␟name␟Place de la Commune de Paris␟ref:FR:FANTOIR␟751132251D␟source␟cadastre-dgi-fr source : Direction Générale des Impôts - Cadastre. Mise à jour : 2013␟surface␟paving_stones␟wikidata␟Q3390354
372609520␟highway␟pedestrian␟lit␟yes␟motor_vehicle␟no␟name␟Rue de la Butte aux Cailles␟surface␟sett
372609521␟emergency␟yes␟highway␟pedestrian␟lit␟yes␟motor_vehicle␟no␟name␟Rue de la Butte aux Cailles␟source:maxspeed␟BMO arrêté n°2013P0819␟surface␟sett
372609522␟highway␟service␟motor_vehicle␟destination␟oneway␟no␟service␟emergency_access␟surface␟asphalt
965882503␟crossing␟marked␟footway␟crossing␟highway␟footway␟surface␟asphalt
```

![](https://github.com/ethicnology/osmtograph/blob/main/assets/osmtograph-output.png?raw=true)

> **_NOTE:_** As you can see they are many nodes with a
> [degree](https://en.wikipedia.org/wiki/Degree_(graph_theory)) equal to two.
> **If you don't care about GPS coordinates** you could remove degree two nodes.
