# HTTP method

## GET Request

Retrieves data from a server without modifying any resources. It's considered
safe and idempotent, meaning multiple identical requests will produce the same
result

```sh
curl https://api.example.com/resource
```

## POST Request

Submits data to create new resources on the server. It's not idempotent and
can modify server state

```sh
curl -X POST https://api.example.com/resource \
    -H 'Content-Type: Application/json' \
    -d '{"name": "example", "value": "data"}'
```

## PUT Request

Updates existing resources by replacing them entirely. It's idempotent,
requiring a complete resource representation

```sh
curl -X PUT https://api.example.com/resource \
    -H 'Content-Type: Application/json' \
    -d '{"name": "updated_example"}'
```

## DELETE Reqeust

Removes resources from the server permanently

```sh
curl -X DELETE https://api.examle.com/resource/:id
```

## PATCH Request

Performs partial updates to existing resources, modifying only specified field

```sh
curl -X PATCH https://api.example.com/resource/:id \
    -H 'Content-Type: application/json' \
    -d '{"field": "new_value"}'
```
