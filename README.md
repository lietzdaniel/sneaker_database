#Sneaker Database Command Line Interface

Run with `cargo run` in the command line. 

## Description of Commands

### Search

With the Search command, you can search for a shoe on the StockX Website to add to your database. You will have to choose a shoe from the search results and add the size
 you want to add the shoe in.

### Custom Entry

Currently unimplemented.

### Remove a Shoe

Currently unimplemented.

### Show your database

Shows your shoe database with Style ID, Name, Retail Price and Size of your shoes added in a nice ASCII Database. 
Future features: Edit Rows that you want to show

### Get Fun Facts

Currently unimplemented

## Quit the database manager

Quits the program



## Django API + Frontend


Use the Makefile in order to set everything up.

- `make install` to install the requirements for the Django Server
- `make run_server` to run the Django Server
- `make run_website` in other Terminal to run the React Frontend
- `make add_shoe` to add a shoe to your database
- `make run_commandline` to run the command line text adventure
- `make show_database` to show your current database