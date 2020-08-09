INSERT INTO Context (id, name) VALUES (1, 'default');
INSERT INTO Role (id, name) VALUES (1, 'ADMIN');
INSERT INTO Role (id, name) VALUES (2, 'USER');
INSERT INTO Role (id, name) VALUES (3, 'ANONYMOUS');
INSERT INTO UserAccount (id, context_id, name, password) VALUES (1, 1, 'allison', 'changeme');

CREATE TABLE UserRoles (
  user_id integer NOT NULL,
  role_id integer NOT NULL
);

INSERT INTO UserRoles (user_id, role_id) VALUES(1, 1);
