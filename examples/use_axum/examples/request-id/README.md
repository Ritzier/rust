# request-id

If header `x-request-id` not provided, would generate a unique id for each
connection

```sh
2024-11-25T19:44:40.550283Z DEBUG http_request{request_id="2727bc2e-8e32-43d9-9da8-6200d15f0a16"}: tower_http::trace::on_request: started processing request
2024-11-25T19:44:40.550363Z  INFO http_request{request_id="2727bc2e-8e32-43d9-9da8-6200d15f0a16"}: request_id: Hello world!
2024-11-25T19:44:40.550401Z DEBUG http_request{request_id="2727bc2e-8e32-43d9-9da8-6200d15f0a16"}: tower_http::trace::on_response: finished processing request l atency=0 ms status=200

2024-11-25T19:44:41.530773Z DEBUG http_request{request_id="43d48907-24d6-4819-8882-8779443781ae"}: tower_http::trace::on_request: started processing request
2024-11-25T19:44:41.530851Z  INFO http_request{request_id="43d48907-24d6-4819-8882-8779443781ae"}: request_id: Hello world!
2024-11-25T19:44:41.530894Z DEBUG http_request{request_id="43d48907-24d6-4819-8882-8779443781ae"}: tower_http::trace::on_response: finished processing request latency=0 ms status=200
```

## Custom Request ID

```sh
curl -H "x-request-id: test-123" http://localhost:3000
```

```sh
2024-11-25T20:00:43.338342Z DEBUG http_request{request_id="test-323 1"}: tower_http::trace::on_request: started processing request
2024-11-25T20:00:43.338418Z  INFO http_request{request_id="test-323 1"}: request_id: Hello world!
2024-11-25T20:00:43.338455Z DEBUG http_request{request_id="test-323 1"}: tower_http::trace::on_response: finished processing request la
tency=0 ms status=200
```
