POST http://localhost:8080/table/add_items
{
    "table_number": 1,
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
HTTP 200
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


GET http://localhost:8080/table/list_items
{
    "table_number": 1
}
HTTP 200
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$.status" == "success"
jsonpath "$.message" == "Found 10 table item(s)"
jsonpath "$.table_items" count == 10
jsonpath "$.table_items[0].item_id" == {{item_id_1}}
jsonpath "$.table_items[1].item_id" == {{item_id_2}}
jsonpath "$.table_items[2].item_id" == {{item_id_3}}
jsonpath "$.table_items[3].item_id" == {{item_id_4}}
jsonpath "$.table_items[4].item_id" == {{item_id_5}}
jsonpath "$.table_items[5].item_id" == {{item_id_6}}
jsonpath "$.table_items[6].item_id" == {{item_id_7}}
jsonpath "$.table_items[7].item_id" == {{item_id_8}}
jsonpath "$.table_items[8].item_id" == {{item_id_9}}
jsonpath "$.table_items[9].item_id" == {{item_id_10}}


GET http://localhost:8080/table/list_items
{
    "table_number": 1,
    "items_ids": [8, 9]
}
HTTP 200
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$.status" == "success"
jsonpath "$.message" == "Found 2 table item(s)"
jsonpath "$.table_items" count == 2
jsonpath "$.table_items[0].item_id" == {{item_id_8}}
jsonpath "$.table_items[1].item_id" == {{item_id_9}}


GET http://localhost:8080/table/list_items
{
    "table_number": 1,
    "items_names": ["うどん"]
}
HTTP 200
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$.status" == "success"
jsonpath "$.message" == "Found 1 table item(s)"
jsonpath "$.table_items" count == 1
jsonpath "$.table_items[0].item_id" == {{item_id_2}}
jsonpath "$.table_items[0].item_name" == "うどん"


GET http://localhost:8080/table/list_items
{
    "table_number": 1,
    "items_ids": [8, 9],
    "items_names": ["うどん"]
}
HTTP 200
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$.status" == "success"
jsonpath "$.message" == "Found 3 table item(s)"
jsonpath "$.table_items" count == 3
jsonpath "$.table_items[0].item_id" == {{item_id_2}}
jsonpath "$.table_items[1].item_id" == {{item_id_8}}
jsonpath "$.table_items[2].item_id" == {{item_id_9}}


