[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)[![codecov](https://codecov.io/gh/ethicnology/ophois/branch/main/graph/badge.svg?token=YGE7F4GUCK)](https://codecov.io/gh/ethicnology/ophois)[![coverage](https://github.com/ethicnology/ophois/actions/workflows/coverage.yml/badge.svg)](https://github.com/ethicnology/ophois/actions/workflows/coverage.yml)

# Ophoïs, creates street graph from OpenStreetMap

## installation

#### pre-built

Download the
[**lastest linux release**](https://github.com/ethicnology/ophois/releases)

If you trust/verified this code, make it executable and add it to your path
```sh
sudo chmod +x ophois
sudo mv ophois /usr/local/bin
```

#### or build from sources

```sh
cargo build --release # >= Rust 1.58
# output should be in /target/release/ophois
```

## :one: download a map

```sh
CITY=Pantin # Save your city in an environment variable
ophois download --city $CITY
```

## :two: extract

```sh
cat $CITY.osm | ophois format | ophois extract > $CITY-extracted.graph
```

**same command with space separator**
> **_NOTE:_** Default separator is "**␟**" ASCII 31 (0x1F) Unit Separator but you can use any suitable separator as long you specify it with **--separator**

```sh
cat $CITY.osm | ophois format | ophois extract --separator ' ' > $CITY-extracted.graph
```

## :three: simplify
The tool used to generate the following screenshots is [cartographe](https://ethicnology.github.io/cartographe/)  
**keep the largest component, remove degree two nodes, replace nodes with under delta links by links and replace links (and nodes) which distance is under delta by a midpoint node connected to neighbours**

```sh
cat $CITY-extracted.graph | ophois simplify --delta 10.0 > $CITY-simplified.graph
```

### extracted input

![](https://github.com/ethicnology/osmtograph/blob/main/datasets/cailles.png)

### remove degree two nodes

![](https://github.com/ethicnology/osmtograph/blob/main/datasets/test_remove_degree_two_nodes_after.png)

### replace nodes that only have distance links less than delta with links between their neighbours

![](https://github.com/ethicnology/osmtograph/blob/main/datasets/test_remove_under_delta_nodes_after_delta=6.png)

> **_NOTE:_** delta=6

### replace links (including nodes) which are under delta distance by a midpoint node

![](https://github.com/ethicnology/osmtograph/blob/main/datasets/test_remove_under_delta_links_after_delta=6.png)

> **_NOTE:_** delta=6

## :four: discretize

### split links that have distance over 2*delta in equal parts

```sh
cat $CITY-simplified.graph | ophois discretize --delta 6.0 > $CITY-discretized.graph
```

![](https://github.com/ethicnology/osmtograph/blob/main/datasets/test_discretize_after_delta=6.png)

> **_NOTE:_** delta=6

## one line simplify and discretize

```sh
ophois download --city $CITY; cat $CITY.osm | ophois format | ophois extract | ophois simplify --delta 10 | ophois discretize --delta 5 > $CITY.graph
```

**same command with space separator**

```sh
ophois download --city $CITY; cat $CITY.osm | ophois format | ophois extract -s ' ' | ophois simplify -s ' ' -d 10 | ophois discretize -s ' ' -d 5 > $CITY.graph
```

## graph format

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

## Authors
* [ethicnology](https://github.com/ethicnology)
* [Matthieu Latapy](https://www-complexnetworks.lip6.fr/~latapy/)