# Scylla

#### Start Scylla Server

~ sudo systemctl start scylla-server

#### Create table users

CREATE TABLE users(id UUID PRIMARY KEY, fname text, lname text, email text, password text, created_at timestamp, updated_at timestamp);
CREATE TABLE documents(id UUID PRIMARY KEY, title text, body text, image text, created_at timestamp toTimeStamp(now()));
