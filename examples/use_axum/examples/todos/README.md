# Todos

This example demonstrates basic CRUD (Create, Read, Update, Delete) operatoins
for a Todo list application using Axum

## Usage

1. **Create a new todo**:

```sh
curl -X POST http://localhost:3000/todos \
    -H "Content-Type: application/json" \
    -d '{"text": "Buy groceries"}'
```

This will return the created Todo with a new UUID

```sh
{"id":"28ea8dc1-8a5e-48d9-be3d-69e0f2fe22d8","text":"Buy groceries","completed":false}
```

2. **List all todos**:

```sh
curl http://localhost:3000/todos
```

List todos with pagination:

```sh
curl "http://localhost:3000/todos?offset=0&limit=10"
```

3. **Update a Todo**:

```sh
curl -X PATCH http://localhost:3000/todos/<UUID> \
    -H "Content-Type: application/json" \
    -d '{"completed": true}'
```

4. **Delete a Todo**:

```sh
curl -X DELETE http://localhost:3000/todos/<UUID>
```
