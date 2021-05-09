# Scylla

#### Start Scylla Server

~ sudo systemctl start scylla-server

#### Create table users

CREATE TABLE users(id UUID PRIMARY KEY, fname text, lname text, email text, password text);
