POST http://localhost:8080/tables/1/items
{
    "items_names": [
        "拉麺",
        "うどん",
        "そば",
        "そうめん",
        "中華そば",
        "とんかつ",
        "からあげ",
        "やきにく",
        "カレーライス",
        "オムライス"
    ]
}
HTTP 201
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$.status" == "success"
jsonpath "$.message" == "Added 10 item(s) to table number 1"
jsonpath "$.items_ids" count == 10
[Captures]
item_id_1: jsonpath "$.items_ids[0]"
item_id_2: jsonpath "$.items_ids[1]"
item_id_3: jsonpath "$.items_ids[2]"
item_id_4: jsonpath "$.items_ids[3]"
item_id_5: jsonpath "$.items_ids[4]"
item_id_6: jsonpath "$.items_ids[5]"
item_id_7: jsonpath "$.items_ids[6]"
item_id_8: jsonpath "$.items_ids[7]"
item_id_9: jsonpath "$.items_ids[8]"
item_id_10: jsonpath "$.items_ids[9]"


GET http://localhost:8080/tables/1
HTTP 200
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$.items" count >= 10
jsonpath "$.table_number" == 1


GET http://localhost:8080/tables/1/items
HTTP 200
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$" count >= 10


GET http://localhost:8080/tables/1/items/{{item_id_1}}
HTTP 200
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$.item_id" == {{item_id_1}}
jsonpath "$.table_number" == 1
