install:
	cd sneaker_webserver && poetry install
	
test:
	cd sneaker_webserver && poetry run pytest

run_server: 
	
	cd sneaker_webserver && poetry run ./manage.py migrate
	cd sneaker_webserver && poetry run ./manage.py makemigrations
	cd sneaker_webserver && poetry run ./manage.py runserver

run_website:
	cd sneaker_website && npm install 
	cd sneaker_website && npm start

add_shoe:
	cargo run add

show_database:
	cargo run show
	
run_commandline:
	cargo run 
	
