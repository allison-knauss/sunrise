# AAA Server

Do not expose to the open internet!

## Endpoints

- `GET /login/<username>/<password>` - Get a valid auth token if this user is valid. HTTP 200 with body token if successful, HTTP 404 on failure.
- `GET /validate/<token>` - Validate the token, returning HTTP 200 with body Claims if valid, HTTP 404 if invalid.
