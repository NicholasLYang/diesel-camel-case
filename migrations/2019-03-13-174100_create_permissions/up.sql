CREATE TYPE action_type AS ENUM ('create', 'read', 'update', 'delete');
CREATE TYPE action_modifier AS ENUM ('all', 'self');

CREATE TABLE permissions (
id SERIAL PRIMARY KEY,
resource_name VARCHAR,
action action_type,
modifier action_modifier
);
-- Your SQL goes here
