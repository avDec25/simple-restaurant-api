POST http://localhost:8080/tables/108/items
{
    "items_names": [
        "Hamburgers",
        "Hot Dogs",
        "Buffalo Wings",
        "Apple Pie",
        "Barbecue Ribs",
        "Macaroni and Cheese",
        "Fried Chicken",
        "Clam Chowder",
        "Cornbread",
        "Cheesecake",
        "Meatloaf",
        "Chicken Pot Pie",
        "Biscuits and Gravy",
        "Pancakes",
        "Cobb Salad",
        "Tater Tots",
        "Philly Cheese steak",
        "Reuben Sandwich",
        "Jambalaya",
        "French Fries"
    ]
}
HTTP 201
[Asserts]
header "x-request-id" matches "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
jsonpath "$.status" == "success"
jsonpath "$.message" == "Added 20 item(s) to table number 108"
jsonpath "$.items_ids" count == 20
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
item_id_11: jsonpath "$.items_ids[10]"
item_id_12: jsonpath "$.items_ids[11]"
item_id_13: jsonpath "$.items_ids[12]"
item_id_14: jsonpath "$.items_ids[13]"
item_id_15: jsonpath "$.items_ids[14]"
item_id_16: jsonpath "$.items_ids[15]"
item_id_17: jsonpath "$.items_ids[16]"
item_id_18: jsonpath "$.items_ids[17]"
item_id_19: jsonpath "$.items_ids[18]"
item_id_20: jsonpath "$.items_ids[19]"


DELETE http://localhost:8080/tables/108/items/{{item_id_1}}
{"item_id": {{item_id_1}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_1}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_2}}
{"item_id": {{item_id_2}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_2}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_3}}
{"item_id": {{item_id_3}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_3}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_4}}
{"item_id": {{item_id_4}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_4}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_5}}
{"item_id": {{item_id_5}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_5}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_6}}
{"item_id": {{item_id_6}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_6}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_7}}
{"item_id": {{item_id_7}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_7}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_8}}
{"item_id": {{item_id_8}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_8}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_9}}
{"item_id": {{item_id_9}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_9}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_10}}
{"item_id": {{item_id_10}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_10}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_11}}
{"item_id": {{item_id_11}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_11}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_12}}
{"item_id": {{item_id_12}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_12}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_13}}
{"item_id": {{item_id_13}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_13}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_14}}
{"item_id": {{item_id_14}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_14}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_15}}
{"item_id": {{item_id_15}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_15}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_16}}
{"item_id": {{item_id_16}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_16}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_17}}
{"item_id": {{item_id_17}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_17}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_18}}
{"item_id": {{item_id_18}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_18}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_19}}
{"item_id": {{item_id_19}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_19}}"

DELETE http://localhost:8080/tables/108/items/{{item_id_20}}
{"item_id": {{item_id_20}} }
HTTP 200
[Asserts]
jsonpath "$.status" == "success"
jsonpath "$.message" == "Removed table item with item_id {{item_id_20}}"

