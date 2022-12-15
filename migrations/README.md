# Migration scripts

## Installation
````bash
cargo install sqlx-cli
````

## Create database

````bash
sqlx database setup
````

## Use migration
````bash
# Creating new script file.
sqlx migrate add <Name for script>

# Run migration
sqlx migrate run
````
