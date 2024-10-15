GET http://localhost:8080/health_check
HTTP 200
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
body contains "Health OK"
