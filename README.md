# The Cocktail Catalogue Backend
## Prerequisites
### SQLite
Install SQLite. Guide TBA.

### Database (OLD: DELETE)
First, install PostgreSQL:
```bash
$ sudo apt update
$ sudo apt install postgresql postgresql-contrib
```
Then, create a new role:
```bash
$ sudo -u postgres createuser --interactive
Enter name of role to add: cocktailsdb
Shall the new role be a superuser? (y/n) y
```
Choose whatever name you like, but do make the role a superuser. Now, a new
database must be made:
```bash
$ sudo -u postgres createdb cocktailsdb
```
Create a new non-root user with the same name as the role:
```bash
$ sudo adduser cocktailsdb
```
Through this user, enter the Postgres prompt:
```bash
$ sudo -u cocktailsdb psql
```
Typing the `\conninfo` command should return information about the database
connection, e.g. the port:
```
cocktailsdb=# \conninfo
You are connected to database "cocktailsdb" as user "cocktailsdb" via socket in "/var/run/postgresql" at port "5432".
```

## TODO
- [ ] Add regular automatic backups of the database (for example using `.backup ?DB? FILE` from SQLite)
- [ ] Utilize serde deserialization from [serde_rusqlite](https://github.com/twistedfall/serde_rusqlite).