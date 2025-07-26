# KenchiDB

<p align="center">
  <img alt="pic" src="https://imgs.xkcd.com/comics/standards_2x.png" />
</p>

## Description

Strongly typed embedded document database.

Must implement:
- Paged storage engine;
- Copy-on-write B-tree indexes with atomic transactions;
- Simplified BSON format to communicate with client libraries;
- Single writer multiple readers;