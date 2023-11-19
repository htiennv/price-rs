# price-rs
This is a aggregate price service from multi source provider written in Rust

A service contains:
* A http server handles incomming request from client for get prices
* A worker run loop aggregate prices from multi providers and update price into in-memory cache
