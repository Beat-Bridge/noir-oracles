# Noir Oracles JSON-RPC Server for Spotify API Oracle

A JSON-RPC server that provides an API to interact with the Spotify API.

## Overview

This project sets up a JSON-RPC server using the `jsonrpc_http_server` library in Rust. The server listens on port 3000 and provides a single method named "get_top_tracks" that can be called remotely using JSON-RPC.

## Features

* Sets up a JSON-RPC server on port 3000
* Provides a single API method "get_top_tracks" that returns a hardcoded string
* Currently not using any parameters or making external API calls


## Running the Server

To run the server, simply execute the `create_server` function in the `main.rs` file. This will start the server and make it listen on port 3000.

## API Documentation

## Contributing

Contributions are welcome! If you'd like to contribute to this project, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License.