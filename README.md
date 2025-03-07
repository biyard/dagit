# d.AGIT

## Common Environments

| Name     | Description                                                                    |
| -------- | ------------------------------------------------------------------------------ |
| RUST_LOG | Logging libraries based on tracing(`debug`, `info`, `error`)                   |
| SERVICE  | Package name to be executed. default is `main-ui`                              |
| DOMAIN   | Base domain name (ex. dev.example.com) will be used to compose signing message |
| ENV      | Development environment(`local`, `dev`, `prod`)                                |

## Development

### Running API Server(api)

- It runs `SERVICE` crate.

### Database Setup

Before running the API server, you need to create the following trigger functions in your PostgreSQL database:

```sql
  CREATE OR REPLACE FUNCTION set_updated_at()
    RETURNS TRIGGER AS $$
  BEGIN
    NEW.updated_at := EXTRACT(EPOCH FROM now());
    RETURN NEW;
  END;
  $$ LANGUAGE plpgsql;

  CREATE OR REPLACE FUNCTION set_created_at()
    RETURNS TRIGGER AS $$
  BEGIN
    NEW.created_at := EXTRACT(EPOCH FROM now());
    RETURN NEW;
  END;
  $$ LANGUAGE plpgsql;
```

```bash
export SERVICE=api
export DATABASE_TYPE=postgres
export DATABASE_URL=postgres://localhost:5432/dagit
make run
```

### Running Web UI(main-ui)

- It will interact with API server in `dev` environment.
  - If you want to change it, set `API_URL` environment.

```bash
export SERVICE=main-ui
export API_URL=http://localhost:3000
make run
```

## Deployment

### Main UI

```bash
export STACK=dagit-main-ui-dev-stack
export SERVICE=main-ui
export ENV=dev
export BASE_DOMAIN=dagit.club
export DOMAIN=dev.dagit.club

export ENABLE_S3=true
export ENABLE_LAMBDA=true
make deploy
```

### Build UI

```bash
export STACK=dagit-build-ui-dev-stack
export SERVICE=build-ui
export ENV=dev
export BASE_DOMAIN=dagit.club
export DOMAIN=build.dev.dagit.club

export ENABLE_S3=true
export ENABLE_LAMBDA=true
make deploy
```

### Main API

```bash
export ENV=dev
export STACK=dagit-api-dev-stack
export SERVICE=api
export ENABLE_RDS=true
export ENABLE_LAMBDA=true
export BASE_DOMAIN=dagit.club
export DOMAIN=api.dev.dagit.club
export DATABASE_URL=""
export DATABASE_TYPE=postgres
make deploy
```
