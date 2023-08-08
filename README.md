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



##Django API + Frontend

Django:
`cd sneaker_webserver`
`python3 manage.py migrate` (only first time to create the DB)
`python3 manage.py runserver`

React in other Terminal:

`cd sneaker_webserver/sneaker_website`
`npm start`

Then add any shoe you want via the Command Line Interface.<>