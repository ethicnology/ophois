# Ophoïs, the opener of the ways, creates streets graph from OpenStreetMap

## installation

### pre-built

Download the
[**lastest linux release**](https://github.com/ethicnology/ophois/releases)

### build from sources

```sh
cargo build --release # >= Rust 1.58
```

## procedure

### Download a map

```sh
ophois download --city Pantin
```

### extract

```sh
CITY=Pantin; cat $CITY.osm | ophois format | ophois extract > $CITY-raw.graph
```

**same command with space separator**

```sh
CITY=Pantin; cat $CITY.osm | ./ophois format | ./ophois extract --separator ' ' > $CITY-raw.graph
```

### Simplify

#### keep the largest component, remove degree two nodes, replace nodes with under delta links by links and replace links (and nodes) which distance is under delta by a midpoint node connected to neighbours

```sh
CITY=Pantin; cat $CITY-raw.graph | ./ophois simplify --delta 10.0 > $CITY-simplified.graph
```

### Discretize

#### graph in equal parts which are between delta and delta*2

```sh
CITY=Pantin; cat $CITY-simplified.graph | ./ophois discretize --delta 5.0 > $CITY-discretized.graph
```

### One line simplify and discretize

```sh
CITY=Pantin; ./ophois download --city $CITY; cat $CITY.osm | ./ophois format | ./ophois extract | ./ophois simplify --delta 10 | ./ophois discretize --delta 5 > $CITY.graph
```

**same command with space separator**

```sh
CITY=Pantin; ./ophois download --city $CITY; cat $CITY.osm | ./ophois format | ./ophois extract -s ' ' | ./ophois simplify -s ' ' -d 10 | ./ophois discretize -s ' ' -d 5 > $CITY.graph
```

## output

### graph file

#### example

> **_NOTE:_** Default separator is "**␟**" ASCII 31 (0x1F) Unit Separator

```sh
node_id␟latitude␟longitude #represents a node
node_id␟latitude␟longitude
node_id␟node_id #represents a link
node_id␟node_id
```

#### real life data

```sh
3758221295␟48.8275185␟2.3484976 #represents a node
3761637488␟48.8275416␟2.3486683
3761637488␟3758221295 #represents a link
```
