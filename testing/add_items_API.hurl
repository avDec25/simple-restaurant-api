POST http://localhost:8080/table/add_items
{
    "table_number": 8,
    "items_names": [
        "拉麺",
        "うどん",
        "そば",
        "そうめん",
        "中華そば"
    ]
}
HTTP 200
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$.status" == "success"
jsonpath "$.message" == "Added 5 item(s) to table number 8"
jsonpath "$.items_ids" count == 5
